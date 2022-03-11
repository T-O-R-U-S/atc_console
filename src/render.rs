use std::default;

use crate::{color::Colour, key::Key, HEIGHT, RES, WIDTH};

use fltk::{
    app::{self, App},
    enums::Event,
    prelude::*,
    window::{DoubleWindow, Window as FWin},
};
use minifb::{Window as MWin, WindowOptions};
use pixels::{raw_window_handle::HasRawWindowHandle, Pixels, SurfaceTexture};

pub trait RenderBackend {
    /// Updates the screen. Should panic on error, since
    /// this is a failure on the interpreter side that
    /// cannot be handled by the user.
    fn update(&mut self, buf: [Colour; RES]);

    fn new() -> Self;

    fn is_open(&self) -> bool;

    fn key(&self, key: Key) -> bool;
}

impl RenderBackend for MWin {
    fn update(&mut self, buf: [Colour; HEIGHT * WIDTH]) {
        self.update_with_buffer(&buf.map(|e| e as u32), WIDTH, HEIGHT)
            .unwrap()
    }

    fn new() -> Self {
        Self::new(
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

pub struct FltkPixels(FWin, App, Pixels);

impl RenderBackend for FltkPixels {
    fn update(&mut self, buf: [Colour; RES]) {
        let pixels = self.2.get_frame().chunks_exact_mut(4).enumerate();

        for (idx, pix) in pixels {
            pix.copy_from_slice(&buf[idx].into_rgba());
        }

        if self
            .2
            .render()
            .map_err(|e| eprintln!("pixels.render() failed: {}", e))
            .is_err()
        {
            self.1.quit();
        }

        app::flush();
        app::awake();
    }

    fn new() -> Self {
        let app = App::default();
        let mut win = FWin::default()
            .with_label("ATC Fantasy Console")
            .with_size(WIDTH as i32, HEIGHT as i32);

        win.end();

        win.show();

        let pixel_width = win.pixel_w() as u32;
        let pixel_height = win.pixel_h() as u32;
        let surface_texture = SurfaceTexture::new(pixel_width, pixel_height, &win);
        let pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap();

        Self(win, app, pixels)
    }

    fn is_open(&self) -> bool {
        self.1.wait()
    }

    fn key(&self, key: Key) -> bool {
        todo!()
    }
}