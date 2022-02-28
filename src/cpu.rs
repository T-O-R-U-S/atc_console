use crate::color::Colour;

#[derive(Debug, Clone)]
pub struct Cpu {
	pub memory: [Mem; 255],
	pub buf: [Colour; crate::WIDTH * crate::HEIGHT],
	pub header: HeaderData
}

#[derive(Copy, Clone, Debug)]
pub enum Mem {
	Str([char; 8]),
	ByteArr([u8; 8]),
	Int(i64),
	Float(f64),
	Nil
}

impl Mem {
	pub fn to_num(&self) -> f64 {
		match self {
			Mem::Int(i) => *i as f64,
			Mem::Float(i) => *i,
			any => panic!("Expected int, found {any:?}")
		}
	}
}

#[derive(Debug, Clone)]
pub struct HeaderData {
	title: String,
	repeat: bool,
	alt_colours: bool
}

impl Cpu {
	pub fn new() -> Self {
		Cpu {
			memory: [Mem::Nil; 255],
			buf: [Colour::DarkGreen; 65025],
			header: HeaderData {
				title: "ATC Fantasy Console".into(),
				repeat: false,
				alt_colours: false
			}
		}
	}

	pub fn run(&mut self, bytecode: Vec<u8>) {
		let mut bytecode = bytecode.into_iter();

		while let Some(header @ 0x01..) = bytecode.next() {
			match header {
				0x01 => {
					let mut string = String::new();
					// FIXME: Better way to do this?
					while let Some(byte @ 0x00 | byte @ 0x02..) = bytecode.next() {
						match byte {
							0x00 => string.push(bytecode.next().unwrap().into()),
							any => string.push(any.into())
						}
					}

					self.header.title = string;
				}
				0x02 => {
					self.header.repeat = true;
				}
				0x03 => todo!("WARN: Alt colour pallette not implemented!"),
				_ => panic!("Unexpected byte in header info")
		}

		let bytecode_clone = bytecode.clone();

		loop {
				while let Some(code) = bytecode.next() {
					match code {
						0x00 => {}
						0x01 => {
							let x: usize = bytecode.next().unwrap().into();
							let y: usize = bytecode.next().unwrap().into();
							let clr = bytecode.next().unwrap();
							self.buf[x * y] = Colour::from_hex(clr);
						}
						0xf0 => {
							let lhs = bytecode.next().unwrap() as usize;
							let rhs = bytecode.next().unwrap() as usize;
							let addr = bytecode.next().unwrap() as usize;

							self.memory[addr] = Mem::Float(self.memory[lhs].to_num() / self.memory[rhs].to_num())
						}
						0xf1 => {
							let lhs = bytecode.next().unwrap() as usize;
							let rhs = bytecode.next().unwrap() as usize;
							let addr = bytecode.next().unwrap() as usize;

							self.memory[addr] = Mem::Float(self.memory[lhs].to_num() - self.memory[rhs].to_num())
						}
						0xf2 => {
							let lhs = bytecode.next().unwrap() as usize;
							let rhs = bytecode.next().unwrap() as usize;
							let addr = bytecode.next().unwrap() as usize;

							self.memory[addr] = Mem::Float(self.memory[lhs].to_num() + self.memory[rhs].to_num())
						}
						0xf3 => {
							let lhs = bytecode.next().unwrap() as usize;
							let rhs = bytecode.next().unwrap() as usize;
							let addr = bytecode.next().unwrap() as usize;

							self.memory[addr] = Mem::Float(self.memory[lhs].to_num() * self.memory[rhs].to_num())
						}
						0xa1 => {
							let ty = bytecode.next().unwrap();

							let data: [u8; 8] = [
								bytecode.next().unwrap(),
								bytecode.next().unwrap(),
								bytecode.next().unwrap(),
								bytecode.next().unwrap(),
								bytecode.next().unwrap(),
								bytecode.next().unwrap(),
								bytecode.next().unwrap(),
								bytecode.next().unwrap(),
							];

							let addr = bytecode.next().unwrap();

							let out = match ty {
								0xe0 => Mem::Int(i64::from_le_bytes(data)),
								0xf0 => Mem::Float(f64::from_le_bytes(data)),
								0xab => Mem::Str(
									data.map(|e| e as char)
								),
								0x8a => Mem::ByteArr(
									data
								),
								any => panic!("Unknown type: {any:x}")
							};
						}
						inst => panic!("Unrecognized instruction: {inst:x}")
					}
				}

				if !self.header.repeat {
					break
				}

				bytecode = bytecode_clone.clone();
			}
		}
	}
}