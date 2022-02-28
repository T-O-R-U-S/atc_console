#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Key {
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
	Spc  = 0xf0,
	Esc  = 0xf1
}

use Key::*;
use minifb::Key as MKey;

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
			Esc => MKey::Escape,
		}
	}
}