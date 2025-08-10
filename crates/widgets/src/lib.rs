use toolkit::{
    Argb8888, Color, DEFAULT_FONT, DrawCommand,
    glam::Vec2,
    widget::{DesiredSize, Widget},
};

pub mod panel;

pub struct Text {
    id: String,

    pub value: String,

    font: String,
    pub size: f32,
}

impl Text {
    pub fn empty(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            value: String::new(),
            font: DEFAULT_FONT.to_string(),
            size: 12.0,
        }
    }

    pub fn new(value: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            font: DEFAULT_FONT.to_string(),
            size: 12.0,
        }
    }
}

impl Widget for Text {
    fn id(&self) -> &str {
        &self.id
    }

    fn desired_size(&self) -> DesiredSize {
        DesiredSize::Min(Vec2::default())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw<'frame>(&'frame self, out: &mut toolkit::CommandBuffer<'frame>) {
        out.push(DrawCommand::Text {
            content: &self.value,
            font: &self.font,
            size: self.size,
            color: Color::Simple(Argb8888::RED),
        });
    }

    fn layout(&mut self, bounds: toolkit::widget::Rect) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use toolkit::{
        Anchor, CommandBuffer, Context, DesktopOptions, EventLoop, GUI, UserWindow,
        widget::{Container, Rect},
        window::{Window, WindowRequest},
    };

    use crate::*;

    #[test]
    fn example() {
        let mut event_loop = EventLoop::new(App).unwrap();
        event_loop.run().unwrap();
    }

    #[derive(Default)]
    struct Panel {
        vec: Vec<Box<dyn Widget>>,
    }

    impl Container for Panel {
        fn add_child(&mut self, child: Box<dyn Widget>) {
            self.vec.push(child);
        }

        fn children(&self) -> &[Box<dyn Widget>] {
            &self.vec
        }

        fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
            &mut self.vec
        }
    }

    impl Widget for Panel {
        fn layout(&mut self, _bounds: Rect) {}

        fn id(&self) -> &str {
            ""
        }

        fn desired_size(&self) -> DesiredSize {
            DesiredSize::Min(Vec2::default())
        }

        fn as_container(&self) -> Option<&dyn Container> {
            None
        }

        fn as_container_mut(&mut self) -> Option<&mut dyn Container> {
            None
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }

        fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>) {
            for w in &self.vec {
                w.draw(out);
            }
        }
    }

    struct App;
    impl GUI for App {
        fn setup_windows(&mut self) -> Vec<Box<dyn UserWindow<App>>> {
            vec![Box::new(MainWindow::default())]
        }
    }

    #[derive(Default)]
    struct MainWindow {
        value: u64,
    }

    impl UserWindow<App> for MainWindow {
        fn request(&self) -> WindowRequest {
            WindowRequest::new("desktop").desktop(DesktopOptions {
                title: "Test application".into(),
                resizable: false,
                decorations: false,
            })
        }

        fn setup(&self, _gui: &mut App) -> Box<dyn Container> {
            let mut panel = Panel::default();
            let mut text = Text::new("Hello, world!", "label");
            text.size = 24.0;
            panel.add_child(Box::new(text));

            Box::new(panel)
        }

        fn update<'ctx>(&mut self, _gui: &mut App, context: &'ctx mut Context<'ctx>) {
            self.value += 1;
            let text = context.get_mut_by_id::<Text>("label").unwrap();
            text.value = format!("Count: {}", self.value);
        }
    }
}
