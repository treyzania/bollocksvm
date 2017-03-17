mod cpu;

use std::mem;
use cpu::CPU;

fn main() {
	  let mut cpu = cpu::CPU::new();

    let ad: u64 = 0xffffffffffff1110;

    println!("load => {}", cpu.load(ad));
    cpu.store(ad, 69);
    println!("store.");
    println!("load => {}", cpu.load(ad));
    cpu.store(ad + 1, 42);
    println!("store.");
    println!("load => {}", cpu.load(ad + 1));
}
