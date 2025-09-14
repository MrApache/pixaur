use widgets::panel::{HorizontalAlign, Panel, TestPanelLayoutWidget};
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
    Empty(Panel<Empty>)
}

impl Widget for Root {
    fn id(&self) -> Option<&str> {
        match self {
            Root::Panel(panel) => panel.id(),
            Root::TestPanel(panel) => panel.id(),
            Root::Empty(panel) => panel.id(),
        }
    }

    fn desired_size(&self) -> DesiredSize {
        match self {
            Root::Panel(panel) => panel.desired_size(),
            Root::TestPanel(panel) => panel.desired_size(),
            Root::Empty(panel) => panel.desired_size(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        match self {
            Root::Panel(panel) => panel,
            Root::TestPanel(panel) => panel,
            Root::Empty(panel) => panel,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        match self {
            Root::Panel(panel) => panel,
            Root::TestPanel(panel) => panel,
            Root::Empty(panel) => panel,
        }
    }

    fn draw<'frame>(&'frame self, out: &mut toolkit::commands::CommandBuffer<'frame>) {
        match self {
            Root::Panel(panel) => panel.draw(out),
            Root::TestPanel(panel) => panel.draw(out),
            Root::Empty(panel) => panel.draw(out),
        }
    }

    fn layout(&mut self, bounds: toolkit::types::Rect) {
        match self {
            Root::Panel(panel) => panel.layout(bounds),
            Root::TestPanel(panel) => panel.layout(bounds),
            Root::Empty(panel) => panel.layout(bounds),
        }
    }
}

pub struct Empty;
impl Widget for Empty {
    fn id(&self) -> Option<&str> {
        None
    }

    fn desired_size(&self) -> DesiredSize {
        DesiredSize::Min(Vec2::ZERO)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw<'frame>(&'frame self, _: &mut toolkit::commands::CommandBuffer<'frame>) {
    }

    fn layout(&mut self, _: toolkit::types::Rect) {
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
        panel.horizontal_align = HorizontalAlign::Start;
        panel.background = Color::Simple(Argb8888::BLACK).into();

        {
            let mut inner_panel = Panel::<Root>::new();
            inner_panel.background = Color::Simple(Argb8888::WHITE).into();
            inner_panel.horizontal_align = HorizontalAlign::Start;
            {
                for _ in 0..10 {
                    let mut empty_panel = Panel::<Empty>::new();
                    empty_panel.background = Color::Simple(Argb8888::random()).into();
                    inner_panel.add_child(Root::Empty(empty_panel));
                }
            }

            panel.add_child(Root::Panel(inner_panel));
        }

        {
            let mut inner_panel = Panel::<Root>::new();
            inner_panel.background = Texture::new(gui.texture).into();

            {
                let mut test_layout_widget = TestPanelLayoutWidget::default();
                test_layout_widget.min = Vec2::new(100.0, 100.0);
                inner_panel.add_child(Root::TestPanel(test_layout_widget));
            }

            panel.add_child(Root::Panel(inner_panel));
        }


        {
            let mut inner_panel = Panel::<Root>::new();
            inner_panel.background = Color::LinearGradient(LinearGradient::new(Argb8888::PURPLE, Argb8888::BLUE, 45.0)).into();
            inner_panel.horizontal_align = HorizontalAlign::Start;
            inner_panel.spacing = 10.0;
            {
                for _ in 0..5 {
                    let mut empty_panel = Panel::<Empty>::new();
                    empty_panel.background = Color::Simple(Argb8888::random()).into();
                    inner_panel.add_child(Root::Empty(empty_panel));
                }
            }
            panel.add_child(Root::Panel(inner_panel));
        }

        {
            let mut inner_panel = Panel::<Root>::with_id("Id");
            inner_panel.background = Color::Simple(Argb8888::WHITE).into();
            inner_panel.horizontal_align = HorizontalAlign::Start;
            {
                for _ in 0..10 {
                    let mut empty_panel = Panel::<Empty>::new();
                    empty_panel.background = Color::Simple(Argb8888::random()).into();
                    inner_panel.add_child(Root::Empty(empty_panel));
                }
            }

            panel.add_child(Root::Panel(inner_panel));
        }

        panel
    }

    fn update<'ctx>(&mut self, _gui: &mut App, _context: &'ctx mut Context<'ctx, Root, Panel<Root>>) {}
}
