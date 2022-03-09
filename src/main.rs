#![feature(box_syntax)]
#![feature(let_else)]

pub mod color;
pub mod cpu;
pub mod key;

use color::Colour;
use cpu::Cpu;

use minifb::{Window, WindowOptions};

use std::fs::read;

use std::panic::{self};
use std::thread::sleep;
use std::time::Duration;

pub const WIDTH: usize = 255;
pub const HEIGHT: usize = 255;

fn main() {
    let mut cpu = Cpu::new();

    let bytecode = read("./game.atc").unwrap();

    cpu.run(bytecode);
}
