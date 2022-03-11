use crate::{color::Colour, key::Key, render::RenderBackend, HEIGHT, RES, WIDTH};

pub struct Cpu<T: RenderBackend> {
    pub memory: [Mem; 255],
    pub buf: [Colour; RES],
    pub window: T,
    pub header: HeaderData,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Mem {
    Str([char; 8]),
    ByteArr([u8; 8]),
    Int(i64),
    Float(f64),
    Nil,
}

impl Mem {
    pub fn to_num(&self) -> f64 {
        match self {
            Mem::Int(i) => *i as f64,
            Mem::Float(i) => *i,
            any => panic!("Expected int, found {any:?}"),
        }
    }
}

pub struct HeaderData {
    title: String,
    repeat: bool,
    alt_colours: bool,
}

pub struct ByteCode(Vec<u8>, usize);

impl ByteCode {
    pub fn next(&mut self) -> Option<u8> {
        let Some(out) = self.0.get(self.1) else {
            return None
        };
        self.1 += 1;

        Some(*out)
    }

    pub fn jmp(&mut self, byte: usize) {
        self.1 = byte
    }

    pub fn new(bytecode: Vec<u8>) -> Self {
        Self(bytecode, 0)
    }
}

impl<T: RenderBackend> Cpu<T> {
    pub fn new() -> Self {
        Cpu {
            memory: [Mem::Nil; 255],
            buf: [Colour::Green; 65025],
            header: HeaderData {
                title: "ATC Fantasy Console".into(),
                repeat: false,
                alt_colours: false,
            },
            window: T::new(),
        }
    }

    pub fn run(&mut self, bytecode: Vec<u8>) {
        let mut bytecode = ByteCode::new(bytecode);

        while let Some(header @ 0x01..) = bytecode.next() {
            match header {
                0x01 => {
                    let mut string = String::new();
                    // FIXME: Better way to do this?
                    while let Some(byte @ 0x00 | byte @ 0x02..) = bytecode.next() {
                        match byte {
                            0x00 => string.push(bytecode.next().unwrap().into()),
                            any => string.push(any.into()),
                        }
                    }

                    self.header.title = string;
                }
                0x02 => {
                    self.header.repeat = true;
                }
                0x03 => todo!("ERR: Alt colour pallette not implemented!"),
                any => panic!("Unexpected byte ({any:x}) in header info"),
            }
        }

        'a: loop {
            while let Some(code) = bytecode.next() {
                if !self.window.is_open() {
                    break 'a;
                }
                match code {
                    0x00 => {}
                    0x01 => {
                        let x: usize = bytecode.next().unwrap().into();
                        let y: usize = bytecode.next().unwrap().into();
                        let clr = bytecode.next().unwrap();
                        // This is the only way to invert the Y axis
                        self.buf[x + y * WIDTH] = Colour::from_hex(clr);
                    }
                    0x02 => {
                        let x: usize =
                            self.memory[bytecode.next().unwrap() as usize].to_num() as usize;
                        let y: usize =
                            self.memory[bytecode.next().unwrap() as usize].to_num() as usize;
                        let clr = bytecode.next().unwrap();
                        self.buf[(x % WIDTH) + (y % HEIGHT) * WIDTH] = Colour::from_hex(clr);
                    }
                    0xf0 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] =
                            Mem::Float(self.memory[lhs].to_num() / self.memory[rhs].to_num())
                    }
                    0xf1 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] =
                            Mem::Float(self.memory[lhs].to_num() - self.memory[rhs].to_num())
                    }
                    0xf2 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] =
                            Mem::Float(self.memory[lhs].to_num() + self.memory[rhs].to_num())
                    }
                    0xf3 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] =
                            Mem::Float(self.memory[lhs].to_num() * self.memory[rhs].to_num())
                    }
                    0xf4 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] = Mem::Int(
                            self.memory[lhs].to_num() as i64 / self.memory[rhs].to_num() as i64,
                        )
                    }
                    0xf5 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] = Mem::Int(
                            self.memory[lhs].to_num() as i64 - self.memory[rhs].to_num() as i64,
                        )
                    }
                    0xf6 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] = Mem::Int(
                            self.memory[lhs].to_num() as i64 + self.memory[rhs].to_num() as i64,
                        )
                    }
                    0xf7 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] = Mem::Int(
                            self.memory[lhs].to_num() as i64 * self.memory[rhs].to_num() as i64,
                        )
                    }
                    0xb0 => {
                        let addr_num = bytecode.next().unwrap() as usize;
                        let out_addr = bytecode.next().unwrap() as usize;

                        self.memory[out_addr] = if self.memory[addr_num] == Mem::Int(0x00) {
                            Mem::Int(0x01)
                        } else {
                            Mem::Int(0x00)
                        }
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

                        let addr = bytecode.next().unwrap() as usize;

                        let out = match ty {
                            0xe0 => Mem::Int(i64::from_le_bytes(data)),
                            0xf0 => Mem::Float(f64::from_le_bytes(data)),
                            0xab => Mem::Str(data.map(|e| e as char)),
                            0x8a => Mem::ByteArr(data),
                            any => panic!("Unknown type: {any:x}"),
                        };

                        self.memory[addr] = out;
                    }
                    0xa2 => {
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

                        let addr = bytecode.next().unwrap() as usize;

                        if self.memory[addr] == Mem::Nil {
                            let out = match ty {
                                0xe0 => Mem::Int(i64::from_le_bytes(data)),
                                0xf0 => Mem::Float(f64::from_le_bytes(data)),
                                0xab => Mem::Str(data.map(|e| e as char)),
                                0x8a => Mem::ByteArr(data),
                                any => panic!("Unknown type: {any:x}"),
                            };

                            self.memory[addr] = out;
                        }
                    }
                    0xa3 => {
                        let arr_addr = bytecode.next().unwrap() as usize;
                        let idx @ 0..=7 = bytecode.next().unwrap() as usize else {
                                                            panic!("Array index out of bounds. (Zero based indexing!)")
                                                        };
                        let item = bytecode.next().unwrap();

                        match &mut self.memory[arr_addr] {
                            Mem::ByteArr(arr) => arr[idx] = item,
                            Mem::Str(arr) => arr[idx] = item as char,
                            any => {
                                panic!("Expected array, found {any:?} at address {arr_addr:x}")
                            }
                        }
                    }
                    0xe1 => {
                        let var_addr = bytecode.next().unwrap() as usize;

                        let jmp_byte = [
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                        ];

                        if self.memory[var_addr] == Mem::Int(0x01) {
                            bytecode.jmp(
                                usize::from_le_bytes(jmp_byte)
                            )
                        }
                    }
                    0xe2 => {
                        let var_addr = bytecode.next().unwrap() as usize;

                        let jmp_byte = [
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                        ];

                        if self.memory[var_addr] != Mem::Int(0x01) {
                            bytecode.jmp(
                                usize::from_le_bytes(jmp_byte)
                            )
                        }
                    }
                    0xd0 => {
                        let keycode = Key::from_hex(bytecode.next().unwrap());
                        let addr = bytecode.next().unwrap() as usize;

                        if self.window.key(keycode) {
                            self.memory[addr] = Mem::Int(0x01)
                        } else {
                            self.memory[addr] = Mem::Int(0x00)
                        };
                    }
                    inst => panic!("Unrecognized instruction: {inst:x}"),
                }

                self.window.update(self.buf);
            }

            if !self.header.repeat {
                break;
            }

            bytecode.jmp(0)
        }
    }
}
