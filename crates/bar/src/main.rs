use toolkit::{
    app::App,
    glam::Vec2,
    include_asset,
    types::{Argb8888, Stroke},
    widget::{Callbacks, Container, Context, Sender, Tree, WidgetQuery},
    window::WindowRequest,
    Anchor, ContentManager, Error, EventLoop, Handle, SpecialOptions, TargetMonitor, WidgetEnum,
    WindowRoot,
};
use widgets::{
    button::{Button, ButtonCallbacks},
    image::Image,
    impl_empty_widget, impl_proxy_widget,
    panel::{HorizontalAlign, Panel, VerticalAlign},
    text::Text,
    timer::{Timer, TimerCallback},
};
impl_empty_widget!(Empty);

enum WindowContext {
    UpdateClock,
    PrintText,
}

impl Context for WindowContext {
    type Widget = Root;
    type WindowRoot = Root;
    fn execute(&self, _: &mut ContentManager, tree: &mut Tree<Self>) {
        match self {
            WindowContext::UpdateClock => {
                let clock = tree.get_mut_element::<Text<Self>>("Clock").unwrap();
                let local: chrono::DateTime<chrono::Local> = chrono::Local::now();
                clock.set_text(&format!("{}", local.format("%Y-%m-%d %H:%M:%S")));
            }
            WindowContext::PrintText => println!("Hello, World!"),
        }
    }
}

#[derive(WidgetEnum)]
#[context(WindowContext)]
enum Elements {
    Text(Text<WindowContext>),
    Button(Button<WindowContext, Image<WindowContext>, CallbackImpls>),
    Timer(Timer<WindowContext, CallbackImpls>),
}

impl WidgetQuery<WindowContext> for Elements {
    fn get_element<QW: toolkit::widget::Widget<WindowContext>>(&self, id: &str) -> Option<&QW> {
        match self {
            Elements::Text(text) => text.get_element(id),
            Elements::Button(button) => button.get_element(id),
            Elements::Timer(timer) => timer.get_element(id),
        }
    }

    fn get_mut_element<QW: toolkit::widget::Widget<WindowContext>>(
        &mut self,
        id: &str,
    ) -> Option<&mut QW> {
        match self {
            Elements::Text(text) => text.get_mut_element(id),
            Elements::Button(button) => button.get_mut_element(id),
            Elements::Timer(timer) => timer.get_mut_element(id),
        }
    }
}

impl Default for Elements {
    fn default() -> Self {
        Self::Text(Text::default())
    }
}

#[derive(Default)]
pub struct Root(Panel<WindowContext, Elements>);
impl_proxy_widget!(Root, WindowContext);

impl WindowRoot<WindowContext, Root> for Root {
    fn request(&self) -> WindowRequest {
        WindowRequest::new("bar")
            .with_size(1920, 30)
            .bottom(SpecialOptions {
                anchor: Anchor::Bottom,
                exclusive_zone: 30,
                target: TargetMonitor::Primary,
            })
    }

    fn setup(&mut self, app: &mut App<WindowContext, Self, Self>) {
        let content_manager = app.content_manager();
        let font = content_manager.include_font(include_asset!("MSW98UI-Regular.ttf"));
        let icon = content_manager.include_svg_as_texture(include_asset!("arch.svg"), 16, 16);

        let mut root = Panel::<WindowContext, Elements>::new();
        root.padding.left = 6.0;
        root.padding.right = 6.0;
        root.rectangle.background = Argb8888::new(212, 208, 200, 255).into();
        root.rectangle.stroke = Stroke::NONE;
        root.vertical_align = VerticalAlign::Center;
        root.horizontal_align = HorizontalAlign::Center;

        let mut time = Text::with_id("Clock");
        time.set_font(font.clone());
        time.set_text("left: true false");
        time.size = 14;
        time.color = Argb8888::BLACK.into();
        root.add_child(Elements::Text(time));

        let mut button: Button<WindowContext, Image<WindowContext>, CallbackImpls> = Button::new();
        button.size = Vec2::new(24.0, 24.0);
        let content = button.content_mut();
        content.size = Vec2::new(16.0, 16.0);
        content.handle = Some(Handle::Svg(icon));
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

        let mut timer = Timer::<WindowContext, CallbackImpls>::new();
        timer.interval = 0.1;
        timer.running = true;
        timer.repeat = true;

        root.add_child(Elements::Timer(timer));
        root.add_child(Elements::Button(button));

        self.0 = root;
    }

    fn root_mut(&mut self) -> &mut Root {
        self
    }

    fn root(&self) -> &Root {
        self
    }
}

#[derive(Default)]
struct CallbackImpls;
impl Callbacks for CallbackImpls {}
impl TimerCallback<WindowContext> for CallbackImpls {
    fn on_triggered(&self, sender: &mut Sender<WindowContext>) {
        sender.create_event(WindowContext::UpdateClock);
    }
}

impl ButtonCallbacks<WindowContext> for CallbackImpls {
    fn on_clicked(&self, sender: &mut Sender<WindowContext>) {
        sender.create_event(WindowContext::PrintText);
    }
}

fn main() -> Result<(), Error> {
    let mut app = App::new();
    app.add_window(Root::default());

    let mut event_loop = EventLoop::new(app)?;
    event_loop.run()
}
