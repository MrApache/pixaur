use toolkit::{
    commands::{DrawRectCommand, DrawTextureCommand},
    glam::Vec2,
    types::{styling::BackgroundStyle, Argb8888, Bounds, Color, Stroke},
    widget::{
        Anchor, Context, DefaultID, DesiredSize, NoID, Sender, Spacing, StaticID, Widget, WidgetID,
    },
    WidgetQuery,
};

#[derive(Default, Debug, Clone, Copy)]
enum ButtonFsm {
    #[default]
    Normal,
    Hovered,
    Pressed,
    PressedOutside,
}

#[derive(Default, Clone)]
pub struct ButtonStyle {
    pub background: BackgroundStyle,
    pub stroke: Stroke,
}

#[derive(Default)]
pub struct ButtonMock;
impl<C: Context> ButtonCallbacks<C> for ButtonMock {}

#[allow(dead_code, unused_variables)]
pub trait ButtonCallbacks<C: Context>: Default + Send + Sync + 'static {
    fn on_enter(&self, sender: &mut Sender<C>) {}
    fn on_exit(&self, sender: &mut Sender<C>) {}
    fn on_press(&self, sender: &mut Sender<C>) {}
    fn on_clicked(&self, sender: &mut Sender<C>) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Alignment {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(WidgetQuery)]
pub struct Button<C, W, CB = ButtonMock, ID = DefaultID>
where
    C: Context,
    W: Widget<C>,
    CB: ButtonCallbacks<C>,
    ID: WidgetID,
{
    pub size: Vec2,
    pub normal: ButtonStyle,
    pub hover: ButtonStyle,
    pub pressed: ButtonStyle,
    pub alignment: Alignment,
    pub padding: Spacing,
    pub anchor: Anchor,

    rect: Bounds,
    state: ButtonFsm,

    callbacks: CB,
    id: ID::IdType,
    #[content]
    content: W,
    _phantom: std::marker::PhantomData<C>,
}

impl<C, W, CB> Button<C, W, CB, DefaultID>
where
    C: Context,
    W: Widget<C>,
    CB: ButtonCallbacks<C>,
{
    #[must_use]
    pub fn new_default() -> Self {
        Self::new_with_id(None)
    }

    #[must_use]
    pub fn new_id(id: impl Into<String>) -> Self {
        Self::new_with_id(Some(id.into()))
    }
}

impl<C, W, CB> Button<C, W, CB, StaticID>
where
    C: Context,
    W: Widget<C>,
    CB: ButtonCallbacks<C>,
{
    #[must_use]
    pub fn new_static(id: &'static str) -> Self {
        Self::new_with_id(id)
    }
}

impl<C, W, CB> Button<C, W, CB, NoID>
where
    C: Context,
    W: Widget<C>,
    CB: ButtonCallbacks<C>,
{
    #[must_use]
    pub fn new() -> Self {
        Self::new_with_id(())
    }
}

impl<C, W, CB, ID> Default for Button<C, W, CB, ID>
where
    C: Context,
    W: Widget<C>,
    CB: ButtonCallbacks<C>,
    ID: WidgetID,
{
    fn default() -> Self {
        Self::new_with_id(ID::IdType::default())
    }
}

