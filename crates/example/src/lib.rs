use widgets::panel::{HorizontalAlign, Panel, TestPanelLayoutWidget};
use toolkit::{
    glam::Vec2,
    include_asset,
    types::{Argb8888, Color, LinearGradient, Texture},
    widget::Container,
    window::WindowRequest,
    ContentManager, Context, DesktopOptions,
    TextureHandle, UserWindow, GUI,
};

#[derive(Default)]
pub struct App {
    texture: TextureHandle,
}

impl GUI for App {
    fn setup_windows(&mut self) -> Vec<Box<dyn UserWindow<App>>> {
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

impl UserWindow<App> for MainWindow {
    fn request(&self) -> WindowRequest {
        WindowRequest::new("desktop").desktop(DesktopOptions {
            title: "Test application".into(),
            resizable: true,
            decorations: false,
        })
    }

    fn setup(&self, gui: &mut App) -> Box<dyn Container> {
        let mut panel = Panel::new();
        panel.horizontal_align = HorizontalAlign::Start;
        panel.background = Color::Simple(Argb8888::BLACK).into();

        {
            let mut inner_panel = Panel::new();
            inner_panel.background = Color::Simple(Argb8888::WHITE).into();
            inner_panel.horizontal_align = HorizontalAlign::Start;
            {
                for _ in 0..10 {
                    let mut empty_panel = Panel::new();
                    empty_panel.background = Color::Simple(Argb8888::random()).into();
                    inner_panel.add_child(Box::new(empty_panel));
                }
            }
            panel.add_child(Box::new(inner_panel));
        }

        {
            let mut inner_panel = Panel::new();
            inner_panel.background = Texture::new(gui.texture).into();

            {
                let mut test_layout_widget = TestPanelLayoutWidget::default();
                test_layout_widget.min = Vec2::new(100.0, 100.0);
                inner_panel.add_child(Box::new(test_layout_widget));
            }

            panel.add_child(Box::new(inner_panel));
        }

        {
            let mut inner_panel = Panel::new();
            inner_panel.background = Color::LinearGradient(LinearGradient::new(Argb8888::PURPLE, Argb8888::BLUE, 45.0)).into();
            inner_panel.horizontal_align = HorizontalAlign::Start;
            inner_panel.spacing = 10.0;
            {
                for _ in 0..5 {
                    let mut empty_panel = Panel::new();
                    empty_panel.background = Color::Simple(Argb8888::random()).into();
                    inner_panel.add_child(Box::new(empty_panel));
                }
            }
            panel.add_child(Box::new(inner_panel));
        }

        {
            let mut inner_panel = Panel::with_id("Id");
            inner_panel.background = Color::Simple(Argb8888::WHITE).into();
            inner_panel.horizontal_align = HorizontalAlign::Start;
            {
                for _ in 0..10 {
                    let mut empty_panel = Panel::new();
                    empty_panel.background = Color::Simple(Argb8888::random()).into();
                    inner_panel.add_child(Box::new(empty_panel));
                }
            }

            panel.add_child(Box::new(inner_panel));
        }


        Box::new(panel)
    }

    fn update<'ctx>(&mut self, _gui: &mut App, _context: &'ctx mut Context<'ctx>) {}
}
