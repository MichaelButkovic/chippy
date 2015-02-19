// 16 1-byte registers - V0 to VF
// 4k total memory (4096)

#![feature(io, path, os)]

use std::os;
use std::old_io;
use std::old_io::File;
use std::old_io::BufferedReader;

mod input;
mod graphics;
mod cpu;

struct CPU {
    memory: [u16; 4096], // system memory 
    V: [u8; 16], // registers (unsigned char: 1 byte)
    PC: u16, // program counter (unsigned short: 2 bytes)
    I: u16, // index register (unsigned short: 2 bytes)
}

fn load(game: String) {
    let mut path = os::getcwd().unwrap();
    path.push(game.trim());
    let mut reader = File::open(&path).unwrap();

    loop {
        match reader.read_byte() {
            Ok(value) => {
               println!("{:X}",value); 
            }
            Err(e) => {
                return;
            }
        }
    }
}

fn main() {
    // setUpGraphics();
    // setUpInput();

    // initiialize system
    // load game into memory
    
    println!("enter the name of the game");

    let input = old_io::stdin().read_line().ok().expect("Failed to read line");
    let game = format!("roms/{}", input);

    load(game);

    // emulation loop 
    /*loop {
        // do 1 cycle
        // draw screen if draw flag is set
        // store key press state
    }*/
}
