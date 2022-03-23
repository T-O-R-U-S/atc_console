use crate::{color::Colour, key::Key, HEIGHT, RES, WIDTH};

use fltk::{
    enums::{Key as FKey},
    app::{self, App},
    prelude::*,
    window::{Window as FWin},
};
use minifb::{Window as MWin, WindowOptions};
use pixels::{Pixels, SurfaceTexture};

pub trait RenderBackend {
    /// Updates the screen. Should panic on error, since
    /// this is a failure on the interpreter side that
    /// cannot be handled by the user.
    fn update(&mut self, buf: [Colour; RES]);

    fn new() -> Self;

    fn is_open(&self) -> bool;

    fn key(&self, key: Key) -> bool;

    fn fltk_up(&self) {}
}

impl From<Colour> for u32 {
    fn from(clr: Colour) -> Self {
        clr as u32
    }
}

impl RenderBackend for MWin {
    fn update(&mut self, buf: [Colour; HEIGHT * WIDTH]) {
        self.update_with_buffer(&buf.map(u32::from), WIDTH, HEIGHT)
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

    fn fltk_up(&self) {}
}

pub struct FltkPixels(FWin, App, Pixels);

impl RenderBackend for FltkPixels {
    fn update(&mut self, buf: [Colour; RES]) {
        let pixels = self.2.get_frame().chunks_exact_mut(4);

        for (pix, new) in pixels.zip(buf) {
            pix.copy_from_slice(&new.into_rgba());
        }

        if self
            .2
            .render()
            .map_err(|e| eprintln!("pixels.render() failed: {:?}", e))
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
            .with_size(WIDTH as i32 * 2, HEIGHT as i32 * 2);

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
        // Could defintely be handled better.

        if app::event_text().is_empty() {
            return match app::event_key() {
                FKey::Up => Key::Up == key,
                FKey::Down => Key::Dwn == key,
                FKey::Left => Key::Lft == key,
                FKey::Right => Key::Rght == key,
                FKey::ControlL => Key::LCtrl == key,
                FKey::ControlR => Key::RCtrl == key,
                _ => false
            }
        }

        if let Some(mat) = Key::from_str(&app::event_text()) && mat == key {
            true
        } else {
            false
        }
    }

    fn fltk_up(&self) {
        app::awake();
    }
}
