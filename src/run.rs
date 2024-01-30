use std::fs::File;
use std::io::{self, Read};

use crate::hardware::Hardware;
use crate::instructions;

pub fn run(file_path: &str) {
    let mut hardware = Hardware::default();

    let program = read_binary_file(file_path).unwrap();
    hardware.load(&program);

    main_loop(&mut hardware);
}

pub fn main_loop(hardware: &mut Hardware) {
    while let Some(instruction) = hardware.next() {
        if instruction != 0b0000_0000_0000_0000 {
            println!("{:#06x}: {:#018b}", hardware.program_counter.get(), instruction);
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

        assert!(hardware.memory.get(0x3000) == 0b1110_0010_1111_1111);
        assert!(hardware.memory.get(0x3005) == 0b0110_1000_0100_0000);
        assert!(hardware.memory.get(0x3009) == 0b0000_1111_1111_1010);
        assert!(hardware.memory.get(0x300A) == 0b0000_0000_0000_0000);
    }

    #[test]
    fn ld() {
        let mut hardware = Hardware::default();
        hardware.load(&[
             0b0010_0010_0000_0001,
             0b0000_0000_0000_0000,
             0b0000_1111_1111_0000,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000);
        assert!(hardware.flags.is_positive());
    }

    #[test]
    fn ldi() {
        let mut hardware = Hardware::default();
        hardware.load(&[
             0b1010_0010_0000_0000,
             0b0011_0000_0000_0011,
             0b0000_0000_0000_0000,
             0b0000_1111_1111_0000,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000);
        assert!(hardware.flags.is_positive());
    }
}
