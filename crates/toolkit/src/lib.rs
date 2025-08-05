mod error;
mod renderer;
mod content;
mod color;
pub mod widget;
pub mod window;

pub use color::*;
pub use error::*;
use wgpu::{naga::back, Surface};
pub use wl_client::{Anchor, window::{DesktopOptions, SpecialOptions}};

use crate::{
    content::Content,
    renderer::GPU,
    widget::{Container, Rect, Widget},
    window::{Window, WindowPointer, WindowRequest}
};

use wl_client::{window::WindowLayer, WlClient};
use ab_glyph::{point, Font};
use std::{
    ffi::c_void,
    ptr::NonNull,
    sync::Arc
};

use wayland_client::{Connection, EventQueue, Proxy};

pub const DEFAULT_FONT: &str = "Ubuntu Regular";

#[derive(Debug, Clone)]
pub enum DrawCommand<'a> {
    Rect {
        rect: Rect,
        color: Color,
    },
    Text {
        size: f32,
        font: &'a str,
        content: &'a str,
        color: Color,
    },
}

#[derive(Default)]
pub struct CommandBuffer<'frame> {
    pub storage: Vec<DrawCommand<'frame>>,
}

impl<'frame> CommandBuffer<'frame> {
    pub fn push(&mut self, cmd: DrawCommand<'frame>) {
        self.storage.push(cmd);
    }

    pub fn extend<I: IntoIterator<Item = DrawCommand<'frame>>>(&mut self, iter: I) {
        iter.into_iter().for_each(|cmd| self.push(cmd));
    }

    pub(crate) fn pop(&mut self) -> Option<DrawCommand<'frame>> {
        self.storage.pop()
    }
}

#[allow(unused)]
pub trait GUI {
    fn load_content(&self, content: &mut Content) {}
    fn setup_windows(&mut self) -> Vec<Box<dyn UserWindow<Self>>>;
}

pub struct EventLoop<T: GUI> {
    gui: T,
    content: Content,

    client: WlClient,
    event_queue: EventQueue<WlClient>,
    display_ptr: NonNull<c_void>,

    gpu: GPU,
}

impl<T: GUI> EventLoop<T> {
    pub fn new(app: T) -> Result<Self, Error> {

        let conn = Connection::connect_to_env()
            .expect("Failed to connect to the Wayland server.");

        let display = conn.display();
        let mut event_queue = conn.new_event_queue();
        let qh = event_queue.handle();

        let _registry = display.get_registry(&qh, Arc::new("".to_string()));

        let mut client = WlClient::default();

        event_queue.roundtrip(&mut client).unwrap(); //Register objects
        event_queue.roundtrip(&mut client).unwrap(); //Register outputs

        let mut content = Content::default();
        content.include_font(include_bytes!("../../../assets/Ubuntu-Regular.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-Light.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-LightItalic.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-Bold.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-BoldItalic.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-Italic.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-Medium.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-MediumItalic.ttf"));

        //Fix egl error: BadDisplay
        let (display_ptr, gpu) = {
            let display_ptr = NonNull::new(display.id().as_ptr() as *mut c_void).unwrap();
            let dummy = client.create_window_backend(qh, "dummy", 1, 1, WindowLayer::default());
            event_queue.roundtrip(&mut client).unwrap(); //Init dummy
            
            let dummy_ptr = dummy.lock().unwrap().as_ptr();
            let ptr = WindowPointer::new(display_ptr, dummy_ptr);
            let gpu = GPU::new(ptr)?;

            drop(dummy);

            client.destroy_window_backend("dummy");
            event_queue.roundtrip(&mut client).unwrap(); //Destroy dummy

            (display_ptr, gpu)
        };

        Ok(Self {
            gui: app,
            content,

            client,
            event_queue,
            display_ptr,

            gpu
        })
    }

