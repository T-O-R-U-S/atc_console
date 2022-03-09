use crate::{color::Colour, key::Key, HEIGHT, WIDTH};
use minifb::{Window, WindowOptions};

pub trait RenderBackend {
    /// Updates the screen. Should panic on error, since
    /// this is a failure on the interpreter side that
    /// cannot be handled by the user.
    fn update(&mut self, buf: [Colour; HEIGHT * WIDTH]);

    fn new() -> Self;

    fn is_open(&self) -> bool;

    fn key(&self, key: Key) -> bool;
}

impl RenderBackend for Window {
    fn update(&mut self, buf: [Colour; HEIGHT * WIDTH]) {
        self.update_with_buffer(&buf.map(|e| e as u32), WIDTH, HEIGHT)
            .unwrap()
    }

    fn new() -> Self {
        Window::new(
            "ATC Fantasy Console",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .unwrap()
    }

    fn is_open(&self) -> bool {
        self.is_open()
    }

    fn key(&self, key: Key) -> bool {
        self.is_key_pressed(key.to_fb_key(), minifb::KeyRepeat::No)
    }
}
