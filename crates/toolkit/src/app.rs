use bevy_ecs::{prelude::*, schedule::ScheduleLabel, system::ScheduleSystem};
use glam::Vec2;
use std::{
    any::{type_name, TypeId},
    collections::{HashMap, HashSet},
    ffi::c_void,
    ptr::NonNull,
    sync::{atomic::AtomicU16, Arc},
};
use wayland_client::{Connection, EventQueue, Proxy};
use wgpu::{Instance, InstanceDescriptor};
use wl_client::{window::WindowLayer, WlClient};

use crate::{
    debug::FpsCounter,
    rendering::{Gpu, Renderer, RendererPlugin},
    widget::Plugin,
    window::{Window, WindowPointer},
    Client, ContentPlugin, Error, Transform, UserWindow, WindowId, Windows,
};

pub struct App {
    unique_plugins: HashSet<TypeId>,

    world: World,

    event_queue: EventQueue<WlClient>,
}

impl App {
    fn init_client() -> Result<(EventQueue<WlClient>, Client, Gpu, Renderer), Error> {
        let conn = Connection::connect_to_env()?;

        let display = conn.display();
        let mut event_queue = conn.new_event_queue();
        let qh = event_queue.handle();

        let _registry = display.get_registry(&qh, Arc::new("".to_string()));

        let mut client = WlClient::default();

        event_queue.roundtrip(&mut client)?; //Register objects
        event_queue.roundtrip(&mut client)?; //Register outputs

        //Fix egl error: BadDisplay
        let (display_ptr, gpu, renderer) = {
            let display_ptr = NonNull::new(display.id().as_ptr() as *mut c_void).unwrap();
            let dummy = client.create_window_backend(qh, "dummy", 1, 1, WindowLayer::default());
            event_queue.roundtrip(&mut client)?; //Init dummy

            let dummy_ptr = dummy.lock().unwrap().as_ptr();
            let ptr = WindowPointer::new(display_ptr, dummy_ptr);

            let instance = Instance::new(&InstanceDescriptor::default());
            let surface = instance.create_surface(ptr)?;

            let gpu = Gpu::new(instance, &surface)?;
            let renderer = Renderer::new(&gpu, None, &surface)?;

            drop(dummy);

            client.destroy_window_backend("dummy");
            event_queue.roundtrip(&mut client)?; //Destroy dummy

            (display_ptr, gpu, renderer)
        };

        let client = Client {
            inner: client,
            display_ptr,
        };

        Ok((event_queue, client, gpu, renderer))
    }

    pub fn new() -> Result<Self, Error> {
        let startup = Schedule::new(Startup);
        let first = Schedule::new(First);
        let update_schedule = Schedule::new(Update);
        let update_transforms_schedule = Schedule::new(UpdateRoot);
        let collect_draw_commands = Schedule::new(CollectDrawCommands);
        let render = Schedule::new(Render);

        let mut schedules = Schedules::new();
        schedules.insert(startup);
        schedules.insert(first);
        schedules.insert(update_schedule);
        schedules.insert(update_transforms_schedule);
        schedules.insert(collect_draw_commands);
        schedules.insert(render);

        let (event_queue, client, gpu, renderer) = Self::init_client()?;

        let mut app = Self {
            unique_plugins: Default::default(),
            world: World::new(),

            event_queue,
        };

        app.insert_resource(schedules);
        app.insert_resource(renderer);
        app.insert_resource(Windows {
            handle: app.event_queue.handle(),
            active: HashMap::new(),
            not_initalized: vec![],
        });
        app.insert_resource(client);
        app.insert_resource(gpu);
        app.add_plugin(RendererPlugin);
        app.add_plugin(ContentPlugin);

        app.add_systems(Startup, init_windows);
        app.add_systems(First, (update_windows, mark_roots, remove_roots));
        app.add_systems(UpdateRoot, set_window_size_at_root);

        Ok(app)
    }

