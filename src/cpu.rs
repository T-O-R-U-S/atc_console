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
    pub fn to_num(&self) -> Result<f64, Mem> {
        match self {
            Mem::Int(i) => Ok(*i as f64),
            Mem::Float(i) => Ok(*i),
            any => Err(*any),
        }
    }
}

pub struct HeaderData {
    title: String,
    repeat: bool,
    // TODO: impl alt_colours
    #[allow(dead_code)]
    alt_colours: bool,
    keep_open: bool,
    debug: bool
}

impl Default for HeaderData {
    fn default() -> Self {
        HeaderData {
            title: "ATC Fantasy Console".into(),
            repeat: false,
            alt_colours: false,
            keep_open: false,
            debug: false
        }
    }
}

pub struct ByteCode(Vec<u8>, usize);

pub enum ByteOption<'a> {
    Some(u8),
    None(&'a ByteCode),
}

impl ByteOption<'_> {
    pub fn unwrap(self) -> u8 {
        match self {
            ByteOption::Some(num) => num,
            ByteOption::None(bytecode) => panic!("Unexpected EOF at byte {}", bytecode.1),
        }
    }

    pub fn expect(self, panic_str: &str) -> u8 {
        match self {
            ByteOption::Some(num) => num,
            ByteOption::None(bytecode) => panic!("{panic_str} @ {}", bytecode.1),
        }
    }
}

impl ByteCode {
    pub fn next(&mut self) -> ByteOption {
        let Some(out) = self.0.get(self.1) else {
            return ByteOption::None(self)
        };
        self.1 += 1;

        ByteOption::Some(*out)
    }

