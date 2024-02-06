mod registers;
mod memory;
mod hardware;
mod run;
mod instructions;
mod utils;
mod traps;

use std::path::Path;

use termios::*;

fn main() {
    let file_path = std::env::args().nth(1).expect("missing file path");
    if !Path::new(&file_path).exists() {
        panic!("file does not exist");
    }

    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin).unwrap();

    let mut new_termios = termios.clone();
    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO);

    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

    let mut hardware = hardware::Hardware::default();
    run::run(&file_path, &mut hardware);

    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}
