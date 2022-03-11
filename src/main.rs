#![feature(box_syntax)]
#![feature(let_else)]
#![feature(int_abs_diff)]
#![feature(generic_arg_infer)]
#![feature(array_zip)]

pub mod color;
pub mod cpu;
pub mod key;
pub mod render;

use cpu::Cpu;

use minifb::Window;
use render::FltkPixels;

use std::fs::read;

pub const WIDTH: usize = 255;
pub const HEIGHT: usize = 255;
pub const RES: usize = WIDTH * HEIGHT;

fn main() {
    let mut cpu: Cpu<FltkPixels> = Cpu::new();

    let bytecode = read("./game.atc").unwrap();

    cpu.run(bytecode);
}
