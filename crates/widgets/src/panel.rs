use toolkit::{
    commands::{CommandBuffer, DrawCommand, DrawRectCommand, DrawTextureCommand},
    glam::{Vec2, Vec4},
    types::styling::*,
    types::*,
    widget::{Container, DesiredSize, Widget},
};

#[derive(Copy, Clone, Debug, Default)]
pub enum LayoutMode {
    #[default]
    Vertical,
    Horizontal,
}

pub struct Panel {
    id: String,
    rect: Rect,
    content: Vec<Box<dyn Widget>>,
    pub padding: Vec4,
    pub spacing: f32,
    pub mode: LayoutMode,

    pub background: BackgroundStyle,
    pub stroke: Stroke,
}

impl Panel {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            content: vec![],
            rect: Rect::default(),
            background: Color::Simple(Argb8888::WHITE).into(),
            padding: Vec4::new(2.0, 2.0, 2.0, 2.0),
            spacing: 2.0,
            mode: LayoutMode::Vertical,
            stroke: Stroke::default(),
        }
    }
}

impl Widget for Panel {
    fn id(&self) -> &str {
        &self.id
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
            BackgroundStyle::Color(color) => {
                out.push(DrawRectCommand::new(self.rect.clone(), color.clone()))
            }
            BackgroundStyle::Texture(texture) => {
                out.push(DrawTextureCommand::new(self.rect.clone(), texture.clone()))
            }
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

        let mut cursor_x = min_x;
        let available_height = max_y - min_y;

        let len = self.content.len();

        // 1. Считаем суммарную ширину Min-виджетов и количество Fill-виджетов
        let mut total_min_width = 0.0;
        let mut fill_count = 0;

        self.content
            .iter()
            .for_each(|widget| match widget.desired_size() {
                DesiredSize::Min(size) => total_min_width += size.x,
                DesiredSize::Fill => fill_count += 1,
            });

        // Общая ширина, занятная spacing (между элементами, их len-1)
        let total_spacing = self.spacing * len.saturating_sub(1) as f32;
        // Вычисляем доступное пространство для Fill-виджетов, учитывая padding и spacing
        let total_available_width = max_x - total_spacing - total_min_width - self.padding.z;
        let fill_width = total_available_width / fill_count as f32;

        // 2. Расставляем дочерние элементы по горизонтали с учётом spacing и fill_width
        for (i, child) in self.content.iter_mut().enumerate() {
            let (width, height) = match child.desired_size() {
                DesiredSize::Min(vec2) => (vec2.x, vec2.y.min(available_height)),
                DesiredSize::Fill => (fill_width, available_height),
            };

            let child_bounds = Rect {
                min: Vec2::new(cursor_x, min_y),
                max: Vec2::new(width, height),
            };

            child.layout(child_bounds);

            cursor_x += width;

            // Добавляем spacing после элемента, кроме последнего
            if i != len - 1 {
                cursor_x += self.spacing;
            }

            // Если вышли за границы — прекращаем
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
}

impl Widget for TestPanelLayoutWidget {
    fn id(&self) -> &str {
        "test_widget"
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
        )));
    }

    fn layout(&mut self, bounds: Rect) {
        self.rect = bounds;
    }
}
