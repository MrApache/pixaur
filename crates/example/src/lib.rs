use widgets::{
    panel::{Panel, TestPanelLayoutWidget},
    text::Text,
};
use toolkit::{
    glam::Vec2,
    include_asset,
    types::{Argb8888, Color, LinearGradient, Texture},
    widget::Container,
    window::WindowRequest,
    ContentManager, Context, DesktopOptions, FontHandle,
    TextureHandle, UserWindow, GUI,
};

#[derive(Default)]
pub struct App {
    texture: TextureHandle,
    font: FontHandle,
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
        panel.background = Color::Simple(Argb8888::BLACK).into();

        let mut subpanel = Panel::new();

        let mut test_layout_widget = TestPanelLayoutWidget::default();
        test_layout_widget.min = Vec2::new(100.0, 100.0);
        subpanel.add_child(Box::new(test_layout_widget));

        subpanel.background = Texture::new(gui.texture).into();
        panel.add_child(Box::new(subpanel));

        let mut subpanel = Panel::new();
        subpanel.background =
            Color::LinearGradient(LinearGradient::new(Argb8888::PURPLE, Argb8888::BLUE, 45.0))
                .into();
        panel.add_child(Box::new(subpanel));

        Box::new(panel)
    }

    fn update<'ctx>(&mut self, _gui: &mut App, _context: &'ctx mut Context<'ctx>) {}
}
