use widgets::{
    panel::{Panel, TestPanelLayoutWidget},
};
use toolkit::{
    glam::Vec2,
    include_asset,
    types::{Argb8888, Color, LinearGradient, Texture},
    widget::{Container, DesiredSize, Widget},
    window::WindowRequest,
    ContentManager, Context, DesktopOptions,
    TextureHandle, UserWindow, GUI,
};

#[derive(Default)]
pub struct App {
    texture: TextureHandle,
}

impl GUI<Root, Panel<Root>> for App {
    fn setup_windows(&mut self) -> Vec<Box<dyn UserWindow<Root, Panel<Root>, App>>> {
        vec![Box::new(MainWindow::default())]
    }

    fn load_content(&mut self, content: &mut ContentManager) {
        self.texture = content.include_texture(include_asset!("billy.jpg"));
    }
}

#[derive(Default)]
struct MainWindow {
    _value: u64,
}

pub enum Root {
    Panel(Panel<Root>),
    TestPanel(TestPanelLayoutWidget),
}

impl Widget for Root {
    fn id(&self) -> Option<&str> {
        match self {
            Root::Panel(panel) => panel.id(),
            Root::TestPanel(panel) => panel.id()
        }
    }

    fn desired_size(&self) -> DesiredSize {
        match self {
            Root::Panel(panel) => panel.desired_size(),
            Root::TestPanel(panel) => panel.desired_size()
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        match self {
            Root::Panel(panel) => panel,
            Root::TestPanel(panel) => panel
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        match self {
            Root::Panel(panel) => panel,
            Root::TestPanel(panel) => panel
        }
    }

    fn draw<'frame>(&'frame self, out: &mut toolkit::commands::CommandBuffer<'frame>) {
        match self {
            Root::Panel(panel) => panel.draw(out),
            Root::TestPanel(panel) => panel.draw(out)
        }
    }

    fn layout(&mut self, bounds: toolkit::types::Rect) {
        match self {
            Root::Panel(panel) => panel.layout(bounds),
            Root::TestPanel(panel) => panel.layout(bounds)
        }
    }
}

impl UserWindow<Root, Panel<Root>, App> for MainWindow {
    fn request(&self) -> WindowRequest {
        WindowRequest::new("desktop").desktop(DesktopOptions {
            title: "Test application".into(),
            resizable: true,
            decorations: false,
        })
    }

    fn setup(&self, gui: &mut App) -> Panel<Root> {
        let mut panel = Panel::<Root>::new();
        panel.background = Color::Simple(Argb8888::BLACK).into();

        let mut test_layout_widget = TestPanelLayoutWidget::default();
        test_layout_widget.min = Vec2::new(100.0, 100.0);
        let test_layout_widget = Root::TestPanel(test_layout_widget);

        let mut panel_1 = Panel::<Root>::new();
        panel_1.add_child(test_layout_widget);
        panel_1.background = Texture::new(gui.texture).into();
        let subpanel = Root::Panel(panel_1);

        panel.add_child(subpanel);

        let mut panel_2 = Panel::<Root>::new();
        panel_2.background = Color::LinearGradient(LinearGradient::new(Argb8888::PURPLE, Argb8888::BLUE, 45.0)).into();
        let subpanel = Root::Panel(panel_2);
        panel.add_child(subpanel);

        panel
        //Box::new(panel)
    }

    fn update<'ctx>(&mut self, _gui: &mut App, _context: &'ctx mut Context<'ctx, Root, Panel<Root>>) {}
}
