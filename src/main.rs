mod registers;
mod memory;
mod hardware;
mod run;
mod instructions;
mod utils;

fn main() {
    run::run("test.obj");
}
