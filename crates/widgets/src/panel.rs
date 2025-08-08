use toolkit::{
    glam::Vec2,
    widget::{
        Container, Rect, Size, Spacing, Widget
    },
    Argb8888,
    Color
};

pub struct Panel {
    id: String,
    pub background: Color,
}

impl Panel {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            background: Color::Simple(Argb8888::BLACK),
        }
    }
}

impl Widget for Panel {
    fn id(&self) -> &str {
        &self.id
    }

    fn desired_size(&self) -> Size {
        Size::default()
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
            rect: Rect::new(Vec2::new(100.0, 100.0), Vec2::new(300.0, 300.0)),
            color: self.background.clone()
        });
    }
}

impl Container for Panel {
    fn add_child(&mut self, child: Box<dyn Widget>) {
    }

    fn layout(&mut self, bounds: Rect) {
    }

    fn children(&self) -> &[Box<dyn Widget>] {
        &[]
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut []
    }
}
