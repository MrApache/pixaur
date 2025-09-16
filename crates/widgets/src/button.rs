use toolkit::{
    commands::{DrawRectCommand, DrawTextureCommand},
    glam::Vec2,
    types::{styling::BackgroundStyle, Argb8888, Corners, Rect, Stroke},
    widget::{DesiredSize, Padding, Widget},
};

#[derive(Default, Debug, Clone, Copy)]
enum ButtonFsm {
    #[default]
    Normal,
    Hovered,
    Pressed,
    PressedOutside,
}

pub trait ButtonCallbacks: Default + Send + Sync + 'static {
    fn on_enter(&self) {}
    fn on_exit(&self) {}
    fn on_press(&self) {}
    fn on_clicked(&self) {}
}

#[derive(Default, Clone)]
pub struct ButtonStyle {
    pub background: BackgroundStyle,
    pub stroke: Stroke,
}

#[derive(Default)]
pub struct ButtonMockCallbacks;
impl ButtonCallbacks for ButtonMockCallbacks {}

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

#[derive(Default)]
pub struct Button<C: ButtonCallbacks, T: Widget> {
    pub size: Vec2,
    pub normal: ButtonStyle,
    pub hover: ButtonStyle,
    pub pressed: ButtonStyle,
    pub alignment: Alignment,
    pub padding: Padding,

    rect: Rect,
    id: Option<String>,
    state: ButtonFsm,

    content: T,
    callbacks: C,
}

impl<C: ButtonCallbacks, T: Widget> Button<C, T> {
    #[must_use]
    pub fn new() -> Self {
        Self::new_with_id(None)
    }

    pub fn with_id(id: impl Into<String>) -> Self {
        Self::new_with_id(Some(id.into()))
    }

    fn new_with_id(id: Option<String>) -> Self {
        Self {
            size: Vec2::new(30.0, 30.0),
            normal: ButtonStyle {
                background: Argb8888::LIGHT_GRAY.into(),
                stroke: Stroke {
                    color: [Argb8888::DARK_GRAY; 4],
                    width: 1.0,
                    corners: Corners::NONE,
                },
            },
            hover: ButtonStyle {
                background: Argb8888::new(230, 230, 230, 255).into(),
                stroke: Stroke {
                    color: [Argb8888::BLUE; 4],
                    width: 1.0,
                    corners: Corners::NONE,
                },
            },
            pressed: ButtonStyle {
                background: Argb8888::GRAY.into(),
                stroke: Stroke {
                    color: [Argb8888::DARK_GRAY; 4],
                    width: 1.0,
                    corners: Corners::NONE,
                },
            },
            id,
            content: T::default(),
            callbacks: C::default(),
            rect: Rect::ZERO,
            state: ButtonFsm::Normal,
            alignment: Alignment::Center,
            padding: Padding {
                left: 2.0,
                right: 2.0,
                top: 2.0,
                bottom: 2.0,
            },
        }
    }

    pub fn content_mut(&mut self) -> &mut T {
        &mut self.content
    }
}

impl<C: ButtonCallbacks, T: Widget> Widget for Button<C, T> {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn desired_size(&self) -> toolkit::widget::DesiredSize {
        DesiredSize::Min(self.size)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
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

    fn layout(&mut self, bounds: toolkit::types::Rect) {
        self.rect = bounds.clone();

        let content_size = match self.content.desired_size() {
            DesiredSize::Min(min) => Vec2::new(
                min.x
                    .min(self.size.x - self.padding.left - self.padding.right),
                min.y
                    .min(self.size.y - self.padding.top - self.padding.bottom),
            ),
            DesiredSize::FillMinY(y) => Vec2::new(
                self.size.x - self.padding.left - self.padding.right,
                y.max(self.size.y - self.padding.top - self.padding.bottom),
            ),
            DesiredSize::Fill => Vec2::new(
                self.size.x - self.padding.left - self.padding.right,
                self.size.y - self.padding.top - self.padding.bottom,
            ),
        };

        let content_x = match self.alignment {
            Alignment::TopLeft | Alignment::CenterLeft | Alignment::BottomLeft => {
                self.rect.min.x + self.padding.left
            }
            Alignment::TopCenter | Alignment::Center | Alignment::BottomCenter => {
                self.rect.min.x + (self.size.x - content_size.x) / 2.0
            }
            Alignment::TopRight | Alignment::CenterRight | Alignment::BottomRight => {
                self.rect.min.x + self.size.x - content_size.x - self.padding.right
            }
        };

        let content_y = match self.alignment {
            Alignment::TopLeft | Alignment::TopCenter | Alignment::TopRight => {
                self.rect.min.y + self.padding.top
            }
            Alignment::CenterLeft | Alignment::Center | Alignment::CenterRight => {
                self.rect.min.y + (self.size.y - content_size.y) / 2.0
            }
            Alignment::BottomLeft | Alignment::BottomCenter | Alignment::BottomRight => {
                self.rect.min.y + self.size.y - content_size.y - self.padding.bottom
            }
        };

        let content_rect = Rect {
            min: Vec2::new(content_x, content_y),
            max: Vec2::new(content_size.x, content_size.y),
        };

        self.content.layout(content_rect);
    }

    fn update(&mut self, ctx: &toolkit::widget::FrameContext) {
        let is_inside = self.rect.contains(ctx.position());
        let is_pressed = ctx.buttons().left();
        match self.state {
            ButtonFsm::Normal => {
                if is_inside {
                    self.state = ButtonFsm::Hovered;
                    self.callbacks.on_enter();
                }
            }

            ButtonFsm::Hovered => {
                if !is_inside {
                    self.state = ButtonFsm::Normal;
                    self.callbacks.on_exit();
                } else if is_pressed {
                    self.state = ButtonFsm::Pressed;
                    self.callbacks.on_press();
                }
            }

            ButtonFsm::Pressed => {
                if !is_pressed {
                    self.state = ButtonFsm::Hovered;
                    self.callbacks.on_clicked();
                } else if !is_inside {
                    self.state = ButtonFsm::PressedOutside;
                }
            }

            ButtonFsm::PressedOutside => {
                if !is_pressed {
                    self.state = ButtonFsm::Normal;
                    self.callbacks.on_clicked();
                    self.callbacks.on_exit();
                }
            }
        }

        self.content.update(ctx);
    }
}
