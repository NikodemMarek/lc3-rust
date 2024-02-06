use std::fs::File;
use std::io::{self, Read, Write};

use crate::hardware::Hardware;
use crate::instructions;

pub fn run<R: Read, W: Write>(file_path: &str, hardware: &mut Hardware<R, W>) {
    let program = read_binary_file(file_path).unwrap();
    hardware.load(&program);

    main_loop(hardware);
}

pub fn main_loop<R: Read, W: Write>(hardware: &mut Hardware<R, W>) {
    while let Some(instruction) = hardware.next() {
        if instruction != 0b0000_0000_0000_0000 {
            instructions::process(instruction, hardware);
        }
    }
}

fn read_binary_file(file_path: &str) -> io::Result<Vec<u16>> {
    let mut file = File::open(file_path)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let arr = buffer.chunks(2).map(|chunk| {
        let mut bytes = [0; 2];
        bytes.copy_from_slice(chunk);
        u16::from_be_bytes(bytes)
    }).collect::<Vec<_>>();

    Ok(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_program_from_file() {
        let mut hardware = Hardware::default();

        let program = read_binary_file("test.obj").unwrap();
        hardware.load(&program);

        assert_eq!(hardware.memory.get(0x3000), 0b1110_0010_1111_1111);
        assert_eq!(hardware.memory.get(0x3005), 0b0110_1000_0100_0000);
        assert_eq!(hardware.memory.get(0x3009), 0b0000_1111_1111_1010);
        assert_eq!(hardware.memory.get(0x300A), 0b0000_0000_0000_0000);
    }
}
