#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
// Virtual keycodes for the ATC Fantasy Console
pub enum Key {
    Q = 0x00,
    W = 0x01,
    E = 0x02,
    A = 0x03,
    S = 0x04,
    D = 0x05,
    Z = 0x06,
    X = 0x07,
    C = 0x08,
    Up = 0xd1,
    Dwn = 0xd2,
    Lft = 0xd3,
    Rght = 0xd4,
    Spc = 0xf0,
}

use minifb::Key as MKey;
use Key::*;

impl Key {
    pub fn to_fb_key(&self) -> MKey {
        match self {
            Q => MKey::Q,
            W => MKey::W,
            E => MKey::E,
            A => MKey::A,
            S => MKey::S,
            D => MKey::D,
            Z => MKey::Z,
            X => MKey::X,
            C => MKey::C,
            Up => MKey::Up,
            Dwn => MKey::Down,
            Lft => MKey::Left,
            Rght => MKey::Right,
            Spc => MKey::Space,
        }
    }

    pub fn from_str(string: &str) -> Option<Key> {
        Some(match string.to_uppercase().as_str() {
            "Q" => Key::Q,
            "W" => Key::W,
            "E" => Key::E,
            "A" => Key::A,
            "S" => Key::S,
            "D" => Key::D,
            "Z" => Key::Z,
            "X" => Key::X,
            "C" => Key::C,
            "UP" => Key::Up,
            "DWN" => Key::Dwn,
            "LFT" => Key::Lft,
            "RGHT" => Key::Rght,
            "SPC" | " " => Key::Spc,
            _ => return None,
        })
    }

    pub fn from_hex(hex: u8) -> Key {
        macro_rules! hexcode {
            ($($ident: ident = $expr: expr),+) => {
              match hex {
              $(
                $expr => $ident
              ),*,
              any => panic!("Cannot convert {any:x} to keycode")
            }
          }
        }

        hexcode! {
            Q    = 0x00,
            W    = 0x01,
            E    = 0x02,
            A    = 0x03,
            S    = 0x04,
            D    = 0x05,
            Z    = 0x06,
            X    = 0x07,
            C    = 0x08,
            Up   = 0xd1,
            Dwn  = 0xd2,
            Lft  = 0xd3,
            Rght = 0xd4,
            Spc  = 0xf0
        }
    }
}
