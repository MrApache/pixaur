use toolkit::{
    types::{Argb8888, Color, Stroke}, widget::Container, window::WindowRequest, Anchor, Error, EventLoop, SpecialOptions, TargetMonitor, UserWindow, WindowId, GUI
};
use widgets::{panel::{HorizontalAlign, Panel, PanelBuilder, VerticalAlign}, text::Text};

//#[derive(Default)]
//struct App {}
//
//impl GUI for App {
//    fn setup_windows(&mut self) -> Vec<Box<dyn toolkit::UserWindow<Self>>> {
//        vec![Box::new(BarWindow)]
//    }
//}
//
//struct BarWindow;
//
//impl UserWindow<App> for BarWindow {
//
//    fn setup(&self, gui: &mut App) -> Box<dyn toolkit::widget::Container> {
//        let mut root = Panel::new();
//        root.background = Color::Simple(Argb8888::new(17, 17, 27, 255)).into();
//        root.stroke = Stroke::none();
//        root.vertical_align = VerticalAlign::Center;
//        root.horizontal_align = HorizontalAlign::Center;
//
//        let mut time = Text::default();
//        time.set_text("23:59");
//        //time.set_text("qq");
//        //time.set_text("qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq");
//        time.size = 16;
//        root.add_child(Box::new(time));
//
//        Box::new(root)
//    }
//
//    fn update<'ctx>(&mut self, gui: &mut App, context: &'ctx mut toolkit::Context<'ctx>) {}
//}

fn main() -> Result<(), Error> {

}


pub struct BarWindow;

impl UserWindow for BarWindow {
    fn request(&self) -> toolkit::window::WindowRequest {
        WindowRequest::new("bar")
            .with_size(1920, 35)
            .bottom(SpecialOptions {
                anchor: Anchor::Top,
                exclusive_zone: 35,
                target: TargetMonitor::Primary,
            })
    }

    fn setup(&self, commands: &mut Commands, window_id: WindowId) {
        PanelBuilder
    }
}
