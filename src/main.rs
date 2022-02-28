use minifb::{Window, WindowOptions};

use std::time::{ Instant };

const WIDTH: usize = 320;
const HEIGHT: usize = 320;

#[allow(dead_code)]
#[repr(u32)]
enum Colour {
    Black = 0x000000,
    White = 0xfcfcfc,
    Gray = 0x7c7c7c,
    
    Red = 0xC70039,
    Green = 0x3CFF00,
    Blue = 0x00BBFF,
    
    DarkGreen = 0x007800,
    DarkBlue = 0x0058f8,
    DarkRed = 0xa81000,

    Cyan = 0x00fcfc,
    
    Orange = 0xFF5733,
    Yellow = 0xf8b800,
    Brown = 0x7E4100,
    
    Purple = 0xA600FF,
    Pink = 0xFF0074,

    LightGray = 0xbcbcbc,
}

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
