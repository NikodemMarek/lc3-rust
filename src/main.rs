mod registers;
mod memory;
mod hardware;
mod run;

fn main() {
    println!("Hello, world!");

    let pc = registers::ProgramCounter::default();
}
