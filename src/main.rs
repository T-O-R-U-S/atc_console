#![feature(box_syntax)]
#![feature(let_else)]
#![feature(int_abs_diff)]
#![feature(generic_arg_infer)]
#![feature(let_chains)]

pub mod color;
pub mod cpu;
pub mod key;
pub mod render;

use cpu::Cpu;

use render::FltkPixels;

use std::{fs::read, env::args};

pub const WIDTH: usize = 255;
pub const HEIGHT: usize = 255;
pub const RES: usize = WIDTH * HEIGHT;

fn main() {
    let mut cpu: Cpu<FltkPixels> = Cpu::new();

    let file_name = args().nth(1).unwrap();

    let bytecode = read(&file_name).unwrap();

    cpu.run(bytecode);
}
