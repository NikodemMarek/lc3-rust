use crate::hardware::Hardware;
use crate::utils::{imm5, offset6, pcoffset9, register_at};

pub fn process(instruction: u16, hardware: &mut Hardware) {
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
        0b1000_0000_0000_0000 => {
            if instruction & 0b0000_1111_1111_1111 == 0b0000_0000_0000_0000 {
                // RTI
            } else if instruction & 0b0000_1000_0000_0000 == 0b0000_0000_0000_0000 {
                // JSSR
            } else {
                // JSR
            }
        }, // JSR / JSRR / RTI
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
        0b1001_0000_0000_0000 => {}, // NOT
        0b0011_0000_0000_0000 => {}, // ST
        0b1011_0000_0000_0000 => {}, // STI
        0b0111_0000_0000_0000 => {}, // STR
        0b1111_0000_0000_0000 => {}, // TRAP
        0b1101_0000_0000_0000 => {
            // This in not the defalt behaviour of the LC3, but it's useful for testing.
            hardware.program_counter.set(crate::memory::MEMORY_SIZE as u16);
        }, // reserved
        _ => panic!("unrecognised instruction"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run::main_loop;

    #[test]
    fn add() {
        let mut hardware = Hardware::default();
        hardware.registers.set(2, 15);
        hardware.registers.set(3, 15);
        hardware.load(&[
             0b0001_0010_1000_0011u16 as i16,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 30);
        assert!(hardware.flags.is_positive());

        let mut hardware = Hardware::default();
        hardware.registers.set(2, 10);
        hardware.load(&[
             0b0001_0010_1011_0001u16 as i16,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) as i16 == -5);
        assert!(hardware.flags.is_negative());
    }

    #[test]
    fn and() {
        let mut hardware = Hardware::default();
        hardware.registers.set(2, 0b0000_1100_1111_0000u16 as i16);
        hardware.registers.set(3, 0b0000_1111_0011_0000u16 as i16);
        hardware.load(&[
             0b0101_0010_1000_0011u16 as i16,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 0b0000_1100_0011_0000u16 as i16);
        assert!(hardware.flags.is_positive());

        let mut hardware = Hardware::default();
        hardware.registers.set(2, 0b1111_1111_0000_0000u16 as i16);
        hardware.load(&[
             0b0101_0010_1011_0001u16 as i16,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) as i16 == 0b1111_1111_0000_0000u16 as i16);
        assert!(hardware.flags.is_negative());
    }

    #[test]
    fn br() {
        // This tests if br is behaving as expected by setting a value for register, that would
        // otherwise be omitted.
        let mut hardware = Hardware::default();
        hardware.load(&[
             0b0000_0000_0000_0010u16 as i16,
             0b1101_0000_0000_0000u16 as i16, // exit
             0b0000_0000_0000_0000u16 as i16,
             0b0010_0010_0000_0001u16 as i16,
             0b1101_0000_0000_0000u16 as i16, // exit
             0b0000_1111_1111_0000u16 as i16,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000u16 as i16);

        let mut hardware = Hardware::default();
        hardware.flags.set_zero();
        hardware.load(&[
             0b0000_1100_0000_0010u16 as i16,
             0b1101_0000_0000_0000u16 as i16, // exit
             0b0000_0000_0000_0000u16 as i16,
             0b0010_0010_0000_0001u16 as i16,
             0b1101_0000_0000_0000u16 as i16, // exit
             0b0000_1111_1111_0000u16 as i16,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000u16 as i16);
    }

    #[test]
    fn jmp() {
        // This tests works the same way as br.
        let mut hardware = Hardware::default();
        hardware.registers.set(2, 0x3002);
        hardware.load(&[
             0b1100_0000_1000_0000u16 as i16,
             0b1101_0000_0000_0000u16 as i16, // exit
             0b0010_0010_0000_0001u16 as i16,
             0b1101_0000_0000_0000u16 as i16, // exit
             0b0000_1111_1111_0000u16 as i16,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000u16 as i16);
    }
    #[test]
    fn ret() {
        // This tests works the same way as br.
        let mut hardware = Hardware::default();
        hardware.registers.set(7, 0x3002);
        hardware.load(&[
             0b1100_0001_1100_0000u16 as i16,
             0b1101_0000_0000_0000u16 as i16, // exit
             0b0010_0010_0000_0001u16 as i16,
             0b1101_0000_0000_0000u16 as i16, // exit
             0b0000_1111_1111_0000u16 as i16,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000u16 as i16);
    }

    #[test]
    fn ld() {
        let mut hardware = Hardware::default();
        hardware.load(&[
             0b0010_0010_0000_0001u16 as i16,
             0b1101_0000_0000_0000u16 as i16, // exit
             0b0000_1111_1111_0000u16 as i16,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000u16 as i16);
        assert!(hardware.flags.is_positive());
    }

    #[test]
    fn ldi() {
        let mut hardware = Hardware::default();
        hardware.load(&[
             0b1010_0010_0000_0000u16 as i16,
             0b0011_0000_0000_0011u16 as i16,
             0b1101_0000_0000_0000u16 as i16, // exit
             0b0000_1111_1111_0000u16 as i16,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000u16 as i16);
        assert!(hardware.flags.is_positive());
    }

    #[test]
    fn ldr() {
        let mut hardware = Hardware::default();
        hardware.registers.set(2, 0x3001);
        hardware.load(&[
             0b0110_0010_1000_0001u16 as i16,
             0b1101_0000_0000_0000u16 as i16, // exit
             0b0000_1111_1111_0000u16 as i16,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000u16 as i16);
        assert!(hardware.flags.is_positive());

        let mut hardware = Hardware::default();
        hardware.registers.set(2, 0x3004);
        hardware.load(&[
             0b0000_1111_1111_0000u16 as i16,
             0b0110_0010_1011_1100u16 as i16,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000u16 as i16);
    }

    #[test]
    fn lea() {
        let mut hardware = Hardware::default();
        hardware.load(&[
             0b1110_0010_0000_1111u16 as i16,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 0b0011_0000_0001_0000u16 as i16);
        assert!(hardware.flags.is_positive());
    }
}
