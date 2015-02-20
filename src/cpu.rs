use std::os;
use std::str;
use std::old_io;
use std::old_io::File;
use std::old_io::BufferedReader;

// cpu info goes here
pub struct CPU {
    memory: [u64; 4096], // system memory 
    V: [u8; 16], // registers (unsigned char: 1 byte)
    PC: u64, // program counter (unsigned short: 2 bytes)
    I: u64, // index register (unsigned short: 2 bytes)
}

pub fn load(game: String) {
    let mut path = os::getcwd().unwrap();
    path.push(game.trim());
    let mut reader = File::open(&path).unwrap();

    loop {
        match reader.read_be_uint_n(2) {
            Ok(value) => {
               println!("value: {:X}", value);
               // add value to memory array
               match value >> 12 {
                   0x0 => {
                       println!("0");
                   },
                   0x1 => op_1NNN(value),
                   0x2 => op_2NNN(value),
                   0x3 => op_3XNN(value),
                   0x4 => op_4XNN(value),
                   0x5 => op_5XY0(value),
                   0x6 => op_6XNN(value),
                   0x7 => op_7XNN(value),
                   0x8 => {
                       println!("8");
                   },
                   0x9 => op_9XY0(value),
                   0xA => op_ANNN(value),
                   0xB => op_BNNN(value),
                   0xC => op_CXNN(value),
                   0xD => op_DXYN(value),
                   0xE => {
                       println!("E");
                   },
                   0xF => {
                       println!("F");
                   },   
                   _ => println!("WHAT'D YOU DO"),
               }
            }
            Err(e) => {
                return;
            }
        }
    }
}

// Calls RCA 1802 program at address NNN.
fn op_0NNN(value:u64) {
    return;
}

// Clears the screen.
fn op_00E0(value:u64) {
    return;
}

// Returns from a subroutine.
fn op_00EE(value:u64) {
    return;
}

// Jumps to address NNN.
fn op_1NNN(value:u64) {
    return;
}

// Calls subroutine at NNN.
fn op_2NNN(value:u64) {
    return;
}

// Skips the next instruction if VX equals NN.
fn op_3XNN(value:u64) {
    return;
}

// Skips the next instruction if VX doesn't equal NN.
fn op_4XNN(value:u64) {
    return;
}

// Skips the next instruction if VX equals VY.
fn op_5XY0(value:u64) {
    return;
}

// Sets VX to NN.
fn op_6XNN(value:u64) {
    println!("read the 6xnn opcode");
}

// Adds NN to VX.
fn op_7XNN(value:u64) {
    return;
}

// Sets VX to the value of VY.
fn op_8XY0(value:u64) {
    return;
}

// Sets VX to VX or VY.
fn op_8XY1(value:u64) {
    return;
}

// Sets VX to VX and VY.
fn op_8XY2(value:u64) {
    return;
}

// Sets VX to VX xor VY.
fn op_8XY3(value:u64) {
    return;
}

// Adds VY to VX. VF is set to 1 when ther's a carry, and 0 when there isn't.
fn op_8XY4(value:u64) {
    return;
}

// VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
fn op_8XY5(value:u64) {
    return;
}

// Shifts VX right by one. VF is set to the value of the least significant bit of VX before the
// shift.
fn op_8XY6(value:u64) {
    return;
}

// Sets VX to VY minus VX. VF is set ot 0 when there's a borrow, and 1 when there isn't
fn op_8XY7(value:u64) {
    return;
}

// Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift.
fn op_8XYE(value:u64) {
    return;
}

// Skips the next instruction if VX doesn't equal VY.
fn op_9XY0(value:u64) {
    return;
}

// Sets I to the address NNN.
fn op_ANNN(value:u64) {
    return;
}

// Sets Jumps to the address NNN plus V0.
fn op_BNNN(value:u64) {
    return;
}

// Sets VX to a random number, masked by NN.
fn op_CXNN(value:u64) {
    return;
}

// Sprites stored in memory at location in index register (I), maximum 8bits wide. Wraps around the
// screen. If when drawn, clears a pixel, register VF is set to 1 otherwise it is zero. All drawing
// is XOR drawing (i.e. it toggles the screen pixels).
fn op_DXYN(value:u64) {
    return;
}

// Skips the next instruction if the key stored in VX is pressed.
fn op_EX9E(value:u64) {
    return;
}

// Skips the next instruction if the key stored in VS isn't pressed.
fn op_EXA1(value:u64) {
    return;
}

// Sets VX to the value of the delay timer.
fn op_FX07(value:u64) {
    return;
}

// A key press is awaited, and then stored in VX.
fn op_FX0A(value:u64) {
    return;
}

// SEts the delay timer to VX.
fn op_FX15(value:u64) {
    return;
}

// Sets the sound timer to VX.
fn op_FX18(value:u64) {
    return;
}

// Adds VX to I.
fn op_FX1E(value:u64) {
    return;
}

// Sets I to the location of the sprite for the character in VX. Characters 0-F (in hex) are
// represented by a 4x5 font.
fn op_FX29(value:u64) {
    return;
}

// Stores the Binary-coded decimal representation of VX, with the most significant of three digits
// at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2.
// (In other words, take the decimal representation of VX, place the hundreds digit in memory at
// location in I, the tens digit at location I+1, and the ones digit at location I+2.).
fn op_FX33(value:u64) {
    return;
}

// Stores V0 to Vx in memory starting at address I.
fn op_FX55(value:u64) {
    return;
}

// Fills V0 to VX with values from memory starting at address I.
fn op_FX65(value:u64) {
    return;
}

