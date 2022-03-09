#![feature(box_syntax)]
#![feature(let_else)]
#![feature(int_abs_diff)]

pub mod color;
pub mod cpu;
pub mod key;
pub mod render;

use cpu::Cpu;

use minifb::Window;

use std::fs::read;

pub const WIDTH: usize = 255;
pub const HEIGHT: usize = 255;

fn main() {
    let mut cpu: Cpu<Window> = Cpu::new();

    let bytecode = read("./game.atc").unwrap();

    cpu.run(bytecode);
}
