use std::any::Any;
use toolkit::{widget::{Container, Rect, Size, Spacing, Widget}, window::{Window, WindowRequest}, Anchor, Argb8888, Color, CommandBuffer, Context, DesktopOptions, EventLoop, LinearGradient, UserWindow, GUI};
use widgets::{panel::Panel, Text};

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
        WindowRequest::new("desktop")
            .desktop(DesktopOptions {
                title: "Test application".into(),
                resizable: true,
                decorations: false,
            })
    }

    fn setup(&self, _gui: &mut App) -> Box<dyn Container> {
        let mut panel = Panel::new("Panel");
        panel.background = Color::LinearGradient(LinearGradient::new(Argb8888::RED, Argb8888::YELLOW));

        Box::new(panel)
    }

    fn update<'ctx>(&mut self, _gui: &mut App, context: &'ctx mut Context<'ctx>) {
    }
}


fn main() {
    let mut event_loop = EventLoop::new(App).unwrap();
    event_loop.run().unwrap();
}

