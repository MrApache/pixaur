use toolkit::{types::{Argb8888, Color, Corners, Stroke}, window::WindowRequest, Anchor, Error, EventLoop, SpecialOptions, UserWindow, GUI};
use widgets::panel::Panel;

#[derive(Default)]
struct App {

}

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
                target: Default::default(), //TODO expose TargetMonitor
            })
    }

    fn setup(&self, gui: &mut App) -> Box<dyn toolkit::widget::Container> {
        let mut panel = Panel::new(""); //TODO Allow 'None' id
        
        panel.background = Color::Simple(Argb8888::new(17, 17, 27, 255)).into();
        panel.stroke = Stroke { //TODO fn none()
            color: Argb8888::TRANSPARENT,
            width: 0.0,
            corners: Corners { //TODO fn none()
                left_top: 0.0,
                left_bottom: 0.0,
                right_top: 0.0,
                right_bottom: 0.0,
            },
        };
        Box::new(panel)
    }

    fn update<'ctx>(&mut self, gui: &mut App, context: &'ctx mut toolkit::Context<'ctx>) {

    }
}

fn main() -> Result<(), Error>{
    let mut event_loop = EventLoop::new(App::default())?;
    event_loop.run()
}
