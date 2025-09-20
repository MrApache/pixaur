use toolkit::{
    app::App,
    glam::Vec2,
    include_asset,
    types::{Argb8888, Color, LinearGradient, Texture},
    widget::{Anchor, Callbacks, Context, Empty, NoID, Sender, StaticID, Tree, WidgetQuery},
    window::WindowRequest,
    ContentManager, DesktopOptions, Handle, WidgetEnum, WindowRoot,
};
use widgets::{
    button::{Button, ButtonCallbacks},
    impl_proxy_widget,
    rectangle::Rectangle,
    row::Row,
    timer::{Timer, TimerCallback},
};

#[derive(Default)]
pub enum WindowContext {
    #[default]
    RandomColor,
}

impl Context for WindowContext {
    type Widget = Root;
    type WindowRoot = Root;
    fn execute(&self, _: &mut ContentManager, tree: &mut Tree<Self>) {
        match self {
            WindowContext::RandomColor => {
                //let button = tree
                //    .get_mut_element::<Button<WindowContext, Empty, CallbackImpls, StaticID>>("Button")
                //    .unwrap();
                //button.normal.background = Argb8888::random().into();
            }
        }
    }
}

#[derive(Default)]
pub struct Root(Row<WindowContext, Elements, NoID>);
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

        let mut root = Row::new();
        root.background = Argb8888::RED.into();
        {
            let mut inner_row = Row::new();
            inner_row.background = Argb8888::WHITE.into();
            inner_row.spacing = 10.0;
            {
                for _ in 0..10 {
                    let mut empty = Rectangle::<WindowContext, Empty, NoID>::new();
                    empty.background = Argb8888::random().into();
                    inner_row.content_mut().push(Elements::Empty(empty));
                }
            }

            root.content_mut().push(Elements::Row(inner_row));
        }

        {
            let mut inner_row = Row::new();
            inner_row.background = Texture::new(Handle::Texture(texture)).into();
            {
                let mut button: Button<WindowContext, Empty, CallbackImpls, StaticID> =
                    Button::new_static("Button");
                button.size = Vec2::new(100.0, 100.0);
                button.anchor = Anchor::Center;
                inner_row.content_mut().push(Elements::Button(button));
            }

            root.content_mut().push(Elements::Row(inner_row));
        }

        {
            let mut inner_row = Row::new();
            inner_row.background =
                Color::LinearGradient(LinearGradient::new(Argb8888::PURPLE, Argb8888::BLUE, 45.0))
                    .into();
            inner_row.spacing = 10.0;
            {
                for _ in 0..5 {
                    let mut empty = Rectangle::<WindowContext, Empty, NoID>::new();
                    empty.background = Argb8888::random().into();
                    inner_row.content_mut().push(Elements::Empty(empty));
                }
            }
            root.content_mut().push(Elements::Row(inner_row));
        }

        {
            let mut inner_row = Row::new();
            inner_row.background = Argb8888::WHITE.into();
            {
                for _ in 0..10 {
                    let mut empty = Rectangle::<WindowContext, Empty, NoID>::new();
                    empty.background = Argb8888::random().into();
                    inner_row.content_mut().push(Elements::Empty(empty));
                }
            }

            root.content_mut().push(Elements::Row(inner_row));
        }
        let mut timer = Timer::<WindowContext, CallbackImpls, NoID>::new();
        timer.interval = 0.01;
        timer.running = true;
        timer.repeat = true;
        root.content_mut().push(Elements::Timer(timer));

        self.0 = root;
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
    Row(Row<WindowContext, Elements, NoID>),
    Empty(Rectangle<WindowContext, Empty, NoID>),
    Timer(Timer<WindowContext, CallbackImpls, NoID>),
    Button(Button<WindowContext, Empty, CallbackImpls, StaticID>),
}

impl Default for Elements {
    fn default() -> Self {
        Self::Row(Row::default())
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
