use widgets::panel::{Panel, TestPanelLayoutWidget};
use toolkit::{
    glam::{Vec2, Vec4},
    widget::Container,
    window::WindowRequest,
    Anchor,
    Argb8888,
    Color,
    Context,
    DesktopOptions,
    EventLoop,
    SpecialOptions,
    UserWindow, 
    GUI
};

struct App;
impl GUI for App {
    fn setup_windows(&mut self) -> Vec<Box<dyn UserWindow<App>>> {
        vec![
            Box::new(MainWindow::default()),
            Box::new(SmartPanel),
        ]
    }
}

#[derive(Default)]
struct MainWindow {
    _value: u64,
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
        panel.background = Color::Simple(Argb8888::RED);

        let mut subpanel = Panel::new("Subpanel");
        subpanel.background = Color::Simple(Argb8888::GREEN);
        panel.add_child(Box::new(subpanel));

        let mut subpanel = Panel::new("Subpanel");
        subpanel.background = Color::Simple(Argb8888::BLUE);
        panel.add_child(Box::new(subpanel));

        Box::new(panel)
    }

    fn update<'ctx>(&mut self, _gui: &mut App, _context: &'ctx mut Context<'ctx>) {
    }
}


fn main() {
    let mut event_loop = EventLoop::new(App).unwrap();
    event_loop.run().unwrap();
}

struct SmartPanel;
impl UserWindow<App> for SmartPanel {
    fn request(&self) -> WindowRequest {
        WindowRequest::new("smart_panel")
            .bottom(SpecialOptions {
                anchor: Anchor::Bottom,
                exclusive_zone: 35,
                target: Default::default(),
            })
            .with_size(1920, 35)
    }

    fn setup(&self, _gui: &mut App) -> Box<dyn Container> {
        let mut panel = Panel::new("Panel");
        panel.background = Color::Simple(Argb8888::BLUE);
        panel.padding = Vec4::new(10.0, 10.0, 10.0, 10.0);

        let mut child_panel = Panel::new("panel1");
        child_panel.background = Color::Simple(Argb8888::RED);
        panel.add_child(Box::new(child_panel));

        let mut test_layout_widget = TestPanelLayoutWidget::default();
        test_layout_widget.min = Vec2::new(10.0, 100.0);
        panel.add_child(Box::new(test_layout_widget));

        let mut child_panel = Panel::new("panel1");
        child_panel.background = Color::Simple(Argb8888::GREEN);
        panel.add_child(Box::new(child_panel));

        Box::new(panel)
    }

    fn update<'ctx>(&mut self, _gui: &mut App, _context: &'ctx mut Context<'ctx>) {
    }
}
