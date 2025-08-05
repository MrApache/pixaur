mod error;
mod renderer;
mod content;
mod color;
pub mod widget;
pub mod window;

pub use color::*;
pub use error::*;
pub use wl_client::Anchor;

use crate::{
    content::Content,
    renderer::GPU,
    widget::{Container, Rect},
    window::{Window, WindowRequest}
};

use ab_glyph::{point, Font};
use std::{
    ffi::c_void,
    ptr::NonNull,
    sync::Arc
};

use wayland_client::{Connection, EventQueue, Proxy};
use wl_client::{Layer, WlClient};

pub const DEFAULT_FONT: &str = "Ubuntu Regular";

#[derive(Debug, Clone)]
pub enum DrawCommand<'a> {
    Rect {
        rect: Rect,
        color: Color,
    },
    Text {
        size: f32,
        font: &'a str,
        content: &'a str,
        color: Color,
    },
}

#[derive(Default)]
pub struct CommandBuffer<'frame> {
    pub storage: Vec<DrawCommand<'frame>>,
}

impl<'frame> CommandBuffer<'frame> {
    pub fn push(&mut self, cmd: DrawCommand<'frame>) {
        self.storage.push(cmd);
    }

    pub fn extend<I: IntoIterator<Item = DrawCommand<'frame>>>(&mut self, iter: I) {
        iter.into_iter().for_each(|cmd| self.push(cmd));
    }

    pub(crate) fn pop(&mut self) -> Option<DrawCommand<'frame>> {
        self.storage.pop()
    }
}

#[allow(unused)]
pub trait GUI {
    fn load_content(&self, content: &mut Content) {}
    fn setup_windows(&mut self) -> Vec<Box<dyn UserWindow<Self>>>;
}

pub struct EventLoop<T: GUI> {
    gui: T,
    content: Content,

    client: WlClient,
    event_queue: EventQueue<WlClient>,
    display_ptr: NonNull<c_void>,

    gpu: GPU,
}

impl<T: GUI> EventLoop<T> {
    pub fn new(app: T) -> Result<Self, Error> {

        let conn = Connection::connect_to_env()
            .expect("Failed to connect to the Wayland server.");

        let display = conn.display();
        let mut event_queue = conn.new_event_queue();
        let qh = event_queue.handle();

        let _registry = display.get_registry(&qh, Arc::new("".to_string()));

        let mut client = WlClient::default();

        event_queue.roundtrip(&mut client).unwrap(); //Register objects
        event_queue.roundtrip(&mut client).unwrap(); //Register outputs

        let mut content = Content::default();
        content.include_font(include_bytes!("../../../assets/Ubuntu-Regular.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-Light.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-LightItalic.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-Bold.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-BoldItalic.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-Italic.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-Medium.ttf"));
        content.include_font(include_bytes!("../../../assets/Ubuntu-MediumItalic.ttf"));

        let gpu = GPU::new()?;

        Ok(Self {
            gui: app,
            content,

            client,
            event_queue,
            display_ptr: NonNull::new(display.id().as_ptr() as *mut c_void).unwrap(),

            gpu
        })
    }

    pub fn run(&mut self) -> Result<(), Error>{
        self.gui.load_content(&mut self.content);
        let mut windows = self.init_windows_backends()?;
        let mut counter = FpsCounter::new(144);

        loop {
            let fps = counter.tick();
            println!("FPS: {fps:.1}");

            windows.iter_mut().for_each(|window| {
                let mut cm_buffer = CommandBuffer::default();
                window.graphics.update(&mut self.gui, &mut window.window);

                let mut window_handle = window.window.handle.lock().unwrap();
                window_handle.frame(&self.event_queue.handle());
                if !window_handle.can_draw() {
                    return;
                }

                window_handle.clear();

                window.window.root.draw(&mut cm_buffer);
                while let Some(command) = cm_buffer.pop() {
                    //#[allow(unused)]
                    match command {
                        DrawCommand::Rect { rect, color } => todo!(),
                        DrawCommand::Text { size, font, content, color } => {
                            let font = self.content.get_font(font);
                            
                            let units_per_em = font.units_per_em().unwrap();
                            let scale = size / units_per_em;
                            
                            let ascent = font.ascent_unscaled();
                            let base_y = ascent * scale; // <-- вот это основное изменение
                            
                            let mut pen_x = 0.0;
                            
                            for ch in content.chars() {
                                let glyph = font.glyph_id(ch).with_scale_and_position(size, point(pen_x, base_y));
                                let pos_x = glyph.position.x;
                                let pos_y = glyph.position.y;
                                let h_advance = font.h_advance_unscaled(glyph.id);
                                
                                if let Some(outlined) = font.outline_glyph(glyph) {
                                    outlined.draw(|x, y, c| {
                                        let draw_x = pos_x + x as f32;
                                        let draw_y = pos_y + y as f32;
                                        window_handle.draw_text_at(draw_x as usize, draw_y as usize, c);
                                        //self.client.draw_text(&window.id, draw_x as u32, draw_y as u32, c);
                                    });
                                }
                            
                                pen_x += h_advance as f32 * scale;
                            }
                        }
                    }
                }

                window_handle.commit();
                window_handle.draw();
            });


            self.event_queue.blocking_dispatch(&mut self.client).unwrap();
        }
    }

