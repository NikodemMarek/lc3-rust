use std::fs::File;
use std::io::{self, Read, Write};

use crate::hardware::Hardware;
use crate::instructions;

pub fn run(file_path: &str, output: &mut impl Write) {
    let mut hardware = Hardware::default();

    let program = read_binary_file(file_path).unwrap();
    hardware.load(&program);

    main_loop(&mut hardware, output);
}

pub fn main_loop(hardware: &mut Hardware, output: &mut impl Write) {
    while let Some(instruction) = hardware.next() {
        if instruction != 0b0000_0000_0000_0000 {
            println!("{:#06x}: {:#018b}", hardware.program_counter.get(), instruction);
            instructions::process(instruction, hardware, output);
        }
    }
}

fn read_binary_file(file_path: &str) -> io::Result<Vec<i16>> {
    let mut file = File::open(file_path)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let arr = buffer.chunks(2).map(|chunk| {
        let mut bytes = [0; 2];
        bytes.copy_from_slice(chunk);
        i16::from_be_bytes(bytes)
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

        assert!(hardware.memory.get(0x3000) == 0b1110_0010_1111_1111u16 as i16);
        assert!(hardware.memory.get(0x3005) == 0b0110_1000_0100_0000u16 as i16);
        assert!(hardware.memory.get(0x3009) == 0b0000_1111_1111_1010u16 as i16);
        assert!(hardware.memory.get(0x300A) == 0b0000_0000_0000_0000u16 as i16);
    }
}
