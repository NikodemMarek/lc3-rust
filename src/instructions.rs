use crate::hardware::Hardware;
use crate::utils::{pcoffset9, offset6};

pub fn process(instruction: u16, hardware: &mut Hardware) {
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
        0b0010_0000_0000_0000 => {
            let dr = (instruction & 0b0000_1110_0000_0000) >> 9;
            let pcoffset9 = pcoffset9(instruction);

            let value = hardware.get_offset(pcoffset9);

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // LD
        0b1010_0000_0000_0000 => {
            let dr = (instruction & 0b0000_1110_0000_0000) >> 9;
            let pcoffset9 = pcoffset9(instruction);

            let value = hardware.memory.get(hardware.get_offset(pcoffset9));

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // LDI
        0b0110_0000_0000_0000 => {
            let dr = (instruction & 0b0000_1110_0000_0000) >> 9;
            let baser = (instruction & 0b0000_0001_1100_0000) >> 6;
            let offset6 = offset6(instruction) as i16;

            let loc = (hardware.registers.get(baser) as i16 + offset6).try_into().unwrap();

            let value = hardware.memory.get(loc);

            hardware.registers.set(dr, value);
            hardware.flags.set(value);
        }, // LDR
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run::main_loop;

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

    #[test]
    fn ldr() {
        let mut hardware = Hardware::default();
        hardware.load(&[
             0b0010_0100_0000_0001,
             0b0110_0010_1000_0010,
             0b0011_0000_0000_0010,
             0b0000_0000_0000_0000,
             0b0000_1111_1111_0000,
        ]);
        main_loop(&mut hardware);

        println!("eeee {}", hardware.registers.get(2));
        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000);
        assert!(hardware.flags.is_positive());

        let mut hardware = Hardware::default();
        hardware.load(&[
             0b0000_1111_1111_0000,
             0b0000_0000_0000_0000,
             0b0010_0100_0000_0001,
             0b0110_0010_1011_1100,
             0b0011_0000_0000_0100,
        ]);
        main_loop(&mut hardware);

        assert!(hardware.registers.get(1) == 0b0000_1111_1111_0000);
    }
}