    pub fn run(&mut self) -> Result<(), Error>{
        self.gui.load_content(&mut self.content);
        let mut windows = self.init_windows_backends()?;
        let mut counter = FpsCounter::new(144);

        loop {
            let fps = counter.tick();
            println!("FPS: {fps:.1}");

            windows.iter_mut().for_each(|window| {
                let mut cm_buffer = CommandBuffer::default();
                let mut context = Context {
                    root: &mut window.frontend
                };

                window.handle.update(&mut self.gui, &mut context);

                let mut window_handle = window.backend.lock().unwrap();
                window_handle.frame();
                if !window_handle.can_draw() {
                    return;
                }

                window_handle.clear();

                window.frontend.draw(&mut cm_buffer);
                while let Some(command) = cm_buffer.pop() {
                    #[allow(unused)]
                    match command {
                        DrawCommand::Rect { rect, color } => todo!(),
                        DrawCommand::Text { size, font, content, color } => {
                            let font = self.content.get_font(font);
                            
                            let units_per_em = font.units_per_em().unwrap();
                            let scale = size / units_per_em;
                            
                            let ascent = font.ascent_unscaled();
                            let base_y = ascent * scale;
                            
                            let mut pen_x = 0.0;
                            
                            for ch in content.chars() {
                                let glyph = font.glyph_id(ch).with_scale_and_position(size, point(pen_x, base_y));
                                let pos_x = glyph.position.x;
                                let pos_y = glyph.position.y;
                                let h_advance = font.h_advance_unscaled(glyph.id);
                                
                                if let Some(outlined) = font.outline_glyph(glyph) {
                                    outlined.draw(|x, y, c| {
                                        let draw_x = pos_x + x as f32;
                                        let draw_y = pos_y + y as f32;
                                        window_handle.draw_text_at(draw_x as usize, draw_y as usize, c);
                                        //self.client.draw_text(&window.id, draw_x as u32, draw_y as u32, c);
                                    });
                                }
                            
                                pen_x += h_advance as f32 * scale;
                            }
                        }
                    }
                }

                window_handle.commit();
                window_handle.draw();
            });


            self.event_queue.blocking_dispatch(&mut self.client).unwrap();
        }
    }

    fn init_windows_backends(&mut self) -> Result<Vec<Window<T>>, Error> {
        let user_windows = self.gui.setup_windows();
        let mut backends = Vec::with_capacity(user_windows.len());
        let qh = self.event_queue.handle();

        user_windows.into_iter().try_for_each(|handle| {
            let request = handle.request();
            let backend = self
                .client
                .create_window_backend(qh.clone(), request.id, request.width, request.height, request.layer);
        
            let (width, height, surface_ptr) = {
                let guard = backend.lock().unwrap();
                (guard.width as u32, guard.height as u32, guard.as_ptr())
            };
        
            let window_ptr = WindowPointer::new(self.display_ptr, surface_ptr);
            let surface = self.gpu.create_surface(window_ptr, width, height)?;
            let frontend = handle.setup(&mut self.gui);
            let window = Window::new(frontend, backend, surface, handle);
        
            backends.push(window);
        
            Ok::<(), Error>(())
        })?;


        Ok(backends)
    }
}

pub trait UserWindow<T: GUI> {
    fn request(&self) -> WindowRequest;
    fn setup(&self, gui: &mut T) -> Box<dyn Container>;
    fn update<'ctx>(&mut self, gui: &mut T, context: &'ctx mut Context<'ctx>);
}

pub struct Context<'a> {
    root: &'a mut Box<dyn Container>,
}

impl<'a> Context<'a> {
    fn internal_get_by_id<W: Widget>(container: &'a dyn Container, id: &str) -> Option<&'a W> {
        for w in container.children() {
            if w.id().eq(id) {
                return w.as_any().downcast_ref::<W>();
            }

            if let Some(container) = w.as_container() {
                return Self::internal_get_by_id(container, id);
            }
        }

        None
    }

    fn internal_get_mut_by_id<W: Widget>(container: &'a mut dyn Container, id: &str) -> Option<&'a mut W> {
        for w in container.children_mut() {
            if w.id().eq(id) {
                return w.as_any_mut().downcast_mut::<W>();
            }

            if let Some(container) = w.as_container_mut() {
                return Self::internal_get_mut_by_id(container, id);
            }
        }

        None
    }

    pub fn get_by_id<W: Widget>(&'a self, id: &str) -> Option<&'a W> {
        Self::internal_get_by_id(self.root.as_ref(), id)
    }

    pub fn get_mut_by_id<W: Widget>(&'a mut self, id: &str) -> Option<&'a mut W> {
        Self::internal_get_mut_by_id(self.root.as_mut(), id)
    }
}

use std::time::{Instant, Duration};

pub struct FpsCounter {
    last_frame_time: Instant,
    frame_times: Vec<Duration>,
    max_samples: usize,
}

impl FpsCounter {
    pub fn new(max_samples: usize) -> Self {
        Self {
            last_frame_time: Instant::now(),
            frame_times: Vec::with_capacity(max_samples),
            max_samples,
        }
    }

    pub fn tick(&mut self) -> f64 {
        let now = Instant::now();
        let delta = now - self.last_frame_time;
        self.last_frame_time = now;

        self.frame_times.push(delta);

        if self.frame_times.len() > self.max_samples {
            self.frame_times.remove(0);
        }

        let avg_duration: Duration = self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32;
        1.0 / avg_duration.as_secs_f64()
    }
}
