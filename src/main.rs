mod system;

use system::cpu;
use system::screen;

use std::time::Duration;
use std::thread;

const VITESSE_CPU: u8 = 4;
const FPS: u8 = 16;

fn main() {
    let mut cpu = cpu::init_cpu();
    println!("{}\n\n", cpu);

    let mut screen = screen::init_screen();
    println!("{}\n\n", screen);

    loop {
        for _ in 0..VITESSE_CPU {
            println!("{}", cpu.get_op_code());
        }
        println!("{}\n\n", screen);
        thread::sleep(Duration::from_millis(FPS as u64));
    }
}
