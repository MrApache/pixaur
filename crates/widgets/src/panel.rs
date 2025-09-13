use toolkit::{
    commands::{CommandBuffer, DrawCommand, DrawRectCommand, DrawTextureCommand},
    glam::{Vec2, Vec4},
    types::styling::BackgroundStyle,
    types::{Argb8888, Color, Rect, Stroke},
    widget::{Container, DesiredSize, Widget},
};

#[derive(Copy, Clone, Debug, Default)]
pub enum LayoutMode {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum HorizontalAlign {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum VerticalAlign {
    #[default]
    Start,
    Center,
    End,
}

pub struct Panel {
    id: Option<String>,
    rect: Rect,
    content: Vec<Box<dyn Widget>>,
    pub padding: Vec4,
    pub spacing: f32,
    pub mode: LayoutMode,
    pub vertical_align: VerticalAlign,
    pub horizontal_align: HorizontalAlign,

    pub background: BackgroundStyle,
    pub stroke: Stroke,
}

impl Default for Panel {
    fn default() -> Self {
        Self::new()
    }
}

impl Panel {
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: None,
            content: vec![],
            rect: Rect::default(),
            background: Color::Simple(Argb8888::WHITE).into(),
            padding: Vec4::new(4.0, 4.0, 4.0, 4.0),
            spacing: 4.0,
            mode: LayoutMode::Vertical,
            stroke: Stroke::default(),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Center,
        }
    }

    pub fn with_id(id: impl Into<String>) -> Self {
        Self {
            id: Some(id.into()),
            content: vec![],
            rect: Rect::default(),
            background: Color::Simple(Argb8888::WHITE).into(),
            padding: Vec4::new(4.0, 4.0, 4.0, 4.0),
            spacing: 4.0,
            mode: LayoutMode::Vertical,
            stroke: Stroke::default(),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Center,
        }
    }
}

impl Widget for Panel {
    fn id(&self) -> Option<&str> {
        if let Some(id) = &self.id {
            Some(id)
        } else {
            None
        }
    }

    fn desired_size(&self) -> DesiredSize {
        DesiredSize::Fill
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>) {
        match &self.background {
            BackgroundStyle::Color(color) => out.push(DrawRectCommand::new(
                self.rect.clone(),
                color.clone(),
                self.stroke.clone(),
            )),
            BackgroundStyle::Texture(texture) => out.push(DrawTextureCommand::new(
                self.rect.clone(),
                texture.clone(),
                self.stroke.clone(),
            )),
        }

        self.content.iter().for_each(|w| {
            w.draw(out);
        });
    }

    fn layout(&mut self, bounds: Rect) {
        self.rect = bounds;

        // Учитываем padding с обеих сторон для вычисления внутренних границ
        let min_x = self.rect.min.x + self.padding.x; // left
        let min_y = self.rect.min.y + self.padding.y; // bottom
        let max_x = self.rect.max.x - self.padding.z; // right
        let max_y = self.rect.max.y - self.padding.w; // top

        let len = self.content.len();
        let available_width = max_x;
        let available_height = max_y - min_y;

        let mut cursor_x = match self.horizontal_align {
            HorizontalAlign::Start => min_x,
            HorizontalAlign::Center => min_x + available_width / 2.0,
            HorizontalAlign::End => max_x - available_width,
        };

        let mut total_min_width = 0.0;
        let mut fill_count = 0;

        self.content
            .iter()
            .for_each(|widget| match widget.desired_size() {
                DesiredSize::Min(size) => total_min_width += size.x,
                DesiredSize::Fill => fill_count += 1,
                DesiredSize::FillMinY(_) => fill_count += 1,
            });

        let total_spacing = self.spacing * len.saturating_sub(1) as f32;
        let total_available_width = max_x - total_spacing - total_min_width - self.padding.z;
        let fill_width = total_available_width / fill_count as f32;

        for (i, child) in self.content.iter_mut().enumerate() {
            let (width, height) = match child.desired_size() {
                DesiredSize::Min(vec2) => (vec2.x, vec2.y.min(available_height)),
                DesiredSize::Fill => (fill_width, available_height),
                DesiredSize::FillMinY(y) => (fill_width, y.min(available_height)),
            };

            let offset_y = match self.vertical_align {
                VerticalAlign::Start => 0.0,
                VerticalAlign::Center => (available_height - height) / 2.0,
                VerticalAlign::End => available_height - height,
            };

            let offset_x = match self.horizontal_align {
                HorizontalAlign::Start => 0.0,
                HorizontalAlign::Center => width - (width / 2.0),
                HorizontalAlign::End => 0.0,
            };

            let child_bounds = Rect {
                min: Vec2::new(cursor_x - offset_x, min_y + offset_y),
                max: Vec2::new(width, height),
            };

            //println!("Offset: {offset_x}x{offset_y}");

            child.layout(child_bounds);

            cursor_x += width;

            if i != len - 1 {
                cursor_x += self.spacing;
            }

            if cursor_x >= max_x {
                break;
            }
        }
    }
}

impl Container for Panel {
    fn add_child(&mut self, child: Box<dyn Widget>) {
        self.content.push(child);
    }

    fn children(&self) -> &[Box<dyn Widget>] {
        &self.content
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut self.content
    }
}

#[derive(Default)]
pub struct TestPanelLayoutWidget {
    pub min: Vec2,
    rect: Rect,
    pub stroke: Stroke,
}

impl Widget for TestPanelLayoutWidget {
    fn id(&self) -> Option<&str> {
        Some("test_widget")
    }

    fn desired_size(&self) -> DesiredSize {
        DesiredSize::Min(self.min)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>) {
        out.push(DrawCommand::Rect(DrawRectCommand::new(
            self.rect.clone(),
            Argb8888::CYAN,
            self.stroke.clone(),
        )));
    }

    fn layout(&mut self, bounds: Rect) {
        self.rect = bounds;
    }
}
