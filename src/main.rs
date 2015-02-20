// 16 1-byte registers - V0 to VF
// 4k total memory (4096)

#![feature(io, path, os)]

use std::os;
use std::str;
use std::old_io;
use std::old_io::File;
use std::old_io::BufferedReader;

use cpu::CPU;

mod input;
mod graphics;
mod cpu;

fn main() {
    // setUpGraphics();
    // setUpInput();

    // initiialize system
    // let mut c = CPU::new();
    
    // load game into memory
    println!("enter the name of the game");

    let input = old_io::stdin().read_line().ok().expect("Failed to read line");
    let game = format!("roms/{}", input);

    cpu::load(game);

    // emulation loop 
    loop {
        // do 1 cycle
        break;

        // draw screen if draw flag is set
        // store key press state
    }
}
