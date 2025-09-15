use toolkit::{
    glam::Vec2,
    include_asset,
    types::{Argb8888, Stroke},
    widget::{Container, FrameContext, Widget},
    window::WindowRequest,
    Anchor, DesktopOptions, Error, EventLoop, FontHandle, Handle, SpecialOptions, SvgHandle,
    TargetMonitor, WidgetEnum, WindowRoot, WindowRootEnum, GUI,
};
use widgets::{
    button::{Button, ButtonMockCallbacks},
    image::Image,
    impl_empty_widget,
    panel::{HorizontalAlign, Panel, VerticalAlign},
    text::Text,
};

impl_empty_widget!(Empty);

#[derive(Default)]
struct App {
    font: FontHandle,
    icon: SvgHandle,
}

impl GUI for App {
    type Window = Windows;

    fn setup_windows(&mut self) -> Vec<Self::Window> {
        vec![
            //Windows::TestWindow(TestWindowImpl::default()),
            Windows::BarWindow(BarWindowImpl::default()),
        ]
    }

    fn load_content(&mut self, content: &mut toolkit::ContentManager) {
        self.font = content.include_font(include_asset!("MSW98UI-Regular.ttf"));
        self.icon = content.include_svg_as_texture(include_asset!("arch.svg"), 25, 25);
    }
}

#[derive(WindowRootEnum)]
#[window_gui(App)]
enum Windows {
    BarWindow(BarWindowImpl),
    TestWindow(TestWindowImpl),
}

#[derive(WidgetEnum)]
enum BarWindow {
    Text(Text),
    Button(Button<ButtonMockCallbacks, Image>),
}

impl Default for BarWindow {
    fn default() -> Self {
        Self::Text(Text::default())
    }
}

#[derive(Default)]
struct BarWindowImpl {
    root: Panel<BarWindow>,
}

impl BarWindowImpl {
    fn request(&self) -> toolkit::window::WindowRequest {
        WindowRequest::new("bar")
            .with_size(1920, 35)
            .bottom(SpecialOptions {
                anchor: Anchor::Top,
                exclusive_zone: 35,
                target: TargetMonitor::Primary,
            })
    }

    fn setup(&mut self, app: &mut App) {
        let mut root = Panel::<BarWindow>::new();
        root.rectangle.background = Argb8888::new(212, 208, 200, 255).into();
        root.rectangle.stroke = Stroke::none();
        root.vertical_align = VerticalAlign::Center;
        root.horizontal_align = HorizontalAlign::Center;

        let mut time = Text::new(app.font.clone());
        time.set_text("23:59");
        time.size = 14;
        time.color = Argb8888::BLACK.into();
        root.add_child(BarWindow::Text(time));

        let mut button: Button<ButtonMockCallbacks, Image> = Button::new();
        let content = button.content_mut();
        content.size = Vec2::new(25.0, 25.0);
        content.handle = Some(Handle::Svg(app.icon));
        button.normal.background = Argb8888::new(212, 208, 200, 255).into();
        button.normal.stroke.width = 1.0;
        button.normal.stroke.color = [
            Argb8888::WHITE,
            Argb8888::BLACK,
            Argb8888::WHITE,
            Argb8888::BLACK,
        ];

        button.hover = button.normal.clone();

        button.pressed.background = Argb8888::new(192, 192, 192, 255).into();
        button.pressed.stroke.width = 1.0;
        button.pressed.stroke.color = [
            Argb8888::new(128, 128, 128, 255),
            Argb8888::WHITE,
            Argb8888::new(128, 128, 128, 255),
            Argb8888::WHITE,
        ];

        root.add_child(BarWindow::Button(button));

        self.root = root;
    }

    fn draw<'frame>(&'frame self, out: &mut toolkit::commands::CommandBuffer<'frame>) {
        Widget::draw(&self.root, out);
    }

    fn layout(&mut self, bounds: toolkit::types::Rect) {
        Widget::layout(&mut self.root, bounds);
    }

    fn update(&mut self, _: &mut App, ctx: &FrameContext) {
        Widget::update(&mut self.root, ctx);
    }
}

#[derive(Default)]
pub struct TestWindowImpl {
    root: Panel<Empty>,
}

impl TestWindowImpl {
    fn request(&self) -> toolkit::window::WindowRequest {
        WindowRequest::new("panel")
            .with_size(600, 600)
            .desktop(DesktopOptions {
                title: "Window".into(),
                resizable: true,
                decorations: false,
            })
    }

    fn setup(&mut self, _: &mut App) {
        let mut root = Panel::<Empty>::new();
        root.rectangle.background = Argb8888::RED.into();
        root.rectangle.stroke = Stroke::none();

        self.root = root;
    }

    fn draw<'frame>(&'frame self, out: &mut toolkit::commands::CommandBuffer<'frame>) {
        Widget::draw(&self.root, out);
    }

    fn layout(&mut self, bounds: toolkit::types::Rect) {
        Widget::layout(&mut self.root, bounds);
    }

    fn update(&mut self, _: &mut App, ctx: &FrameContext) {
        Widget::update(&mut self.root, ctx);
    }
}

fn main() -> Result<(), Error> {
    let mut event_loop = EventLoop::new(App::default())?;
    event_loop.run()
}
