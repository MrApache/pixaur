use bevy_ecs::{prelude::*, schedule::ScheduleLabel, system::ScheduleSystem};
use std::{
    any::{type_name, TypeId},
    collections::HashSet,
};

use crate::{ecs_rendering::Renderer, widget::Widget};

#[derive(Default)]
pub struct App {
    registered_widgets: HashSet<TypeId>,

    world: World,
    schedules: Schedules,
}

impl App {
    pub fn new() -> Self {
        let update_schedule = Schedule::new(Update);
        let update_transforms_schedule = Schedule::new(UpdateTransforms);
        let collect_draw_commands = Schedule::new(CollectDrawCommands);
        let render = Schedule::new(Render);

        let mut schedules = Schedules::new();
        schedules.insert(update_schedule);
        schedules.insert(update_transforms_schedule);
        schedules.insert(collect_draw_commands);
        schedules.insert(render);

        let mut app = Self {
            registered_widgets: Default::default(),
            world: World::new(),
            schedules,
        };

        app.add_widget(Renderer);

        app
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.schedules.add_systems(schedule, systems);
        self
    }

    pub fn add_widget<T: Widget>(&mut self, widget: T) -> &mut Self {
        let widget_type = TypeId::of::<T>();
        if self.registered_widgets.contains(&widget_type) {
            let type_name = type_name::<T>();
            panic!("Widget '{type_name}' already registered");
        }

        widget.init(self);
        self.registered_widgets.insert(TypeId::of::<T>());
        self
    }

    pub fn insert_resource<R: Resource>(&mut self, value: R) -> &mut Self {
        self.world.insert_resource(value);
        self
    }

    pub fn run(&mut self) {
        loop {
            self.world.run_schedule(UpdateTransforms);
            self.world.run_schedule(Update);
            self.world.run_schedule(CollectDrawCommands);
            self.world.run_schedule(Render);
        }
    }
}

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Update;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct UpdateTransforms;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct CollectDrawCommands;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct Render;
