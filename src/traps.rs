use std::io::{Write, Read};

use crate::hardware::Hardware;

pub fn process(instruction: u16, hardware: &mut Hardware, (input, output): &mut (impl Read, impl Write)) {
    match instruction & 0b0000_0000_1111_1111 {
        0b0000_0000_0010_0000 => {
            let c = input.bytes().next().unwrap().unwrap();

            hardware.registers.set(0, c as i16);
            hardware.flags.set(c as i16);
        }, // GETC
        0b0000_0000_0010_0001 => {
            let c = hardware.registers.get(0);

            output.write_all(&[c as u8]).unwrap();
        }, // OUT
        0b0000_0000_0010_0010 => {
            let string_loc: u16 = hardware.registers.get(0).try_into().unwrap();
            let mut offset: u16 = 0;

            loop {
                let c = hardware.memory.get(string_loc + offset);
                if c == 0 {
                    break;
                }

                output.write_all(&[c as u8]).unwrap();

                offset += 1;
            }
        }, // PUTS
        0b0000_0000_0010_0011 => {
            let c = input.bytes().next().unwrap().unwrap();

            hardware.registers.set(0, c as i16);
            hardware.flags.set(c as i16);

            output.write_all(&[c as u8]).unwrap();
        }, // IN
        0b0000_0000_0010_0100 => {
            // FIXME: This probably does not work as intended.
            let string_loc: u16 = hardware.registers.get(0).try_into().unwrap();
            let mut offset: u16 = 0;

            loop {
                let c = hardware.memory.get(string_loc + offset);
                if c == 0 {
                    break;
                }

                let c1: u8 = (c & 0xFF) as u8;
                output.write_all(&[c1]).unwrap();

                let c2: u8 = (c >> 8) as u8;
                if c2 != 0 {
                    output.write_all(&[c2]).unwrap();
                }

                offset += 1;
            }
        }, // PUTSP
        0b0000_0000_0010_0101 => {
            std::process::exit(1);
        }, // HALT
        i @ _  => println!("unknown trap code: {:#010b}", i),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{setup_default_test, setup_test_with_input};

    #[test]
    fn getc() {
        let (mut hardware, mut io) = setup_test_with_input("Hello World!");
        process(0b0000_0000_0010_0000, &mut hardware, &mut io);

        assert_eq!(hardware.registers.get(0), 'H' as i16);
        assert_eq!(hardware.flags.is_positive(), true);
    }

    #[test]
    fn out() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(0, 'H' as i16);
        process(0b0000_0000_0010_0001, &mut hardware, &mut io);

        assert_eq!(io.1, b"H");
    }

    #[test]
    fn puts() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(0, 0x3100);
        hardware.memory.load(0x3100, &[
            'H' as i16, 'e' as i16, 'l' as i16, 'l' as i16, 'o' as i16, ' ' as i16,
            'W' as i16, 'o' as i16, 'r' as i16, 'l' as i16, 'd' as i16, '!' as i16,
            0x0000,
        ]);

        process(0b0000_0000_0010_0010, &mut hardware, &mut io);

        assert_eq!(io.1, b"Hello World!");
    }

    #[test]
    fn _in() {
        let (mut hardware, mut io) = setup_test_with_input("Hello World!");
        process(0b0000_0000_0010_0011, &mut hardware, &mut io);

        assert_eq!(hardware.registers.get(0), 'H' as i16);
        assert_eq!(hardware.flags.is_positive(), true);
        assert_eq!(io.1, b"H");
    }

    #[test]
    fn putsp() {
        let (mut hardware, mut io) = setup_default_test();
        hardware.registers.set(0, 0x3100);
        hardware.memory.load(0x3100, &[
            'H' as i16, 'e' as i16, 'l' as i16, 'l' as i16, 'o' as i16, ' ' as i16,
            'W' as i16, 'o' as i16, 'r' as i16, 'l' as i16, 'd' as i16, '!' as i16,
            0x0000,
        ]);
        process(0b0000_0000_0010_0100, &mut hardware, &mut io);

        assert_eq!(io.1, b"Hello World!");
    }
}
