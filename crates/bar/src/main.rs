use toolkit::{
    glam::Vec2,
    include_asset,
    types::{Argb8888, Stroke},
    widget::{Container, Context},
    window::WindowRequest,
    Anchor, Error, EventLoop, FontHandle, Handle, SpecialOptions, SvgHandle, TargetMonitor,
    WidgetEnum, WindowRoot, GUI,
};
use widgets::{
    button::{Button, ButtonMockCallbacks},
    image::Image,
    impl_empty_widget, impl_proxy_widget,
    panel::{HorizontalAlign, Panel, VerticalAlign},
    text::Text,
};

impl_empty_widget!(Empty);

#[derive(Default)]
struct App {
    font: FontHandle,
    icon: SvgHandle,
}

impl GUI<BarWindowContext, Root> for App {
    type Window = Root;

    fn setup_windows(&mut self) -> Vec<Self::Window> {
        vec![Root::default()]
    }

    fn load_content(&mut self, content: &mut toolkit::ContentManager) {
        self.font = content.include_font(include_asset!("MSW98UI-Regular.ttf"));
        self.icon = content.include_svg_as_texture(include_asset!("arch.svg"), 16, 16);
    }
}

enum BarWindowContext {
    UpdateClock(String),
}

impl Context for BarWindowContext {
    fn execute(&self) {
        match self {
            BarWindowContext::UpdateClock(id) => {
                //let local: chrono::DateTime<chrono::Local> = chrono::Local::now();
                //text.set_text(&format!("{}", local.format("%Y-%m-%d %H:%M:%S")));
            }
        }
    }
}

#[derive(WidgetEnum)]
#[context(BarWindowContext)]
enum BarWindowElements {
    Text(Text<BarWindowContext>),
    Button(Button<BarWindowContext, Image, ButtonMockCallbacks>),
}

impl Default for BarWindowElements {
    fn default() -> Self {
        Self::Text(Text::default())
    }
}

#[derive(Default)]
pub struct Root(Panel<BarWindowContext, BarWindowElements>);
impl_proxy_widget!(Root, BarWindowContext);

impl WindowRoot<BarWindowContext, Root> for Root {
    type Gui = App;

    fn request(&self) -> toolkit::window::WindowRequest {
        WindowRequest::new("bar")
            .with_size(1920, 30)
            .bottom(SpecialOptions {
                anchor: Anchor::Bottom,
                exclusive_zone: 30,
                target: TargetMonitor::Primary,
            })
    }

    fn setup(&mut self, app: &mut App) {
        let mut root = Panel::<BarWindowContext, BarWindowElements>::new();
        root.padding.left = 6.0;
        root.padding.right = 6.0;
        root.rectangle.background = Argb8888::new(212, 208, 200, 255).into();
        root.rectangle.stroke = Stroke::NONE;
        root.vertical_align = VerticalAlign::Center;
        root.horizontal_align = HorizontalAlign::Center;

        let mut time = Text::with_id("Clock");
        time.set_font(app.font.clone());
        time.set_text("left: true false");
        time.size = 14;
        time.color = Argb8888::BLACK.into();
        root.add_child(BarWindowElements::Text(time));

        let mut button: Button<BarWindowContext, Image, ButtonMockCallbacks> = Button::new();
        button.size = Vec2::new(24.0, 24.0);
        let content = button.content_mut();
        content.size = Vec2::new(16.0, 16.0);
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

        root.add_child(BarWindowElements::Button(button));

        self.0 = root;
    }

    fn root(&mut self) -> &mut Root {
        self
    }
}

fn main() -> Result<(), Error> {
    let mut event_loop = EventLoop::new(App::default())?;
    event_loop.run()
}
