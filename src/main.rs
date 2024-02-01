mod registers;
mod memory;
mod hardware;
mod run;
mod instructions;
mod utils;
mod traps;

fn main() {
    run::run("test.obj", &mut std::io::stdout());
}