    fn init_windows_backends(&mut self) -> Result<Vec<WindowContext<T>>, Error> {
        let user_windows = self.gui.setup_windows();
        let mut backends = Vec::with_capacity(user_windows.len());
        let qh = self.event_queue.handle();

        user_windows.into_iter().for_each(|user_window| {
            let request = user_window.request();
            let handle = match request.layer {
                window::WindowLayer::Desktop(default_options) => self.client.desktop_window(
                    self.display_ptr,
                    &qh,
                    &request.id,
                    request.width,
                    request.height,
                    default_options,
                ),
                window::WindowLayer::Top(special_options) => self.client.special_window(
                    self.display_ptr,
                    &qh,
                    &request.id,
                    request.width,
                    request.height,
                    Layer::Top,
                    special_options
                ),
                window::WindowLayer::Bottom(special_options) => self.client.special_window(
                    self.display_ptr,
                    &qh,
                    &request.id,
                    request.width,
                    request.height,
                    Layer::Bottom,
                    special_options
                ),
                window::WindowLayer::Overlay(special_options) => self.client.special_window(
                    self.display_ptr,
                    &qh,
                    &request.id,
                    request.width,
                    request.height,
                    Layer::Overlay,
                    special_options
                ),
                window::WindowLayer::Background(special_options) => self.client.special_window(
                    self.display_ptr,
                    &qh,
                    &request.id,
                    request.width,
                    request.height,
                    Layer::Overlay,
                    special_options,
                ),
            };

            let root = user_window.setup(&mut self.gui);
            let window = Window::new(root, handle);
            backends.push(WindowContext::new(user_window, window));
        });

        Ok(backends)
    }
}

pub trait UserWindow<T: GUI> {
    fn request(&self) -> WindowRequest;
    fn setup(&self, gui: &mut T) -> Box<dyn Container>;
    fn signals(&self, gui: &mut T, window: &Window);
    fn update(&mut self, gui: &mut T, window: &mut Window);
}

pub struct WindowContext<T: GUI> {
    window: Window,
    graphics: Box<dyn UserWindow<T>>,
}

impl<T: GUI> WindowContext<T> {
    pub const fn new(
        graphics: Box<dyn UserWindow<T>>,
        window: Window
        ) -> Self {
        Self {
            graphics,
            window,
        }
    }
}

use std::time::{Instant, Duration};

pub struct FpsCounter {
    last_frame_time: Instant,
    frame_times: Vec<Duration>,
    max_samples: usize,
}

impl FpsCounter {
    pub fn new(max_samples: usize) -> Self {
        Self {
            last_frame_time: Instant::now(),
            frame_times: Vec::with_capacity(max_samples),
            max_samples,
        }
    }

    pub fn tick(&mut self) -> f64 {
        let now = Instant::now();
        let delta = now - self.last_frame_time;
        self.last_frame_time = now;

        self.frame_times.push(delta);

        if self.frame_times.len() > self.max_samples {
            self.frame_times.remove(0);
        }

        let avg_duration: Duration = self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32;
        1.0 / avg_duration.as_secs_f64()
    }
}

//DrawCommand::Text { font, content, size } => {
//    let font = self.content.get_font(font);
//
//    let units_per_em = font.units_per_em().unwrap();
//    let scale = size / units_per_em;
//
//    let ascent = font.ascent_unscaled();
//    let base_y = ascent * scale; // <-- вот это основное изменение
//
//    let mut pen_x = 0.0;
//
//    for ch in content.chars() {
//        let glyph = font.glyph_id(ch).with_scale_and_position(size, point(pen_x, base_y));
//        let pos_x = glyph.position.x;
//        let pos_y = glyph.position.y;
//        let h_advance = font.h_advance_unscaled(glyph.id);
//        
//        if let Some(outlined) = font.outline_glyph(glyph) {
//            outlined.draw(|x, y, c| {
//                let draw_x = pos_x + x as f32;
//                let draw_y = pos_y + y as f32;
//                //self.client.draw_text(&window.id, draw_x as u32, draw_y as u32, c);
//            });
//        }
//
//        pen_x += h_advance as f32 * scale;
//    }
//},