    pub fn shift(&mut self) -> ByteOption {
        let out = self.0.remove(0);

        ByteOption::Some(out)
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
            header: HeaderData::default(),
            window: T::new(),
        }
    }

    pub fn run(&mut self, bytecode: Vec<u8>) {
        let mut bytecode = ByteCode::new(bytecode);

        while let ByteOption::Some(header) = bytecode.shift() && header != 0 {
            match header {
                0x01 => {
                    let mut string = String::new();
                    while let ByteOption::Some(byte) = bytecode.shift() && byte != 0x01 {
                        match byte {
                            0x00 => string.push(bytecode.shift().unwrap().into()),
                            any => string.push(any.into()),
                        }
                    }

                    self.header.title = string;
                }
                0x02 => {
                    self.header.repeat = true;
                }
                0x03 => todo!("ERR: Alt colour pallette not implemented!"),
                0x04 => self.header.keep_open = true,
                0xd5 => self.header.debug = true,
                any => panic!("Unexpected byte ({any:x}) in header info"),
            }
        }

        'a: loop {

            while let ByteOption::Some(code) = bytecode.next() {
                if !self.window.is_open() {
                    break 'a;
                }

                match code {
                    0x00 => {}
                    0x01 => {
                        let x: usize = bytecode.next().unwrap().into();
                        let y: usize = bytecode.next().unwrap().into();
                        let clr = Colour::from_hex(bytecode.next().unwrap());

                        if clr != Colour::Transparent {
                            self.buf[x + y * WIDTH] = clr;
                        };

                        if self.header.debug {
                            println!("CPIX CALL :: ({x}, {y}) => {clr:x?}");
                        }
                    }
                    0x02 => {
                        let x_byte = bytecode.next().unwrap() as usize;
                        let y_byte = bytecode.next().unwrap() as usize;
                        let x: usize = self.memory[x_byte].to_num().expect(&format!(
                            "Failed to cast {:?} to num at byte {} (attempted to access address x: {x_byte:0>2x}, y: {y_byte:0>2x})",
                            self.memory[x_byte], bytecode.1
                        )) as usize;
                        let y: usize = self.memory[y_byte].to_num().expect(&format!(
                            "Failed to cast {:?} to num at byte {} (attempted to access address y: {y_byte:0>2x}, x: {x_byte:0>2x})",
                            self.memory[y_byte], bytecode.1
                        )) as usize;
                        let clr = bytecode.next().unwrap();

                        let clr = Colour::from_hex(clr);

                        if clr != Colour::Transparent {
                            self.buf[(x % WIDTH) + (y % HEIGHT) * WIDTH] = clr;
                        }

                        if self.header.debug {
                            println!("PIX CALL :: ({x} @ {x_byte:0>2x}, {y} @ {y_byte:0>2x}) => {clr:x?}");
                        }
                    }
                    0x03 => {
                        let mut byte_arr = [[0; 8]; 8];

                        for i in 0..8 {
                            let arr_addr = bytecode.next().unwrap() as usize;

                            let Mem::ByteArr(arr) = self.memory[arr_addr] else {
                                panic!("Expected byte array.")
                            };

                            byte_arr[i] = arr;
                        }

                        let x_addr = bytecode.next().unwrap() as usize;
                        let y_addr = bytecode.next().unwrap() as usize;

                        let Mem::Int(x) = self.memory[x_addr] else {
                            panic!("Expected int at addr {x_addr}, but instead found {:?}", self.memory[x_addr])
                        };

                        let Mem::Int(y) = self.memory[y_addr] else {
                            panic!("Expected int at addr {y_addr}, but instead found {:?}", self.memory[y_addr])
                        };

                        for (y_offset, row) in byte_arr.iter().enumerate() {
                            for (x_offset, pix) in row.iter().enumerate() {
                                let x = x as usize + x_offset;
                                let y = (y as usize + y_offset) * WIDTH;

                                let clr = Colour::from_hex(*pix);

                                if clr == Colour::Transparent {
                                    continue;
                                }

                                self.buf[x + y] = clr;
                            }
                        }

                        if self.header.debug {
                            println!("SPR CALL :: ({x}, {y}) @ {byte_arr:0>2x?}");
                        }
                    }
                    0xf0 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] = Mem::Float(
                            self.memory[lhs].to_num().expect(&format!(
                                "Failed to cast {:?} to number at byte {}",
                                self.memory[lhs], bytecode.1
                            )) / self.memory[rhs].to_num().expect(&format!(
                                "Failed to cast {:?} to number at byte {}",
                                self.memory[lhs], bytecode.1
                            )),
                        );

                        if self.header.debug {
                            println!("FDIV CALL :: ({:?} @ {lhs:0>2x}, {:?} @ {rhs:0>2x}) => {addr:0>2x}", self.memory[lhs], self.memory[rhs]);
                        }
                    }
                    0xf1 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] = Mem::Float(
                            self.memory[lhs].to_num().expect(&format!(
                                "Failed to cast {:?} to number at byte {}",
                                self.memory[lhs], bytecode.1
                            )) - self.memory[rhs].to_num().expect(&format!(
                                "Failed to cast {:?} to number at byte {}",
                                self.memory[lhs], bytecode.1
                            )),
                        );

                        if self.header.debug {
                            println!("FSUB CALL :: ({:?} @ {lhs:0>2x}, {:?} @ {rhs:0>2x}) => {addr:0>2x}", self.memory[lhs], self.memory[rhs]);
                        }
                    }
                    0xf2 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] = Mem::Float(
                            self.memory[lhs].to_num().expect(&format!(
                                "Failed to cast {:?} to number at byte {}",
                                self.memory[lhs], bytecode.1
                            )) + self.memory[rhs].to_num().expect(&format!(
                                "Failed to cast {:?} to number at byte {}",
                                self.memory[lhs], bytecode.1
                            )),
                        );

                        if self.header.debug {
                            println!("FADD CALL :: ({:?} @ {lhs:0>2x}, {:?} @ {rhs:0>2x}) => {addr:0>2x}", self.memory[lhs], self.memory[rhs]);
                        }
                    }
                    0xf3 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] = Mem::Float(
                            self.memory[lhs].to_num().expect(&format!(
                                "Failed to cast {:?} to number at byte {}",
                                self.memory[lhs], bytecode.1
                            )) * self.memory[rhs].to_num().expect(&format!(
                                "Failed to cast {:?} to number at byte {}",
                                self.memory[lhs], bytecode.1
                            )),
                        );

                        if self.header.debug {
                            println!("FMUL CALL :: ({:?} @ {lhs:0>2x}, {:?} @ {rhs:0>2x}) => {addr:0>2x}", self.memory[lhs], self.memory[rhs]);
                        }
                    }
                    0xf4 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] = Mem::Int(
                            self.memory[lhs].to_num().expect(&format!(
                                "Failed to cast {:?} to number at byte {}",
                                self.memory[lhs], bytecode.1
                            )) as i64
                                / self.memory[rhs].to_num().expect(&format!(
                                    "Failed to cast {:?} to number at byte {}",
                                    self.memory[lhs], bytecode.1
                                )) as i64,
                        );

                        if self.header.debug {
                            println!("DIV CALL :: ({:?} @ {lhs:0>2x}, {:?} @ {rhs:0>2x}) => {addr:0>2x}", self.memory[lhs], self.memory[rhs]);
                        }
                    }
                    0xf5 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] = Mem::Int(
                            self.memory[lhs].to_num().expect(&format!(
                                "Failed to cast {:?} to number at byte {}",
                                self.memory[lhs], bytecode.1
                            )) as i64
                                - self.memory[rhs].to_num().expect(&format!(
                                    "Failed to cast {:?} to number at byte {}",
                                    self.memory[lhs], bytecode.1
                                )) as i64,
                        );

                        if self.header.debug {
                            println!("SUB CALL :: ({:?} @ {lhs:0>2x}, {:?} @ {rhs:0>2x}) => {addr:0>2x}", self.memory[lhs], self.memory[rhs]);
                        }
                    }
                    0xf6 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] = Mem::Int(
                            self.memory[lhs].to_num().expect(&format!(
                                "Failed to cast {:?} to number at byte {}",
                                self.memory[lhs], bytecode.1
                            )) as i64
                                + self.memory[rhs].to_num().expect(&format!(
                                    "Failed to cast {:?} to number at byte {}",
                                    self.memory[lhs], bytecode.1
                                )) as i64,
                        );

                        if self.header.debug {
                            println!("ADD CALL :: ({:?} @ {lhs:0>2x}, {:?} @ {rhs:0>2x}) => {addr:0>2x}", self.memory[lhs], self.memory[rhs]);
                        }
                    }
                    0xf7 => {
                        let lhs = bytecode.next().unwrap() as usize;
                        let rhs = bytecode.next().unwrap() as usize;
                        let addr = bytecode.next().unwrap() as usize;

                        self.memory[addr] = Mem::Int(
                            self.memory[lhs].to_num().expect(&format!(
                                "Failed to cast {:?} to number at byte {}",
                                self.memory[lhs], bytecode.1
                            )) as i64
                                * self.memory[rhs].to_num().expect(&format!(
                                    "Failed to cast {:?} to number at byte {}",
                                    self.memory[lhs], bytecode.1
                                )) as i64,
                        );

                        if self.header.debug {
                            println!("MUL CALL :: ({:?} @ {lhs:0>2x}, {:?} @ {rhs:0>2x}) => {addr:0>2x}", self.memory[lhs], self.memory[rhs]);
                        }
                    }
                    0xb0 => {
                        let addr_num = bytecode.next().unwrap() as usize;
                        let out_addr = bytecode.next().unwrap() as usize;

                        self.memory[out_addr] = if self.memory[addr_num] == Mem::Int(0x00) {
                            Mem::Int(0x01)
                        } else {
                            Mem::Int(0x00)
                        };

                        if self.header.debug {
                            println!("NOT CALL :: {:?} @ {addr_num} => {out_addr}", self.memory[addr_num]);
                        }
                    }
                    0xb1 => {
                        let lhs_addr = bytecode.next().unwrap() as usize;
                        let rhs_addr = bytecode.next().unwrap() as usize;

                        let lhs = self.memory[lhs_addr];
                        let rhs = self.memory[rhs_addr];

                        let out = bytecode.next().unwrap() as usize;

                        self.memory[out] = match lhs.to_num() > rhs.to_num() {
                            true => Mem::Int(0x01),
                            false => Mem::Int(0x00),
                        };

                        if self.header.debug {
                            println!("GT CALL :: ({:?} @ {lhs:0>2x?} > {:?} @ {rhs:0>2x?}) => {out:0>2x}", self.memory[lhs_addr], self.memory[rhs_addr]);
                        }
                    }
                    0xb2 => {
                        let lhs_addr = bytecode.next().unwrap() as usize;
                        let rhs_addr = bytecode.next().unwrap() as usize;

                        let lhs = self.memory[lhs_addr];
                        let rhs = self.memory[rhs_addr];

                        let out = bytecode.next().unwrap() as usize;

                        self.memory[out] = match lhs.to_num() < rhs.to_num() {
                            true => Mem::Int(0x01),
                            false => Mem::Int(0x00),
                        };

                        if self.header.debug {
                            println!("LT CALL :: ({:?} @ {lhs:0>2x?} > {:?} @ {rhs:0>2x?}) => {out:0>2x}", self.memory[lhs_addr], self.memory[rhs_addr]);
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

                        if self.header.debug {
                            println!("VAR CALL :: {data:0>2x?} of type {ty:0>2x} @ {addr} => {:?}", out);
                        }
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

                        if self.header.debug {
                            println!("LET CALL :: {data:0>2x?} of type {ty:0>2x} @ {addr} => {:?}", self.memory[addr]);
                        }
                    }
                    0xa3 => {
                        let arr_addr = bytecode.next().unwrap() as usize;
                        let idx @ 0..=7 = bytecode.next().unwrap() as usize else {
                                                            panic!("Array index out of bounds. (Zero based indexing!)")
                                                        };
                        let item = bytecode.next().unwrap();

                        println!("ARRW INFO :: {arr_addr:0>2x}[{idx}] WAS {:?}", self.memory[arr_addr]);

                        match &mut self.memory[arr_addr] {
                            Mem::ByteArr(arr) => arr[idx] = item,
                            Mem::Str(arr) => arr[idx] = item as char,
                            any => {
                                panic!("Expected array, found {any:?} at address {arr_addr:x}")
                            }
                        }

                        if self.header.debug {
                            println!("ARRW CALL :: {arr_addr:0>2x}[{idx}] = {item:0>2x}");
                        }
                    }
                    0xe1 => {
                        let var_addr = bytecode.next().unwrap() as usize;

                        let jmp_byte = usize::from_le_bytes([
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                        ]);

                        if self.memory[var_addr] == Mem::Int(0x01) {
                            bytecode.jmp(jmp_byte)
                        }
                        if self.header.debug {
                            println!("TJMP CALL :: TO {jmp_byte} IF {var_addr:0>2x} WHICH IS {:0>2x?}", self.memory[var_addr]);
                        }
                    }
                    0xe2 => {
                        let var_addr = bytecode.next().unwrap() as usize;

                        let jmp_byte = usize::from_le_bytes([
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                        ]);

                        if self.memory[var_addr] != Mem::Int(0x01) {
                            bytecode.jmp(jmp_byte)
                        }

                        if self.header.debug {
                            println!("FJMP CALL :: TO {jmp_byte} IF NOT {var_addr:0>2x} WHICH IS {:?}", self.memory[var_addr]);
                        }
                    }
                    0xe3 => {
                        let jmp_byte = usize::from_le_bytes([
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                            bytecode.next().unwrap(),
                        ]);

                        bytecode.jmp(jmp_byte);

                        if self.header.debug {
                            println!("JMP CALL :: TO {jmp_byte} WHICH IS {:0>2x?}", bytecode.0.get(bytecode.1));
                        }
                    }
                    0xe4 => {
                        let byte_addr = bytecode.next().unwrap() as usize;

                        let Mem::Int(jmp_byte) = self.memory[byte_addr] else {
                            panic!("Expected int for JMP statement at byte {}", bytecode.1)
                        };

                        bytecode.jmp(jmp_byte as usize);

                        if self.header.debug {
                            println!("VJMP CALL :: TO {jmp_byte} WHICH IS {:0>2x?} @ {byte_addr:0>2x} WHICH IS {:?}", bytecode.0.get(bytecode.1), self.memory[byte_addr]);
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

                        if self.header.debug {
                            println!("KEY CALL :: {keycode:0>2x?} => {addr:0>2x} WHICH IS {:?}", self.memory[addr]);
                        }
                    }
                    0xfb => {
                        self.window.update(self.buf);

                        if self.header.debug {
                            println!("RENDER CALL :: RENDERED FRAME SUCCESSFULLY");
                        }
                    }
                    0xfc => {
                        let cls = bytecode.next().unwrap();

                        let cls = Colour::from_hex(cls);

                        self.buf = [cls; RES];

                        if self.header.debug {
                            println!("CLS CALL :: CLEARED SCREEN TO COLOUR CODE {cls:0>2x?}");
                        }
                    }
                    inst => panic!("Unrecognized instruction: {inst:x} at byte {}", bytecode.1),
                }

                // Must be used when using FLTK, otherwise
                // the frame will not render.
                self.window.fltk_up();

                if self.header.debug {
                    println!("INFO :: BYTE NO. {} (0x{:0>8x})", bytecode.1, bytecode.1);
                }
            }

            if !self.header.repeat {
                if self.header.keep_open {
                    while self.window.is_open() {}
                }
                break;
            }

            bytecode.jmp(0);
        }
    }
}
