use std::fs::File;
use std::io::{self, Read};

use crate::hardware::Hardware;

pub fn run() {
    let mut hardware = Hardware::default();

    let program = read_binary_file("test.obj").unwrap();
    hardware.load(&program);
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

        assert!(hardware.memory.get(0x3000) == 0b1110_0010_1111_1111);
        assert!(hardware.memory.get(0x3005) == 0b0110_1000_0100_0000);
        assert!(hardware.memory.get(0x3009) == 0b0000_1111_1111_1010);
        assert!(hardware.memory.get(0x300A) == 0b0000_0000_0000_0000);
    }
}
