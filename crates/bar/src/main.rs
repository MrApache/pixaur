use toolkit::{
    types::{Argb8888, Color, Stroke},
    widget::Container,
    window::WindowRequest,
    Anchor, Error, EventLoop, SpecialOptions, TargetMonitor, UserWindow, GUI,
};
use widgets::{panel::Panel, text::Text};

#[derive(Default)]
struct App {}

impl GUI for App {
    fn setup_windows(&mut self) -> Vec<Box<dyn toolkit::UserWindow<Self>>> {
        vec![Box::new(BarWindow)]
    }
}

struct BarWindow;

impl UserWindow<App> for BarWindow {
    fn request(&self) -> toolkit::window::WindowRequest {
        WindowRequest::new("bar")
            .with_size(1920, 35)
            .bottom(SpecialOptions {
                anchor: Anchor::Top,
                exclusive_zone: 35,
                target: TargetMonitor::Primary,
            })
    }

    fn setup(&self, gui: &mut App) -> Box<dyn toolkit::widget::Container> {
        let mut root = Panel::new();
        root.background = Color::Simple(Argb8888::new(17, 17, 27, 255)).into();
        root.stroke = Stroke::none();

        let mut time = Text::default();
        time.value = "23:59".into();
        time.size = 16;
        root.add_child(Box::new(time));

        Box::new(root)
    }

    fn update<'ctx>(&mut self, gui: &mut App, context: &'ctx mut toolkit::Context<'ctx>) {}
}

fn main() -> Result<(), Error> {
    let mut event_loop = EventLoop::new(App::default())?;
    event_loop.run()
}
