use toolkit::{
    app::App,
    include_asset,
    types::{Argb8888, Color, LinearGradient, Texture},
    widget::{Callbacks, Container, Context, WidgetQuery},
    window::WindowRequest,
    ContentManager, DesktopOptions, Handle, WidgetEnum, WindowRoot,
};
use widgets::{
    button::{Button, ButtonCallbacks},
    impl_empty_widget, impl_proxy_widget,
    panel::{HorizontalAlign, Panel},
};

impl_empty_widget!(Empty);

pub enum WindowContext {}
impl Context for WindowContext {
    type Widget = Root;
    type WindowRoot = Root;
    fn execute(&self, _: &mut ContentManager, _: &mut toolkit::widget::Tree<Self>) {}
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
                let button: Button<WindowContext, Empty, CallbackImpls> = Button::new();
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
    Button(Button<WindowContext, Empty, CallbackImpls>),
}

impl Default for Elements {
    fn default() -> Self {
        Self::Panel(Panel::default())
    }
}

impl WidgetQuery<WindowContext> for Elements {
    fn get_element<QW: toolkit::widget::Widget<WindowContext>>(&self, id: &str) -> Option<&QW> {
        match self {
            Elements::Button(button) => button.get_element(id),
            Elements::Panel(panel) => panel.get_element(id),
            Elements::Empty(panel) => panel.get_element(id),
        }
    }

    fn get_mut_element<QW: toolkit::widget::Widget<WindowContext>>(
        &mut self,
        id: &str,
    ) -> Option<&mut QW> {
        match self {
            Elements::Button(button) => button.get_mut_element(id),
            Elements::Panel(panel) => panel.get_mut_element(id),
            Elements::Empty(panel) => panel.get_mut_element(id),
        }
    }
}

#[derive(Default)]
pub struct CallbackImpls;
impl Callbacks for CallbackImpls {}
impl ButtonCallbacks<WindowContext> for CallbackImpls {}
