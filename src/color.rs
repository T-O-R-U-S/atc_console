#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Colour {
    Black = 0x000000,
    White = 0xfcfcfc,
    Gray = 0x7c7c7c,

    Red = 0xC70039,
    Green = 0x3CFF00,
    Blue = 0x00BBFF,

    DarkRed = 0xa81000,
    DarkGreen = 0x007800,
    DarkBlue = 0x0058f8,

    Cyan = 0x00fcfc,

    Orange = 0xFF5733,
    Yellow = 0xf8b800,
    Brown = 0x7E4100,

    Purple = 0xA600FF,
    Pink = 0xFF0074,

    LightGray = 0xbcbcbc,

    Transparent = 0x999999
}

use Colour::*;

impl Colour {
    pub fn into_rgba(self) -> [u8; 4] {
        let [b, g, r, _] = (self as u32).to_ne_bytes();
        
        [r, g, b, 0xff]
    }

    pub fn from_hex(num: u8) -> Self {
        match num {
            0x00 => Black,
            0xff => White,
            0x0f => Gray,
            0x1f => LightGray,
            0x0a => DarkRed,
            0x1a => Red,
            0x0b => DarkGreen,
            0x1b => Green,
            0x0c => DarkBlue,
            0x1c => Blue,
            0x2c => Cyan,
            0xab => Yellow,
            0xfa => Orange,
            0x30 => Brown,
            0xac => Purple,
            0xbf => Pink,
            0x99 => Transparent,
            any => panic!("Unknown colour hexcode: {any:x}!"),
        }
    }
}
