// 16 1-byte registers - V0 to VF
// 4k total memory (4096)

#![feature(io)]
#![feature(os)]
#![feature(path)]

use std::os;
use std::old_io::File;
use std::old_io::BufferedReader;

mod input;
mod graphics;
mod cpu;

fn load(s: &str) {
    println!("line: {}", s);
}

fn old_main() {
    // setUpGraphics();
    // setUpInput();

    // initiialize system
    // load game into memory
    
    if os::args().len() < 2 {
        println!("specify an input file");
        return;
    }

    let ref filename = os::args()[1];
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));

    for line in file.lines() {
        let l = line.unwrap();
        load(l.as_slice());
    }

    // emulation loop 
    /*loop {
        // do 1 cycle
        println!("emulating!");
    }*/
}
