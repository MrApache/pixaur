use fontdue::{Font, FontSettings};
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use ttf_parser::Face;

use crate::{
    Error,
    rendering::{Gpu, material::Material},
};

#[macro_export]
macro_rules! include_asset {
    ($path:expr) => {
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $path))
    };
}

#[macro_export]
macro_rules! include_asset_content {
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $path))
    };
}

static DEFAULT_FONT: Lazy<Arc<Font>> = Lazy::new(|| {
    let bytes = include_asset!("Ubuntu-Regular.ttf");
    let font = Font::from_bytes(bytes.as_ref(), FontSettings::default()).unwrap();
    Arc::new(font)
});

#[derive(Clone, Debug)]
pub struct FontHandle {
    pub(crate) inner: Arc<Font>,
}

impl Default for FontHandle {
    fn default() -> Self {
        Self {
            inner: DEFAULT_FONT.clone(),
        }
    }
}

impl AsRef<Font> for FontHandle {
    fn as_ref(&self) -> &Font {
        self.inner.as_ref()
    }
}

static HANDLE_ID: AtomicUsize = AtomicUsize::new(0);
fn next_handle_id() -> usize {
    HANDLE_ID.fetch_add(1, Ordering::SeqCst)
}

#[derive(Default, Debug, Clone, Copy)]
pub struct TextureHandle {
    id: usize,
}

#[derive(Default)]
pub struct ContentManager {
    static_font: HashMap<String, Arc<Font>>,
    static_textures: Vec<Material>,

    queue: Vec<TextureRequest>,
}

pub(crate) struct TextureRequest {
    is_static: bool,
    handle_id: usize,
    bytes: &'static [u8],
}

impl ContentManager {
    pub fn include_font(&mut self, bytes: &'static [u8]) -> FontHandle {
        let font_name = font_name(bytes).unwrap();
        let font = Font::from_bytes(bytes, FontSettings::default()).unwrap();
        let font_handle = Arc::new(font);
        self.static_font.insert(font_name, font_handle.clone());
        FontHandle { inner: font_handle }
    }

    pub fn static_load_font(&mut self, path: &'static str) -> FontHandle {
        let bytes: &'static [u8] = Box::leak(std::fs::read(path).unwrap().into_boxed_slice());
        let font_name = font_name(bytes).unwrap();
        let font = Font::from_bytes(bytes, FontSettings::default()).unwrap();
        let font_handle = Arc::new(font);
        self.static_font.insert(font_name, font_handle.clone());
        FontHandle { inner: font_handle }
    }

    pub(crate) fn get_font(&self, font: &str) -> &Font {
        self.static_font.get(font).unwrap()
    }

    pub fn include_texture(&mut self, bytes: &'static [u8]) -> TextureHandle {
        let handle_id = next_handle_id();
        self.queue.push(TextureRequest {
            bytes,
            handle_id,
            is_static: true,
        });

        TextureHandle { id: handle_id }
    }

    pub fn static_load_texture(&mut self, path: &str) -> Result<TextureHandle, Error> {
        let request = TextureRequest {
            handle_id: next_handle_id(),
            bytes: Box::leak(load_asset(path)?.into_boxed_slice()),
            is_static: true,
        };

        let result = Ok(TextureHandle {
            id: request.handle_id,
        });

        self.queue.push(request);
        result
    }

    pub(crate) fn dispath_queue(&mut self, gpu: &Gpu) -> Result<(), Error> {
        self.queue
            .drain(..)
            .try_for_each(|request| -> Result<(), Error> {
                let material = Material::from_bytes(request.bytes, &gpu.device, &gpu.queue)?;
                if request.is_static {
                    self.static_textures.push(material);
                }
                Ok(())
            })
    }

    pub(crate) fn get_texture(&self, handle: TextureHandle) -> &Material {
        self.static_textures.get(handle.id).unwrap()
    }
}

fn font_name(data: &[u8]) -> Option<String> {
    let face = Face::parse(data, 0).ok()?;
    face.names()
        .into_iter()
        .find(|name| name.name_id == ttf_parser::name_id::FULL_NAME)
        .and_then(|name| name.to_string())
}

pub fn load_asset(path: &str) -> Result<Vec<u8>, Error> {
    let asset_path = get_asset_path().join(path);
    Ok(fs::read(asset_path)?)
}

pub fn load_asset_str(path: &str) -> Result<String, Error> {
    let asset_path = get_asset_path().join(path);
    Ok(fs::read_to_string(asset_path)?)
}

fn get_asset_path() -> PathBuf {
    // В debug используем путь относительно корня проекта
    #[cfg(debug_assertions)]
    {
        use std::path::Path;

        Path::new(env!("CARGO_MANIFEST_DIR")).join("assets")
    }

    // В release — рядом с бинарником
    #[cfg(not(debug_assertions))]
    {
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("assets")
    }
}
