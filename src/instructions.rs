use std::io::{Write, Read};

use crate::hardware::Hardware;
use crate::traps;
use crate::utils::{imm5, offset6, pcoffset9, register_at, pcoffset11};

pub fn process(instruction: u16, hardware: &mut Hardware, io: &mut (impl Read, impl Write)) {
    match instruction & 0b1111_0000_0000_0000 {
        0b0001_0000_0000_0000 => {
            let dr = register_at(instruction, 9);
            let sr1 = register_at(instruction, 6);

            let value = hardware.registers.get(sr1) + if instruction & 0b0000_0000_0010_0000 == 0b0000_0000_0000_0000 {
                // ADD 2 registers
                let sr2 = register_at(instruction, 0);
                hardware.registers.get(sr2)
            } else {
                // ADD register and imm5
                imm5(instruction)
            };

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // ADD
        0b0101_0000_0000_0000 => {
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
        0b0000_0000_0000_0000 => {
            let n = (instruction & 0b0000_1000_0000_0000) == 0b0000_1000_0000_0000;
            let z = (instruction & 0b0000_0100_0000_0000) == 0b0000_0100_0000_0000;
            let p = (instruction & 0b0000_0010_0000_0000) == 0b0000_0010_0000_0000;

            if n && hardware.flags.is_negative() || z && hardware.flags.is_zero() || p && hardware.flags.is_positive() || !n && !z && !p {
                let pcoffset9 = pcoffset9(instruction);
                let location = (hardware.program_counter.get() as i16 + pcoffset9).try_into().unwrap();

                hardware.program_counter.set(location);
            }
        }, // BR
        0b1100_0000_0000_0000 => {
            let baser = register_at(instruction, 6);
            hardware.program_counter.set(hardware.registers.get(baser) as u16);
        }, // JMP / RET
        0b0100_0000_0000_0000 => {
            hardware.registers.set(7, hardware.program_counter.get() as i16);

            if instruction & 0b0000_1000_0000_0000 == 0b0000_0000_0000_0000 {
                // JSSR
                let baser = register_at(instruction, 6);
                hardware.program_counter.set(hardware.registers.get(baser) as u16);
            } else {
                // JSR
                let pcoffset11 = pcoffset11(instruction);
                hardware.program_counter.set((hardware.program_counter.get() as i16 + pcoffset11) as u16);
            }
        }, // JSR / JSRR
        0b0010_0000_0000_0000 => {
            let dr = register_at(instruction, 9);
            let pcoffset9 = pcoffset9(instruction);

            let value = hardware.get_offset(pcoffset9);

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // LD
        0b1010_0000_0000_0000 => {
            let dr = register_at(instruction, 9);
            let pcoffset9 = pcoffset9(instruction);

            let value = hardware.memory.get(hardware.get_offset(pcoffset9) as u16);

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // LDI
        0b0110_0000_0000_0000 => {
            let dr = register_at(instruction, 9);
            let baser = register_at(instruction, 6);
            let offset6 = offset6(instruction) as i16;

            let loc = (hardware.registers.get(baser) as i16 + offset6).try_into().unwrap();

            let value = hardware.memory.get(loc);

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // LDR
        0b1110_0000_0000_0000 => {
            let dr = register_at(instruction, 9);
            let pcoffset9 = pcoffset9(instruction) as i16;

            let value = hardware.program_counter.get() as i16 + pcoffset9;

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // LEA
        0b1001_0000_0000_0000 => {
            let dr = register_at(instruction, 9);
            let sr = register_at(instruction, 6);

            let value = !hardware.registers.get(sr);

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // NOT
        0b1000_0000_0000_0000 => {
            // Unused in a vm.
        }, // RTI
        0b0011_0000_0000_0000 => {
            let sr = register_at(instruction, 9);
            let pcoffset9 = pcoffset9(instruction);

            let loc = (hardware.program_counter.get() as i16 + pcoffset9).try_into().unwrap();

            hardware.memory.set(loc, hardware.registers.get(sr));
        }, // ST
        0b1011_0000_0000_0000 => {
            let sr = register_at(instruction, 9);
            let pcoffset9 = pcoffset9(instruction);

            let loc = (hardware.program_counter.get() as i16 + pcoffset9).try_into().unwrap();
            let loc = hardware.memory.get(loc) as u16;

            hardware.memory.set(loc, hardware.registers.get(sr));
        }, // STI
        0b0111_0000_0000_0000 => {
            let sr = register_at(instruction, 9);
            let baser = register_at(instruction, 6);
            let offset6 = offset6(instruction) as i16;

            let loc = (hardware.registers.get(baser) as i16 + offset6).try_into().unwrap();

            hardware.memory.set(loc, hardware.registers.get(sr));
        }, // STR
        0b1111_0000_0000_0000 => traps::process(instruction, hardware, io), // TRAP
        0b1101_0000_0000_0000 => {}, // reserved
        i @ _  => panic!("unknown instruction: {:#06b}", i >> 12),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::setup_default_test;

    #[test]
    fn add() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(2, 15);
        hardware.registers.set(3, 15);
        process(0b0001_0010_1000_0011u16, &mut hardware, &mut io);

        assert!(hardware.registers.get(1) == 30);
        assert!(hardware.flags.is_positive());

        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(2, 10);
        process(0b0001_0010_1011_0001u16, &mut hardware, &mut io);

        assert!(hardware.registers.get(1) as i16 == -5);
        assert!(hardware.flags.is_negative());
    }

    #[test]
    fn and() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(2, 0b0000_1100_1111_0000u16 as i16);
        hardware.registers.set(3, 0b0000_1111_0011_0000u16 as i16);
        process(0b0101_0010_1000_0011u16, &mut hardware, &mut io);

        assert!(hardware.registers.get(1) == 0b0000_1100_0011_0000u16 as i16);
        assert!(hardware.flags.is_positive());

        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(2, 0b1111_1111_0000_0000u16 as i16);
        process(0b0101_0010_1011_0001u16, &mut hardware, &mut io);

        assert!(hardware.registers.get(1) as i16 == 0b1111_1111_0000_0000u16 as i16);
        assert!(hardware.flags.is_negative());
    }

    #[test]
    fn br() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.flags.set_negative();
        process(0b0000_0000_0000_0010u16, &mut hardware, &mut io);

        assert!(hardware.program_counter.get() == 0x3002);

        let (mut hardware, mut io) = setup_default_test();
        hardware.flags.set_zero();
        process(0b0000_1100_0000_0010u16, &mut hardware, &mut io);

        assert!(hardware.program_counter.get() == 0x3002);

        let (mut hardware, mut io) = setup_default_test();
        process(0b0000_0010_0000_0010u16, &mut hardware, &mut io);

        assert!(hardware.program_counter.get() == 0x3000);
    }

    #[test]
    fn jmp() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(2, 0x3002);
        process(0b1100_0000_1000_0000u16, &mut hardware, &mut io);

        assert!(hardware.program_counter.get() == 0x3002);
    }
    #[test]
    fn ret() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(7, 0x3002);
        process(0b1100_0001_1100_0000u16, &mut hardware, &mut io);

        assert!(hardware.program_counter.get() == 0x3002);
    }

    #[test]
    fn jsr() {
        let (mut hardware, mut io) = setup_default_test();
        process(0b0100_1000_0000_0010u16, &mut hardware, &mut io);

        assert!(hardware.program_counter.get() == 0x3002);
        assert!(hardware.registers.get(7) == 0x3000);
    }
    #[test]
    fn jsrr() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(2, 0x3002);
        process(0b0100_0000_1000_0000u16, &mut hardware, &mut io);

        assert!(hardware.program_counter.get() == 0x3002);
        assert!(hardware.registers.get(7) == 0x3000);
    }

    #[test]
    fn not() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(2, 0b1111_0000_0000_1111u16 as i16);
        process(0b1001_0010_1011_1111u16, &mut hardware, &mut io);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000u16 as i16);
        assert!(hardware.flags.is_positive());
    }

    #[test]
    fn st() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(2, 0b0000_1111_1111_0000u16 as i16);
        process(0b0011_0100_0000_0010u16, &mut hardware, &mut io);

        assert!(hardware.memory.get(0x3002) == 0b0000_1111_1111_0000u16 as i16);
    }

    #[test]
    fn sti() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(2, 0b0000_1111_1111_0000u16 as i16);
        hardware.memory.set(0x3002, 0b0011_0000_1000_0000u16 as i16);
        process(0b1011_0100_0000_0010u16, &mut hardware, &mut io);

        assert!(hardware.memory.get(0x3080) == 0b0000_1111_1111_0000u16 as i16);
    }

    #[test]
    fn str() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(2, 0b0000_1111_1111_0000u16 as i16);
        hardware.registers.set(3, 0x307F);
        process(0b0111_0100_1100_0001u16, &mut hardware, &mut io);

        assert!(hardware.memory.get(0x3080) == 0b0000_1111_1111_0000u16 as i16);
    }

    #[test]
    fn ld() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.memory.set(0x3002, 0b0000_1111_1111_0000u16 as i16);
        process(0b0010_0010_0000_0010u16, &mut hardware, &mut io);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000u16 as i16);
        assert!(hardware.flags.is_positive());
    }

    #[test]
    fn ldi() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.memory.set(0x3000, 0b0011_0000_0000_0010u16 as i16);
        hardware.memory.set(0x3002, 0b0000_1111_1111_0000u16 as i16);
        process(0b1010_0010_0000_0000u16, &mut hardware, &mut io);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000u16 as i16);
        assert!(hardware.flags.is_positive());
    }

    #[test]
    fn ldr() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(2, 0x3001);
        hardware.memory.set(0x3002, 0b0000_1111_1111_0000u16 as i16);
        process(0b0110_0010_1000_0001u16, &mut hardware, &mut io);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000u16 as i16);
        assert!(hardware.flags.is_positive());
    }

    #[test]
    fn lea() {
        let (mut hardware, mut io) = setup_default_test();
        process(0b1110_0010_0000_1111u16, &mut hardware, &mut io);

        assert!(hardware.registers.get(1) == 0b0011_0000_0000_1111u16 as i16);
        assert!(hardware.flags.is_positive());
    }
}
