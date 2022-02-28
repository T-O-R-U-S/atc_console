use crate::color::Colour;

#[derive(Debug, Copy, Clone)]
pub struct Cpu {
	pub memory: [Mem; 255],
	pub buf: [Colour; crate::WIDTH * crate::HEIGHT]
}

#[derive(Copy, Clone, Debug)]
pub enum Mem {
	Str([char; 8]),
	ByteArr([u8; 8]),
	Int(i64),
	Float(f64),
	Nil
}

pub struct HeaderData {
	title: String,
	repeat: bool,
	alt_colours: bool
}

impl Cpu {
	pub fn new() -> Self {
		Cpu {
			memory: [Mem::Nil; 255],
			buf: [Colour::DarkGreen; 102400]
		}
	}
	pub fn run(&mut self, bytecode: Vec<u8>) {
		let mut bytecode = bytecode.into_iter();


		while let Some(header @ 0x01..) = bytecode.next() {
			match header {
				0x01 => while let Some(byte @ 0x00 | byte @ 0x02..) = bytecode.next() {

				}
				_ => panic!("Unexpected byte in header info")
		}
	}
}
}