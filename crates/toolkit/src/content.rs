use std::collections::HashMap;
use ab_glyph::FontRef;
use ttf_parser::Face;

#[derive(Default)]
pub struct Content {
    static_font: HashMap<String, FontRef<'static>>,
    //dynamic_font: HashMap<String, FontRef<'static>>,
}

impl Content {
    pub fn include_font(&mut self, bytes: &'static [u8]) {
        let font_name = font_name(bytes).unwrap();
        let font = FontRef::try_from_slice(bytes).unwrap();
        self.static_font.insert(font_name, font);
    }

    pub fn load_font(&mut self, path: &'static str) {
        let bytes: &'static [u8] = Box::leak(std::fs::read(path).unwrap().into_boxed_slice());
        let font_name = font_name(bytes).unwrap();
        let font = FontRef::try_from_slice(bytes).unwrap();
        self.static_font.insert(font_name, font);
    }

    pub(crate) fn get_font(&self, font: &str) -> &FontRef<'_> {
        self.static_font.get(font).unwrap()
    }
}

fn font_name(data: &[u8]) -> Option<String> {
    let face = Face::parse(data, 0).ok()?;
    face.names()
        .into_iter()
        .find(|name| name.name_id == ttf_parser::name_id::FULL_NAME)
        .and_then(|name| name.to_string())
}