impl<C, W, CB, ID> Button<C, W, CB, ID>
where
    C: Context,
    W: Widget<C>,
    CB: ButtonCallbacks<C>,
    ID: WidgetID,
{
    fn new_with_id(id: ID::IdType) -> Self {
        Self {
            size: Vec2::new(30.0, 30.0),
            normal: ButtonStyle {
                background: BackgroundStyle::Color(Color::Simple(Argb8888::LIGHT_GRAY)),
                stroke: Stroke {
                    color: [Argb8888::DARK_GRAY; 4],
                    width: 1.0,
                },
            },
            hover: ButtonStyle {
                background: BackgroundStyle::Color(Color::Simple(Argb8888::new(
                    230, 230, 230, 255,
                ))),
                stroke: Stroke {
                    color: [Argb8888::BLUE; 4],
                    width: 1.0,
                },
            },
            pressed: ButtonStyle {
                background: BackgroundStyle::Color(Color::Simple(Argb8888::GRAY)),
                stroke: Stroke {
                    color: [Argb8888::DARK_GRAY; 4],
                    width: 1.0,
                },
            },
            id,
            content: W::default(),
            callbacks: CB::default(),
            rect: Bounds::ZERO,
            state: ButtonFsm::Normal,
            alignment: Alignment::Center,
            anchor: Anchor::Left,
            padding: Spacing {
                left: 2.0,
                right: 2.0,
                top: 2.0,
                bottom: 2.0,
            },
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn content_mut(&mut self) -> &mut W {
        &mut self.content
    }

    pub fn content(&self) -> &W {
        &self.content
    }
}

impl<C, W, CB, ID> Widget<C> for Button<C, W, CB, ID>
where
    C: Context,
    W: Widget<C>,
    CB: ButtonCallbacks<C>,
    ID: WidgetID,
{
    fn anchor(&self) -> Anchor {
        self.anchor
    }

    fn desired_size(&self) -> toolkit::widget::DesiredSize {
        DesiredSize::Exact(self.size)
    }

    fn draw<'frame>(&'frame self, out: &mut toolkit::commands::CommandBuffer<'frame>) {
        let style = match self.state {
            ButtonFsm::Normal => &self.normal,
            ButtonFsm::Hovered => &self.hover,
            ButtonFsm::Pressed | ButtonFsm::PressedOutside => &self.pressed,
        };
        match &style.background {
            BackgroundStyle::Color(color) => out.push(DrawRectCommand::new(
                self.rect.clone(),
                color.clone(),
                style.stroke.clone(),
            )),
            BackgroundStyle::Texture(texture) => out.push(DrawTextureCommand::new(
                self.rect.clone(),
                texture.clone(),
                style.stroke.clone(),
            )),
        }
        self.content.draw(out);
    }

    fn layout(&mut self, bounds: toolkit::types::Bounds) {
        self.rect = bounds.clone();

        let content_size = match self.content.desired_size() {
            DesiredSize::Exact(min) => Vec2::new(
                min.x
                    .min(self.size.x - self.padding.left - self.padding.right),
                min.y
                    .min(self.size.y - self.padding.top - self.padding.bottom),
            ),
            DesiredSize::ExactY(y) => Vec2::new(
                self.size.x - self.padding.left - self.padding.right,
                y.max(self.size.y - self.padding.top - self.padding.bottom),
            ),
            DesiredSize::ExactX(x) => Vec2::new(
                x.max(self.size.x - self.padding.right - self.padding.left),
                self.size.y - self.padding.top - self.padding.bottom,
            ),
            DesiredSize::Fill => Vec2::new(
                self.size.x - self.padding.left - self.padding.right,
                self.size.y - self.padding.top - self.padding.bottom,
            ),
            DesiredSize::Ignore => return,
        };

        let content_x = match self.alignment {
            Alignment::TopLeft | Alignment::CenterLeft | Alignment::BottomLeft => {
                self.rect.position.x + self.padding.left
            }
            Alignment::TopCenter | Alignment::Center | Alignment::BottomCenter => {
                self.rect.position.x + (self.size.x - content_size.x) / 2.0
            }
            Alignment::TopRight | Alignment::CenterRight | Alignment::BottomRight => {
                self.rect.position.x + self.size.x - content_size.x - self.padding.right
            }
        };

        let content_y = match self.alignment {
            Alignment::TopLeft | Alignment::TopCenter | Alignment::TopRight => {
                self.rect.position.y + self.padding.top
            }
            Alignment::CenterLeft | Alignment::Center | Alignment::CenterRight => {
                self.rect.position.y + (self.size.y - content_size.y) / 2.0
            }
            Alignment::BottomLeft | Alignment::BottomCenter | Alignment::BottomRight => {
                self.rect.position.y + self.size.y - content_size.y - self.padding.bottom
            }
        };

        let content_rect = Bounds {
            position: Vec2::new(content_x, content_y),
            size: Vec2::new(content_size.x, content_size.y),
        };

        self.content.layout(content_rect);
    }

    fn update(&mut self, ctx: &toolkit::widget::FrameContext, sender: &mut Sender<C>) {
        let is_inside = self.rect.contains(ctx.position());
        let is_pressed = ctx.buttons().left();
        match self.state {
            ButtonFsm::Normal => {
                if is_inside {
                    self.state = ButtonFsm::Hovered;
                    self.callbacks.on_enter(sender);
                }
            }

            ButtonFsm::Hovered => {
                if !is_inside {
                    self.state = ButtonFsm::Normal;
                    self.callbacks.on_exit(sender);
                } else if is_pressed {
                    self.state = ButtonFsm::Pressed;
                    self.callbacks.on_press(sender);
                }
            }

            ButtonFsm::Pressed => {
                if !is_pressed {
                    self.state = ButtonFsm::Hovered;
                    self.callbacks.on_clicked(sender);
                } else if !is_inside {
                    self.state = ButtonFsm::PressedOutside;
                }
            }

            ButtonFsm::PressedOutside => {
                if !is_pressed {
                    self.state = ButtonFsm::Normal;
                    self.callbacks.on_clicked(sender);
                    self.callbacks.on_exit(sender);
                }
            }
        }

        self.content.update(ctx, sender);
    }
}
