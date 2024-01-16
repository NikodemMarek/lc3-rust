mod registers;
mod memory;
mod hardware;
mod run;

fn main() {
    run::run("test.obj");
}
