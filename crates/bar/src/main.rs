use bevy_ecs::system::Commands;
use toolkit::{
    glam::Vec4, types::{Argb8888, Stroke}, window::WindowRequest, Anchor, App, DesktopOptions, Error, SpecialOptions, TargetMonitor, UserWindow, WindowId
};
use widgets::panel::{HorizontalAlign, Panel, PanelBuilder, PanelWidgetPlugin, VerticalAlign};

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
    app.add_plugin(PanelWidgetPlugin);
    app.add_window(BarWindow);
    app.run()
}

pub struct BarWindow;

impl UserWindow for BarWindow {
    fn request(&self) -> toolkit::window::WindowRequest {
        WindowRequest::new("bar")
            .desktop(DesktopOptions {
                title: "xd".into(),
                resizable: true,
                decorations: false,
            })
            //.with_size(1920, 35)
            //.bottom(SpecialOptions {
            //    anchor: Anchor::Top,
            //    exclusive_zone: 35,
            //    target: TargetMonitor::Primary,
            //})
    }

    fn setup(&self, commands: &mut Commands, window_id: WindowId) {
        let root = PanelBuilder::new(window_id, commands)
            .color(Argb8888::new(17, 17, 27, 255))
            //.color(Argb8888::new(255, 37, 207, 255))
            .stroke(Stroke::none())
            .panel(Panel::default()
                .with_padding(Vec4::new(10.0, 10.0, 10.0, 10.0))
                .with_h_align(HorizontalAlign::Center)
                .with_v_align(VerticalAlign::Center)
            )
            .build();


        //PanelBuilder::new(window_id, commands)
        //    .color(Argb8888::CYAN)
        //    .stroke(Stroke::none())
        //    .build_as_child_of(root);
    }
}
