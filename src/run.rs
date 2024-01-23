use std::fs::File;
use std::io::{self, Read};

use crate::hardware::Hardware;

pub fn run(file_path: &str) {
    let mut hardware = Hardware::default();

    let program = read_binary_file(file_path).unwrap();
    hardware.load(&program);

    main_loop(&mut hardware);
}

fn main_loop(hardware: &mut Hardware) {
    while let Some(instruction) = hardware.next() {
        if instruction != 0b0000_0000_0000_0000 {
            println!("{:#06x}: {:#018b}", hardware.program_counter.get(), instruction);
            process_instruction(instruction, hardware);
        }
    }
}

fn process_instruction(instruction: u16, hardware: &mut Hardware) {
    match instruction & 0b1111_0000_0000_0000 {
        0b0001_0000_0000_0000 => {
            if instruction & 0b0000_0000_0010_0000 == 0b0000_0000_0000_0000 {
                // ADD 2 registers
            } else {
                // ADD register and imm5
            }
        }, // ADD
        0b0101_0000_0000_0000 => {
            if instruction & 0b0000_0000_0010_0000 == 0b0000_0000_0000_0000 {
                // AND 2 registers
            } else {
                // AND register and imm5
            }
        }, // AND
        0b0000_0000_0000_0000 => {}, // BR
        0b1100_0000_0000_0000 => {}, // JMP / RET
        0b1000_0000_0000_0000 => {
            if instruction & 0b0000_1111_1111_1111 == 0b0000_0000_0000_0000 {
                // RTI
            } else if instruction & 0b0000_1000_0000_0000 == 0b0000_0000_0000_0000 {
                // JSSR
            } else {
                // JSR
            }
        }, // JSR / JSRR / RTI
        0b0010_0000_0000_0000 => {}, // LD
        0b1010_0000_0000_0000 => {}, // LDI
        0b0110_0000_0000_0000 => {}, // LDR
        0b1110_0000_0000_0000 => {}, // LEA
        0b1001_0000_0000_0000 => {}, // NOT
        0b0011_0000_0000_0000 => {}, // ST
        0b1011_0000_0000_0000 => {}, // STI
        0b0111_0000_0000_0000 => {}, // STR
        0b1111_0000_0000_0000 => {}, // TRAP
        0b1101_0000_0000_0000 => {}, // reserved
        _ => panic!("unrecognised instruction"),
    };
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
