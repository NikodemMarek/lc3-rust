use std::io::{Write, Read};

use crate::hardware::Hardware;

pub fn process<R: Read, W: Write>(instruction: u16, hardware: &mut Hardware<R, W>) {
    match instruction & 0xFF {
        0x20 => {
            let mut buf = [0; 1];
            hardware.io.0.read_exact(&mut buf).unwrap();
            let c = buf[0] as u16;

            hardware.registers.set(0, c);
            hardware.flags.set(c);
        }, // GETC
        0x21 => {
            let c = hardware.registers.get(0) as u8;

            hardware.io.1.write_all(&[c]).unwrap();
        }, // OUT
        0x22 => {
            let string_loc = hardware.registers.get(0) as u16;
            let mut offset = 0;

            loop {
                let c = hardware.get_memory(string_loc + offset);
                if c == 0 {
                    break;
                }

                hardware.io.1.write_all(&[c as u8]).unwrap();

                offset += 1;
            }

            hardware.io.1.flush().unwrap();
        }, // PUTS
        0x23 => {
            hardware.io.1.flush().unwrap();

            let mut buf = [0; 1];
            hardware.io.0.read_exact(&mut buf).unwrap();
            let c = buf[0];

            hardware.registers.set(0, c as u16);
            hardware.flags.set(c as u16);
        }, // IN
        0x24 => {
            let string_loc = hardware.registers.get(0) as u16;
            let mut offset = 0;

            loop {
                let c = hardware.get_memory(string_loc + offset);
                if c == 0 {
                    break;
                }

                let c1: u8 = (c & 0xFF) as u8;
                hardware.io.1.write_all(&[c1]).unwrap();

                let c2: u8 = (c >> 8) as u8;
                if c2 != 0 {
                    hardware.io.1.write_all(&[c2]).unwrap();
                }

                offset += 1;
            }

            hardware.io.1.flush().unwrap();
        }, // PUTSP
        0x25 => {
            std::process::exit(1);
        }, // HALT
        i @ _  => panic!("unknown trap code: {:#010b}", i),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{setup_default_test, setup_test_with_input};

    #[test]
    fn getc() {
        let mut hardware = setup_test_with_input("Hello World!");
        process(0b0000_0000_0010_0000, &mut hardware);

        assert_eq!(hardware.registers.get(0), 'H' as u16);
        assert_eq!(hardware.flags.is_positive(), true);
    }

    #[test]
    fn out() {
        let mut hardware = setup_default_test();
        hardware.registers.set(0, 'H' as u16);
        process(0b0000_0000_0010_0001, &mut hardware);

        assert_eq!(hardware.io.1, b"H");
    }

    #[test]
    fn puts() {
        let mut hardware = setup_default_test();
        hardware.registers.set(0, 0x3100);
        hardware.memory.load(0x3100, &[
            'H' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16, ' ' as u16,
            'W' as u16, 'o' as u16, 'r' as u16, 'l' as u16, 'd' as u16, '!' as u16,
            0x0000,
        ]);

        process(0b0000_0000_0010_0010, &mut hardware);

        assert_eq!(hardware.io.1, b"Hello World!");
    }

    #[test]
    fn _in() {
        let mut hardware = setup_test_with_input("Hello World!");
        process(0b0000_0000_0010_0011, &mut hardware);

        assert_eq!(hardware.registers.get(0), 'H' as u16);
        assert_eq!(hardware.flags.is_positive(), true);
    }

    #[test]
    fn putsp() {
        let mut hardware = setup_default_test();
        hardware.registers.set(0, 0x3100);
        hardware.memory.load(0x3100, &[
            'H' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16, ' ' as u16,
            'W' as u16, 'o' as u16, 'r' as u16, 'l' as u16, 'd' as u16, '!' as u16,
            0x0000,
        ]);
        process(0b0000_0000_0010_0100, &mut hardware);

        assert_eq!(hardware.io.1, b"Hello World!");
    }
}
