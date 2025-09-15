use toolkit::{
    glam::Vec2,
    include_asset,
    types::{Argb8888, Color, LinearGradient, Texture},
    widget::{Container, FrameContext, Widget},
    window::WindowRequest,
    ContentManager, DesktopOptions, Handle, SvgHandle, TextureHandle, WidgetEnum, WindowRoot, GUI,
};
use widgets::{
    button::{Button, ButtonCallbacks},
    impl_empty_widget,
    panel::{HorizontalAlign, Panel, TestPanelLayoutWidget},
};

#[derive(Default)]
pub struct App {
    texture: TextureHandle,
    icon: SvgHandle,
}

impl GUI for App {
    type Window = MainWindow;

    fn load_content(&mut self, content: &mut ContentManager) {
        self.texture = content.include_texture(include_asset!("billy.jpg"));
        self.icon = content.include_svg_as_texture(include_asset!("arch.svg"), 50, 50);
    }

    fn setup_windows(&mut self) -> Vec<MainWindow> {
        vec![MainWindow::default()]
    }
}

#[derive(Default)]
pub struct MainWindow {
    root: Panel<Root>,
}

impl WindowRoot for MainWindow {
    type Gui = App;

    fn request(&self) -> WindowRequest {
        WindowRequest::new("desktop").desktop(DesktopOptions {
            title: "Test application".into(),
            resizable: true,
            decorations: false,
        })
    }

    fn draw<'frame>(&'frame self, out: &mut toolkit::commands::CommandBuffer<'frame>) {
        Widget::draw(&self.root, out);
    }

    fn layout(&mut self, bounds: toolkit::types::Rect) {
        Widget::layout(&mut self.root, bounds);
    }

    fn setup(&mut self, gui: &mut Self::Gui) {
        let mut panel = Panel::<Root>::new();
        panel.horizontal_align = HorizontalAlign::Start;
        panel.rectangle.background = Argb8888::BLACK.into();

        {
            let mut inner_panel = Panel::<Root>::new();
            inner_panel.rectangle.background = Argb8888::WHITE.into();
            inner_panel.horizontal_align = HorizontalAlign::Start;
            {
                for _ in 0..10 {
                    let mut empty_panel = Panel::<Empty>::new();
                    empty_panel.rectangle.background = Argb8888::random().into();
                    inner_panel.add_child(Root::Empty(empty_panel));
                }
            }

            panel.add_child(Root::Panel(inner_panel));
        }

        {
            let mut inner_panel = Panel::<Root>::new();
            inner_panel.rectangle.background = Texture::new(Handle::Svg(gui.icon)).into();

            {
                let mut test_layout_widget = TestPanelLayoutWidget::default();
                test_layout_widget.min = Vec2::new(100.0, 100.0);
                inner_panel.add_child(Root::TestPanel(test_layout_widget));
            }
            {
                let button: Button<TestCallbacks, Empty> = Button::new();
                inner_panel.add_child(Root::Button(button));
            }

            panel.add_child(Root::Panel(inner_panel));
        }

        {
            let mut inner_panel = Panel::<Root>::new();
            inner_panel.rectangle.background =
                Color::LinearGradient(LinearGradient::new(Argb8888::PURPLE, Argb8888::BLUE, 45.0))
                    .into();
            inner_panel.horizontal_align = HorizontalAlign::Start;
            inner_panel.spacing = 10.0;
            {
                for _ in 0..5 {
                    let mut empty_panel = Panel::<Empty>::new();
                    empty_panel.rectangle.background = Argb8888::random().into();
                    inner_panel.add_child(Root::Empty(empty_panel));
                }
            }
            panel.add_child(Root::Panel(inner_panel));
        }

        {
            let mut inner_panel = Panel::<Root>::with_id("Id");
            inner_panel.rectangle.background = Argb8888::WHITE.into();
            inner_panel.horizontal_align = HorizontalAlign::Start;
            {
                for _ in 0..10 {
                    let mut empty_panel = Panel::<Empty>::new();
                    empty_panel.rectangle.background = Argb8888::random().into();
                    inner_panel.add_child(Root::Empty(empty_panel));
                }
            }

            panel.add_child(Root::Panel(inner_panel));
        }

        self.root = panel;
    }

    fn update(&mut self, _: &mut Self::Gui, ctx: &FrameContext) {
        self.root.update(ctx);
    }
}

#[derive(WidgetEnum)]
pub enum Root {
    Panel(Panel<Root>),
    TestPanel(TestPanelLayoutWidget),
    Empty(Panel<Empty>),
    Button(Button<TestCallbacks, Empty>),
}

impl Default for Root {
    fn default() -> Self {
        Self::Panel(Panel::default())
    }
}

impl_empty_widget!(Empty);

#[derive(Default)]
pub struct TestCallbacks;

impl ButtonCallbacks for TestCallbacks {}
