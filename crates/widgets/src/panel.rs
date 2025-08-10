use toolkit::{
    glam::{Vec2, Vec4},
    widget::{
        Container, Rect, Spacing, Widget
    },
    Argb8888,
    Color
};

pub struct Panel {
    id: String,
    content: Vec<Box<dyn Widget>>,
    rect: Rect,
    pub background: Color,
    pub padding: Vec4,  // (left, top, right, bottom)
    pub spacing: f32,
}

impl Panel {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            content: vec![],
            rect: Rect::default(),
            background: Color::Simple(Argb8888::WHITE),
            padding: Vec4::new(2.0, 2.0, 2.0, 2.0),
            spacing: 2.0,
        }
    }
}

impl Widget for Panel {
    fn id(&self) -> &str {
        &self.id
    }

    fn desired_size(&self) -> Vec2 {
        Vec2::MAX
    }

    fn spacing(&self) -> Spacing {
        Spacing::default()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw<'frame>(&'frame self, out: &mut toolkit::CommandBuffer<'frame>) {
        out.push(toolkit::DrawCommand::Rect {
            rect: self.rect.clone(),
            color: self.background.clone()
        });
        self.content.iter().for_each(|w| {
            w.draw(out);
        });
    }

    fn layout(&mut self, bounds: Rect) {
        self.rect = bounds;

        // Вычисляем внутренние границы с учётом padding
        let inner_min_x = self.rect.min.x + self.padding.x; // left
        let inner_min_y = self.rect.min.y + self.padding.y; // top
        let inner_max_x = self.rect.max.x - self.padding.z - self.padding.x; // right
        let inner_max_y = self.rect.max.y - self.padding.w - self.padding.y; // bottom

        let mut cursor_x = inner_min_x;
        let base_y = inner_min_y;
        let max_x = inner_max_x;
        let max_y = inner_max_y;

        let spacing = self.spacing;
        let len = self.content.len();

        for (i, child) in self.content.iter_mut().enumerate() {
            let desired = child.desired_size();

            let available_width = max_x - cursor_x;
            let available_height = max_y - base_y;

            let width = desired.x.min(available_width);
            let height = desired.y.min(available_height);

            let child_bounds = Rect {
                min: Vec2::new(cursor_x, base_y),
                max: Vec2::new(cursor_x + width, base_y + height),
            };

            child.layout(child_bounds);

            cursor_x += width;

            // Добавляем spacing после элемента, кроме последнего
            if i != len - 1 {
                cursor_x += spacing;
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
