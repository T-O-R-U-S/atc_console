#![feature(box_syntax)]
#![feature(let_else)]

pub mod cpu;
pub mod color;
pub mod key;

use color::Colour;
use cpu::Cpu;

use minifb::{Window, WindowOptions};

use std::time::{ Instant };

use std::fs::read;

pub const WIDTH: usize = 255;
pub const HEIGHT: usize = 255;

fn main() {
    let mut buffer: [u32; HEIGHT * WIDTH] = [0xffffff; HEIGHT * WIDTH];

    let mut cpu = Cpu::new();

    let bytecode = read("./game.atc").unwrap();

    cpu.run(bytecode);
}
