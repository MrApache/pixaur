#![allow(clippy::large_enum_variant)]

use toolkit::{
    app::App,
    glam::Vec2,
    include_asset,
    types::{Argb8888, Stroke},
    widget::{Anchor, Callbacks, Context, NoID, Sender, Spacing, StaticID, Tree, WidgetQuery},
    window::WindowRequest,
    ContentManager, Error, EventLoop, Handle, SpecialOptions, TargetMonitor, WidgetEnum,
    WindowRoot,
};
use widgets::{
    button::{Button, ButtonCallbacks},
    image::Image,
    impl_proxy_widget,
    rectangle::Rectangle,
    row::Row,
    text::Text,
    timer::{Timer, TimerCallback},
};

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
                let clock = tree.get_mut_element::<Text<Self, StaticID>>("Clock").unwrap();
                let local: chrono::DateTime<chrono::Local> = chrono::Local::now();
                clock.set_text(&format!("{}", local.format("%H:%M")));
            }
            WindowContext::PrintText => println!("Hello, World!"),
        }
    }
}

#[derive(WidgetEnum)]
#[context(WindowContext)]
//552 - Before
//480 - After
enum Elements {
    Button(
        Button<
            WindowContext,
            Rectangle<WindowContext, Row<WindowContext, StartButton, NoID>, NoID>,
            CallbackImpls,
            NoID,
        >,
    ),
    Tray(Row<WindowContext, TrayElements, NoID>),
}

impl Default for Elements {
    fn default() -> Self {
        Self::Tray(Row::default())
    }
}

#[derive(WidgetEnum)]
#[context(WindowContext)]
//280 - Before
//256 - After
enum StartButton {
    Icon(Image<WindowContext, NoID>),
    Text(Text<WindowContext, NoID>),
}

impl Default for StartButton {
    fn default() -> Self {
        Self::Icon(Image::default())
    }
}

#[derive(WidgetEnum)]
#[context(WindowContext)]
//280 - Before
//272 - After
enum TrayElements {
    Text(Text<WindowContext, StaticID>),
    Timer(Timer<WindowContext, CallbackImpls, NoID>),
}

impl Default for TrayElements {
    fn default() -> Self {
        Self::Text(Text::default())
    }
}

#[derive(Default)]
pub struct Root(Row<WindowContext, Elements>);
impl_proxy_widget!(Root, WindowContext);

impl WindowRoot<WindowContext, Root> for Root {
    fn request(&self) -> WindowRequest {
        WindowRequest::new("bar")
            .with_size(1920, 35)
            .bottom(SpecialOptions {
                anchor: toolkit::Anchor::Bottom,
                exclusive_zone: 35,
                target: TargetMonitor::Primary,
            })
    }

    fn setup(&mut self, app: &mut App<WindowContext, Self, Self>) {
        let content_manager = app.content_manager();
        let font = content_manager.include_font(include_asset!("MSW98UI-Regular.ttf"));
        let icon = content_manager.include_svg_as_texture(include_asset!("arch.svg"), 20, 20);

        let mut root = Row::<WindowContext, Elements>::new_default();
        root.spacing = 2.0;
        root.background = Argb8888::new(212, 208, 200, 255).into();
        root.stroke = Stroke::NONE;

        let mut start: Button<
            WindowContext,
            Rectangle<WindowContext, Row<WindowContext, StartButton, NoID>, NoID>,
            CallbackImpls,
            NoID,
        > = Button::new();

        println!(
            "Size of tray elements {}",
            std::mem::size_of::<TrayElements>()
        );

        start.padding = Spacing::all(1.0);
        start.size = Vec2::new(67.0, 30.0);
        start.anchor |= Anchor::VerticalCenter;
        start.normal.background = Argb8888::new(212, 208, 200, 255).into();
        start.normal.stroke.color = [
            Argb8888::WHITE,
            Argb8888::BLACK,
            Argb8888::WHITE,
            Argb8888::BLACK,
        ];

        start.hover = start.normal.clone();

        start.pressed.background = Argb8888::new(192, 192, 192, 255).into();
        start.pressed.stroke.color = [
            Argb8888::new(128, 128, 128, 255),
            Argb8888::WHITE,
            Argb8888::new(128, 128, 128, 255),
            Argb8888::WHITE,
        ];

        let rectangle = start.content_mut();
        rectangle.background = Argb8888::TRANSPARENT.into();
        rectangle.stroke.color = [
            Argb8888::TRANSPARENT,
            Argb8888::new(128, 128, 128, 255),
            Argb8888::TRANSPARENT,
            Argb8888::new(128, 128, 128, 255),
        ];

        let row = rectangle.content_mut();
        row.background = Argb8888::TRANSPARENT.into();
        row.spacing = 5.0;

        let mut image = Image::new();
        image.handle = Some(Handle::Svg(icon));
        image.size = Vec2::new(20.0, 20.0);
        image.anchor |= Anchor::VerticalCenter;
        row.content_mut().push(StartButton::Icon(image));

        let mut text = Text::new();
        text.set_font(font.clone());
        text.set_text("Irisu");
        text.size = 18;
        text.color = Argb8888::BLACK;
        text.anchor |= Anchor::VerticalCenter;
        text.margin.top = 3.0;
        row.content_mut().push(StartButton::Text(text));

        root.content_mut().push(Elements::Button(start));

        let mut tray = Row::new();
        {
            tray.anchor = Anchor::Right | Anchor::VerticalCenter;
            tray.height = Some(35.0);
            tray.width = Some(100.0);
            tray.background = Argb8888::new(192, 192, 192, 255).into();
            tray.stroke.width = 2.0;
            tray.stroke.color = [
                Argb8888::new(128, 128, 128, 255),
                Argb8888::WHITE,
                Argb8888::new(128, 128, 128, 255),
                Argb8888::WHITE,
            ];

            let mut time = Text::new_static("Clock");
            time.set_font(font.clone());
            time.set_text("00:00");
            time.size = 18;
            time.color = Argb8888::BLACK;
            time.anchor = Anchor::Right | Anchor::VerticalCenter;
            time.margin.right = 5.0;
            time.margin.top = 3.0;
            tray.content_mut().push(TrayElements::Text(time));

            let mut timer = Timer::<WindowContext, CallbackImpls, NoID>::new();
            timer.interval = 0.1;
            timer.running = true;
            timer.repeat = true;
            tray.content_mut().push(TrayElements::Timer(timer));
        }

        root.content_mut().push(Elements::Tray(tray));

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

    /*
    let mut event_loop = toolkit::headless::HeadlessEventLoop::new(app);
    loop {
        event_loop.run_logic();
        event_loop.run_draw();

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    */
}