    fn init_schedule(&mut self, label: impl ScheduleLabel) -> &mut Self {
        let label = label.intern();
        let mut schedules = self.world.resource_mut::<Schedules>();
        if !schedules.contains(label) {
            schedules.insert(Schedule::new(label));
        }
        self
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        let mut schedules = self.world.resource_mut::<Schedules>();
        schedules.add_systems(schedule, systems);
        self
    }

    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        let plugin_type = TypeId::of::<T>();
        if self.unique_plugins.contains(&plugin_type) {
            let type_name = type_name::<T>();
            panic!("Plugin '{type_name}' already registered");
        }

        plugin.init(self);
        self.unique_plugins.insert(TypeId::of::<T>());
        self
    }

    pub fn insert_resource<R: Resource>(&mut self, value: R) -> &mut Self {
        self.world.insert_resource(value);
        self
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let mut fps = FpsCounter::new(144);
        self.world.run_schedule(Startup);
        loop {
            //tracy_client::Client::start();
            self.world.run_schedule(First);
            self.world.run_schedule(UpdateRoot);
            self.world.run_schedule(Update);
            self.world.run_schedule(CollectDrawCommands);
            self.world.run_schedule(Render);
            let mut client = self.world.resource_mut::<Client>();
            self.event_queue.blocking_dispatch(&mut client.inner)?;
            //self.event_queue.dispatch_pending(&mut client.inner)?;
            let tick = fps.tick();
            println!("FPS: {tick}");
        }
    }

    pub fn add_window(&mut self, window: impl UserWindow) -> &mut Self {
        let mut windows = self.world.resource_mut::<Windows>();
        windows.not_initalized.push(Box::new(window));
        self
    }
}

static WINDOW_NEXT_ID: AtomicU16 = AtomicU16::new(0);

fn init_windows(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut client: ResMut<Client>,
    gpu: Res<Gpu>,
) {
    let mut active = HashMap::with_capacity(windows.not_initalized.len());

    let qh = windows.handle.clone();
    windows.not_initalized.drain(..).for_each(|handle| {
        let request = handle.request();
        let backend = client.inner.create_window_backend(
            qh.clone(),
            request.id,
            request.width,
            request.height,
            request.layer,
        );

        let (width, height, surface_ptr) = {
            let guard = backend.lock().unwrap();
            (guard.width as u32, guard.height as u32, guard.as_ptr())
        };

        let window_ptr = WindowPointer::new(client.display_ptr, surface_ptr);
        let (surface, configuration) = gpu.create_surface(window_ptr, width, height).unwrap();
        let id = WindowId(WINDOW_NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
        handle.setup(&mut commands, id.clone());
        let window = Window::new(backend, surface, configuration, handle);

        active.insert(id, window);
    });

    windows.active.extend(active);
}

fn update_windows(mut windows: ResMut<Windows>, gpu: Res<Gpu>) {
    windows.active.values_mut().for_each(|window| {
        let mut backend = window.backend.lock().unwrap();
        if backend.can_resize() {
            window.configuration.width = backend.width as u32;
            window.configuration.height = backend.height as u32;
            gpu.confugure_surface(&window.surface, &window.configuration);
            backend.set_resized();
        }

        backend.frame();
        window.can_draw = backend.can_draw();
    });
}

fn mark_roots(mut commands: Commands, query: Query<Entity, (Without<Children>, Without<Root>)>) {
    query.iter().for_each(|entity| {
        commands.entity(entity).insert(Root);
    });
}

fn remove_roots(mut commands: Commands, query: Query<Entity, (With<Children>, With<Root>)>) {
    query.iter().for_each(|entity| {
        commands.entity(entity).remove::<Root>();
    });
}

fn set_window_size_at_root(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &WindowId), With<Root>>,
) {
    query.iter_mut().for_each(|(mut transform, window_id)| {
        let window = windows.active.get(window_id).unwrap();
        transform.position = Vec2::ZERO;
        transform.size = Vec2::new(
            window.configuration.width as f32,
            window.configuration.height as f32,
        )
    });
}

#[derive(Component)]
pub(crate) struct Root;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Startup;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct First;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct UpdateRoot;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Update;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct CollectDrawCommands;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct Render;
