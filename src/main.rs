mod registers;
mod memory;
mod hardware;
mod run;
mod instructions;
mod utils;
mod traps;

fn main() {
    let mut hardware = hardware::Hardware::default();
    run::run("2048.obj", &mut hardware);
}
