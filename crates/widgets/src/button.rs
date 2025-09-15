use toolkit::{
    commands::{DrawRectCommand, DrawTextureCommand},
    glam::Vec2,
    types::{styling::BackgroundStyle, Argb8888, Corners, Rect, Stroke},
    widget::{DesiredSize, Widget},
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

#[derive(Default)]
pub struct Button<C: ButtonCallbacks, T: Widget> {
    pub size: Vec2,
    pub normal: ButtonStyle,
    pub hover: ButtonStyle,
    pub pressed: ButtonStyle,

    rect: Rect,
    content: T,
    callbacks: C,
    state: ButtonFsm,
    id: Option<String>,
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
                    corners: Corners::none(),
                },
            },
            hover: ButtonStyle {
                background: Argb8888::new(230, 230, 230, 255).into(),
                stroke: Stroke {
                    color: [Argb8888::BLUE; 4],
                    width: 1.0,
                    corners: Corners::none(),
                },
            },
            pressed: ButtonStyle {
                background: Argb8888::GRAY.into(),
                stroke: Stroke {
                    color: [Argb8888::DARK_GRAY; 4],
                    width: 1.0,
                    corners: Corners::none(),
                },
            },
            id,
            content: T::default(),
            callbacks: C::default(),
            state: ButtonFsm::Normal,
            rect: Rect::default(),
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
        self.content.layout(bounds);
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
