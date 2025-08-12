use widgets::{
    panel::{Panel, TestPanelLayoutWidget},
    text::Text,
};

use toolkit::{
    Anchor, ContentManager, Context, DesktopOptions, EventLoop, FontHandle, GUI, SpecialOptions,
    TextureHandle, UserWindow,
    glam::{Vec2, Vec4},
    include_asset,
    types::*,
    widget::Container,
    window::WindowRequest,
};

#[derive(Default)]
struct App {
    texture: TextureHandle,
    font: FontHandle,
}

impl GUI for App {
    fn setup_windows(&mut self) -> Vec<Box<dyn UserWindow<App>>> {
        vec![Box::new(MainWindow::default()), Box::new(SmartPanel)]
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
        let mut panel = Panel::new("Panel");
        panel.background = Color::Simple(Argb8888::BLACK).into();

        let mut subpanel = Panel::new("Subpanel");

        let mut test_layout_widget = TestPanelLayoutWidget::default();
        test_layout_widget.min = Vec2::new(100.0, 100.0);
        subpanel.add_child(Box::new(test_layout_widget));

        //subpanel.background = Color::Simple(Argb8888::GREEN).into();
        subpanel.background = Texture::new(gui.texture)
            //.with_color(Color::LinearGradient(LinearGradient::new(
            //    Argb8888::WHITE,
            //    Argb8888::YELLOW,
            //)))
            .into();
        panel.add_child(Box::new(subpanel));

        let mut subpanel = Panel::new("Subpanel");
        subpanel.background =
            Color::LinearGradient(LinearGradient::new(Argb8888::PURPLE, Argb8888::BLUE, 45.0))
                .into();
        subpanel.padding = Vec4::new(0.0, 0.0, 0.0, 0.0);
        let mut text = Text::new(gui.font.clone(), "Label");
        text.value =
            "Hello, world! (1234567890-=_+qwertyuiop[]\\asd\nfghjkl;'zxcvbnm,./)".to_string();
        //text.value = "Hello, world!".to_string();
        text.color = Color::Simple(Argb8888::RED);
        //text.color = Color::LinearGradient(LinearGradient::new(Argb8888::RED, Argb8888::YELLOW));
        text.size = 16;
        subpanel.add_child(Box::new(text));
        panel.add_child(Box::new(subpanel));

        Box::new(panel)
    }

    fn update<'ctx>(&mut self, _gui: &mut App, _context: &'ctx mut Context<'ctx>) {}
}

fn main() {
    let mut event_loop = EventLoop::new(App::default()).unwrap();
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
        panel.background = Color::Simple(Argb8888::BLUE).into();
        panel.padding = Vec4::new(10.0, 10.0, 10.0, 10.0);

        let mut child_panel = Panel::new("panel1");
        child_panel.background = Color::Simple(Argb8888::RED).into();
        panel.add_child(Box::new(child_panel));

        let mut test_layout_widget = TestPanelLayoutWidget::default();
        test_layout_widget.min = Vec2::new(10.0, 100.0);
        panel.add_child(Box::new(test_layout_widget));

        let mut child_panel = Panel::new("panel1");
        child_panel.background = Color::Simple(Argb8888::GREEN).into();
        panel.add_child(Box::new(child_panel));

        Box::new(panel)
    }

    fn update<'ctx>(&mut self, _gui: &mut App, _context: &'ctx mut Context<'ctx>) {}
}
