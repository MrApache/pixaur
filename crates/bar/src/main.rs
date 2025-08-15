use bevy_ecs::system::Commands;
use toolkit::{
    types::{Argb8888, Stroke}, window::WindowRequest, Anchor, App, DesktopOptions, Error, SpecialOptions, TargetMonitor, UserWindow, WindowId
};
use widgets::panel::{HorizontalAlign, Panel, PanelBuilder, VerticalAlign};

//struct BarWindow;
//
//impl UserWindow<App> for BarWindow {
//
//    fn setup(&self, gui: &mut App) -> Box<dyn toolkit::widget::Container> {
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
//}

fn main() -> Result<(), Error> {
    let mut app = App::new()?;
    app.add_window(BarWindow);
    app.run()
}

pub struct BarWindow;

impl UserWindow for BarWindow {
    fn request(&self) -> toolkit::window::WindowRequest {
        WindowRequest::new("bar")
            //.desktop(DesktopOptions {
            //    title: "xd".into(),
            //    resizable: true,
            //    decorations: false,
            //})
            .with_size(1920, 35)
            .bottom(SpecialOptions {
                anchor: Anchor::Top,
                exclusive_zone: 35,
                target: TargetMonitor::Primary,
            })
    }

    fn setup(&self, commands: &mut Commands, window_id: WindowId) {
        let root = PanelBuilder::new(window_id, commands)
            .color(Argb8888::new(17, 17, 27, 255))
            //.color(Argb8888::new(255, 37, 207, 255))
            .stroke(Stroke::none())
            .panel(Panel {
                vertical_align: VerticalAlign::Center,
                horizontal_align: HorizontalAlign::Center,
                ..Default::default()
            })
            .build();
    }
}
