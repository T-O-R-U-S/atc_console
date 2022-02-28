pub mod cpu;
pub mod color;
pub mod key;

use color::Colour;
use cpu::Cpu;

use minifb::{Window, WindowOptions};

use std::time::{ Instant };

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 320;

fn main() {
    let mut buffer: [u32; HEIGHT * WIDTH] = [0xffffff; HEIGHT * WIDTH];

    let mut window = Window::new(
        "ATC Fantasy Console",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() {
        let now = Instant::now();

        if now.elapsed().as_millis() % 50 != 0 {
            continue;
        }

        for i in 0..102400 {
            buffer[i] = Colour::Purple as u32;
            buffer[(i + WIDTH + 1) % buffer.len()] = Colour::Yellow as u32;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        window.update();
    }
}
