use std::{any::{type_name, TypeId}, collections::HashSet};

use bevy_ecs::{schedule::{IntoScheduleConfigs, Schedule, ScheduleLabel, Schedules}, system::ScheduleSystem, world::World};

use crate::widget::Widget;

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

        let mut schedules = Schedules::new();
        schedules.insert(update_schedule);
        schedules.insert(update_transforms_schedule);

        Self {
            registered_widgets: Default::default(),
            world: World::new(),
            schedules,
        }
    }

    pub fn add_systems<M>(&mut self, schedule: impl ScheduleLabel, systems: impl IntoScheduleConfigs<ScheduleSystem, M>) -> &mut Self {
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

    pub fn run(&mut self) {
        loop {
            self.world.run_schedule(UpdateTransforms);
            self.world.run_schedule(Update);
        }
    }
}

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Update;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct UpdateTransforms;
