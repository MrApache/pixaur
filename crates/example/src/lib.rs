use toolkit::{
    app::App,
    include_asset,
    types::{Argb8888, Color, LinearGradient, Texture},
    widget::{Callbacks, Container, Context, Sender, Tree, WidgetQuery},
    window::WindowRequest,
    ContentManager, DesktopOptions, Handle, WidgetEnum, WindowRoot,
};
use widgets::{
    impl_empty_widget,
    impl_proxy_widget,
    button::{Button, ButtonCallbacks},
    panel::{HorizontalAlign, Panel},
    timer::{Timer, TimerCallback},
};

impl_empty_widget!(Empty);

pub enum WindowContext {
    RandomColor,
}

impl Context for WindowContext {
    type Widget = Root;
    type WindowRoot = Root;
    fn execute(&self, _: &mut ContentManager, tree: &mut Tree<Self>) {
        match self {
            WindowContext::RandomColor => {
                let button = tree.get_mut_element::<Button<WindowContext, Empty, CallbackImpls>>("Button").unwrap();
                button.normal.background = Argb8888::random().into();
            }
        }
    }
}

#[derive(Default)]
pub struct Root(Panel<WindowContext, Elements>);
impl_proxy_widget!(Root, WindowContext);

impl WindowRoot<WindowContext, Self> for Root {
    fn request(&self) -> WindowRequest {
        WindowRequest::new("desktop").desktop(DesktopOptions {
            title: "Test application".into(),
            resizable: true,
            decorations: false,
        })
    }

    fn setup(&mut self, app: &mut App<WindowContext, Self, Self>) {
        let content_manager = app.content_manager();

        let texture = content_manager.include_texture(include_asset!("billy.jpg"));

        let mut panel = Panel::<WindowContext, Elements>::new();
        panel.horizontal_align = HorizontalAlign::Start;
        panel.rectangle.background = Argb8888::BLACK.into();

        {
            let mut inner_panel = Panel::<WindowContext, Elements>::new();
            inner_panel.rectangle.background = Argb8888::WHITE.into();
            inner_panel.horizontal_align = HorizontalAlign::Start;
            {
                for _ in 0..10 {
                    let mut empty_panel = Panel::<WindowContext, Empty>::new();
                    empty_panel.rectangle.background = Argb8888::random().into();
                    inner_panel.add_child(Elements::Empty(empty_panel));
                }
            }

            panel.add_child(Elements::Panel(inner_panel));
        }

        {
            let mut inner_panel = Panel::<WindowContext, Elements>::new();
            inner_panel.rectangle.background = Texture::new(Handle::Texture(texture)).into();
            {
                let button: Button<WindowContext, Empty, CallbackImpls> = Button::with_id("Button");
                inner_panel.add_child(Elements::Button(button));
            }

            panel.add_child(Elements::Panel(inner_panel));
        }

        {
            let mut inner_panel = Panel::<WindowContext, Elements>::new();
            inner_panel.rectangle.background =
                Color::LinearGradient(LinearGradient::new(Argb8888::PURPLE, Argb8888::BLUE, 45.0))
                    .into();
            inner_panel.horizontal_align = HorizontalAlign::Start;
            inner_panel.spacing = 10.0;
            {
                for _ in 0..5 {
                    let mut empty_panel = Panel::<WindowContext, Empty>::new();
                    empty_panel.rectangle.background = Argb8888::random().into();
                    inner_panel.add_child(Elements::Empty(empty_panel));
                }
            }
            panel.add_child(Elements::Panel(inner_panel));
        }

        {
            let mut inner_panel = Panel::<WindowContext, Elements>::with_id("Id");
            inner_panel.rectangle.background = Argb8888::WHITE.into();
            inner_panel.horizontal_align = HorizontalAlign::Start;
            {
                for _ in 0..10 {
                    let mut empty_panel = Panel::<WindowContext, Empty>::new();
                    empty_panel.rectangle.background = Argb8888::random().into();
                    inner_panel.add_child(Elements::Empty(empty_panel));
                }
            }

            panel.add_child(Elements::Panel(inner_panel));
        }
        let mut timer = Timer::new();
        timer.interval = 0.01;
        timer.running = true;
        timer.repeat = true;
        panel.add_child(Elements::Timer(timer));

        self.0 = panel;
    }

    fn root_mut(&mut self) -> &mut Self {
        self
    }

    fn root(&self) -> &Self {
        self
    }
}

#[derive(WidgetEnum)]
#[context(WindowContext)]
pub enum Elements {
    Panel(Panel<WindowContext, Elements>),
    Empty(Panel<WindowContext, Empty>),
    Timer(Timer<WindowContext, CallbackImpls>),
    Button(Button<WindowContext, Empty, CallbackImpls>),
}

impl Default for Elements {
    fn default() -> Self {
        Self::Panel(Panel::default())
    }
}

#[derive(Default)]
pub struct CallbackImpls;
impl Callbacks for CallbackImpls {}
impl ButtonCallbacks<WindowContext> for CallbackImpls {}
impl TimerCallback<WindowContext> for CallbackImpls {
    fn on_triggered(&self, sender: &mut Sender<WindowContext>) {
        sender.create_event(WindowContext::RandomColor);
    }
}
