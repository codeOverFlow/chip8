mod system;

use system::chip;

use std::time::Duration;
use std::thread;
use std::io;
use std::io::prelude::*;
use std::fs::File;

const VITESSE_CPU: u8 = 4;
const FPS: u8 = 16;

fn main() {
    let mut chip = chip::init_chip();

    let mut f = File::open("/home/adrien/Documents/perso/rust/chip8/games/INVADERS").unwrap();
    let mut i: usize = system::cpu::ENTRY_POINT as usize;
    for byte in f.bytes() {
        chip.cpu.memory[i] = byte.unwrap();
        i += 1;
    }

    println!("{}\n", chip.cpu);

    loop {
        let mut opcode: u16 = chip.cpu.get_op_code();
        chip.interpret(opcode);
       
        opcode = chip.cpu.get_op_code();
        chip.interpret(opcode);
        
        opcode = chip.cpu.get_op_code();
        chip.interpret(opcode);
        
        opcode = chip.cpu.get_op_code();
        chip.interpret(opcode);
        thread::sleep(Duration::from_millis(FPS as u64));

        println!("{}", chip.screen);
    }
}
