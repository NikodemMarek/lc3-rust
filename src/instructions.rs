use std::io::{Write, Read};

use crate::hardware::Hardware;
use crate::traps;
use crate::utils::{imm5, offset6, pcoffset9, register_at, pcoffset11};

pub fn process<R: Read, W: Write>(instruction: u16, hardware: &mut Hardware<R, W>) {
    match instruction >> 12 {
        0x0 => {
            let n = (instruction & 0b0000_1000_0000_0000) == 0b0000_1000_0000_0000;
            let z = (instruction & 0b0000_0100_0000_0000) == 0b0000_0100_0000_0000;
            let p = (instruction & 0b0000_0010_0000_0000) == 0b0000_0010_0000_0000;

            if n && hardware.flags.is_negative() || z && hardware.flags.is_zero() || p && hardware.flags.is_positive() {
                let pcoffset9 = pcoffset9(instruction);
                let loc = hardware.program_counter.get() as u32 + pcoffset9 as u32;

                hardware.program_counter.set(loc as u16);
            }
        }, // BR
        0x1 => {
            let dr = register_at(instruction, 9);
            let sr1 = register_at(instruction, 6);

            let value: u32 = hardware.registers.get(sr1) as u32 + if instruction & 0b0000_0000_0010_0000 == 0b0000_0000_0000_0000 {
                // ADD 2 registers
                let sr2 = register_at(instruction, 0);
                hardware.registers.get(sr2)
            } else {
                // ADD register and imm5
                imm5(instruction)
            } as u32;

            hardware.registers.set(dr, value as u16);
            hardware.flags.set(value as u16);
        }, // ADD
        0x2 => {
            let dr = register_at(instruction, 9);
            let pcoffset9 = pcoffset9(instruction);

            let loc = hardware.program_counter.get() as u32 + pcoffset9 as u32;
            let value = hardware.get_memory(loc as u16);

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // LD
        0x3 => {
            let sr = register_at(instruction, 9);
            let pcoffset9 = pcoffset9(instruction);

            let loc = hardware.program_counter.get() as u32 + pcoffset9 as u32;

            hardware.memory.set(loc as u16, hardware.registers.get(sr));
        }, // ST
        0x4 => {
            hardware.registers.set(7, hardware.program_counter.get());

            let loc = if instruction & 0b0000_1000_0000_0000 == 0b0000_0000_0000_0000 {
                // JSSR
                let baser = register_at(instruction, 6);
                hardware.registers.get(baser)
            } else {
                // JSR
                let pcoffset11 = pcoffset11(instruction);
                let loc = hardware.program_counter.get() as u32 + pcoffset11 as u32;
                loc as u16
            };

            hardware.program_counter.set(loc);
        }, // JSR / JSRR
        0x5 => {
            let dr = register_at(instruction, 9);
            let sr1 = register_at(instruction, 6);

            let value = hardware.registers.get(sr1) & if instruction & 0b0000_0000_0010_0000 == 0b0000_0000_0000_0000 {
                // AND 2 registers
                let sr2 = register_at(instruction, 0);
                hardware.registers.get(sr2)
            } else {
                // AND register and imm5
                imm5(instruction)
            };

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // AND
        0x6 => {
            let dr = register_at(instruction, 9);
            let baser = register_at(instruction, 6);
            let offset6 = offset6(instruction);

            let loc = hardware.registers.get(baser) as u32 + offset6 as u32;
            let value = hardware.get_memory(loc as u16).clone();

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // LDR
        0x7 => {
            let sr = register_at(instruction, 9);
            let baser = register_at(instruction, 6);
            let offset6 = offset6(instruction);

            let loc = hardware.registers.get(baser) as u32 + offset6 as u32;
            let value = hardware.registers.get(sr);

            hardware.memory.set(loc as u16, value);
        }, // STR
        0x8 => {
            // Unused in a vm.
        }, // RTI
        0x9 => {
            let dr = register_at(instruction, 9);
            let sr = register_at(instruction, 6);

            let value = !hardware.registers.get(sr);

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // NOT
        0xA => {
            let dr = register_at(instruction, 9);
            let pcoffset9 = pcoffset9(instruction);

            let loc = hardware.program_counter.get() as u32 + pcoffset9 as u32;
            let loc = hardware.get_memory(loc as u16);
            let value = hardware.get_memory(loc);

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // LDI
        0xB => {
            let sr = register_at(instruction, 9);
            let pcoffset9 = pcoffset9(instruction);

            let loc = hardware.program_counter.get() as u32 + pcoffset9 as u32;
            let loc = hardware.get_memory(loc as u16);

            let value = hardware.registers.get(sr);

            hardware.memory.set(loc, value);
        }, // STI
        0xC => {
            let baser = register_at(instruction, 6);

            hardware.program_counter.set(hardware.registers.get(baser));
        }, // JMP / RET
        0xD => {}, // reserved
        0xE => {
            let dr = register_at(instruction, 9);
            let pcoffset9 = pcoffset9(instruction);

            let value = hardware.program_counter.get() as u32 + pcoffset9 as u32;

            hardware.registers.set(dr, value as u16);
            hardware.flags.set(value as u16);
        }, // LEA
        0xF => traps::process(instruction, hardware), // TRAP
        i @ _  => panic!("unknown instruction: {:#06b}", i),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::setup_default_test;

    #[test]
    fn add() {
        let mut hardware = setup_default_test();
        hardware.registers.set(2, 15);
        hardware.registers.set(3, 15);
        process(0b0001_0010_1000_0011u16, &mut hardware);

        assert!(hardware.registers.get(1) == 30);
        assert!(hardware.flags.is_positive());

        let mut hardware = setup_default_test();
        hardware.registers.set(2, 10);
        process(0b0001_0010_1011_0001u16, &mut hardware);

        assert!(hardware.registers.get(1) as i16 == -5);
        assert!(hardware.flags.is_negative());
    }

    #[test]
    fn and() {
        let mut hardware = setup_default_test();
        hardware.registers.set(2, 0b0000_1100_1111_0000);
        hardware.registers.set(3, 0b0000_1111_0011_0000);
        process(0b0101_0010_1000_0011u16, &mut hardware);

        assert!(hardware.registers.get(1) == 0b0000_1100_0011_0000);
        assert!(hardware.flags.is_positive());

        let mut hardware = setup_default_test();
        hardware.registers.set(2, 0b1111_1111_0000_0000);
        process(0b0101_0010_1011_0001u16, &mut hardware);

        assert_eq!(hardware.registers.get(1), 0b1111_1111_0000_0000);
        assert!(hardware.flags.is_negative());
    }

    #[test]
    fn br() {
        let mut hardware = setup_default_test();
        hardware.flags.set_zero();
        process(0b0000_1100_0000_0010, &mut hardware);

        assert_eq!(hardware.program_counter.get(), 0x3002);

        let mut hardware = setup_default_test();
        process(0b0000_0010_0000_0010, &mut hardware);

        assert_eq!(hardware.program_counter.get(), 0x3000);
    }

    #[test]
    fn jmp() {
        let mut hardware = setup_default_test();
        hardware.registers.set(2, 0x3002);
        process(0b1100_0000_1000_0000, &mut hardware);

        assert_eq!(hardware.program_counter.get(), 0x3002);
    }
    #[test]
    fn ret() {
        let mut hardware = setup_default_test();
        hardware.registers.set(7, 0x3002);
        process(0b1100_0001_1100_0000, &mut hardware);

        assert_eq!(hardware.program_counter.get(), 0x3002);
    }

    #[test]
    fn jsr() {
        let mut hardware = setup_default_test();
        process(0b0100_1000_0000_0010, &mut hardware);

        assert_eq!(hardware.program_counter.get(), 0x3002);
        assert_eq!(hardware.registers.get(7), 0x3000);
    }
    #[test]
    fn jsrr() {
        let mut hardware = setup_default_test();
        hardware.registers.set(2, 0x3002);
        process(0b0100_0000_1000_0000, &mut hardware);

        assert_eq!(hardware.program_counter.get(), 0x3002);
        assert_eq!(hardware.registers.get(7), 0x3000);
    }

    #[test]
    fn not() {
        let mut hardware = setup_default_test();
        hardware.registers.set(2, 0b1111_0000_0000_1111);
        process(0b1001_0010_1011_1111, &mut hardware);

        assert_eq!(hardware.registers.get(1), 0b0000_1111_1111_0000);
        assert!(hardware.flags.is_positive());
    }

    #[test]
    fn st() {
        let mut hardware = setup_default_test();
        hardware.registers.set(2, 0b0000_1111_1111_0000);
        process(0b0011_0100_0000_0010, &mut hardware);

        assert_eq!(hardware.memory.get(0x3002), 0b0000_1111_1111_0000);
    }

    #[test]
    fn sti() {
        let mut hardware = setup_default_test();
        hardware.registers.set(2, 0b0000_1111_1111_0000);
        hardware.memory.set(0x3002, 0b0011_0000_1000_0000);
        process(0b1011_0100_0000_0010, &mut hardware);

        assert_eq!(hardware.memory.get(0x3080), 0b0000_1111_1111_0000);
    }

    #[test]
    fn str() {
        let mut hardware = setup_default_test();
        hardware.registers.set(2, 0b0000_1111_1111_0000);
        hardware.registers.set(3, 0x307F);
        process(0b0111_0100_1100_0001, &mut hardware);

        assert_eq!(hardware.memory.get(0x3080), 0b0000_1111_1111_0000);
    }

    #[test]
    fn ld() {
        let mut hardware = setup_default_test();
        hardware.memory.set(0x3002, 0b0000_1111_1111_0000);
        process(0b0010_0010_0000_0010, &mut hardware);

        assert_eq!(hardware.registers.get(1), 0b0000_1111_1111_0000);
        assert!(hardware.flags.is_positive());
    }

    #[test]
    fn ldi() {
        let mut hardware = setup_default_test();
        hardware.memory.set(0x3000, 0b0011_0000_0000_0010);
        hardware.memory.set(0x3002, 0b0000_1111_1111_0000);
        process(0b1010_0010_0000_0000, &mut hardware);

        assert_eq!(hardware.registers.get(1), 0b0000_1111_1111_0000);
        assert!(hardware.flags.is_positive());
    }

    #[test]
    fn ldr() {
        let mut hardware = setup_default_test();
        hardware.registers.set(2, 0x3001);
        hardware.memory.set(0x3002, 0b0000_1111_1111_0000);
        process(0b0110_0010_1000_0001, &mut hardware);

        assert_eq!(hardware.registers.get(1), 0b0000_1111_1111_0000);
        assert!(hardware.flags.is_positive());
    }

    #[test]
    fn lea() {
        let mut hardware = setup_default_test();
        process(0b1110_0010_0000_1111, &mut hardware);

        assert_eq!(hardware.registers.get(1), 0b0011_0000_0000_1111);
        assert!(hardware.flags.is_positive());
    }
}
