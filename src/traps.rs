use std::io::Write;

use crate::hardware::Hardware;

pub fn process(instruction: u16, hardware: &mut Hardware, output: &mut impl Write) {
    match instruction & 0b0000_0000_1111_1111 {
        0b0000_0000_0010_0000 => {}, // GETC
        0b0000_0000_0010_0001 => {}, // OUT
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
        0b0000_0000_0010_0011 => {}, // IN
        0b0000_0000_0010_0100 => {}, // PUTSP
        0b0000_0000_0010_0101 => {}, // HALT
        i @ _  => println!("unknown trap code: {:#010b}", i),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puts() {
        let mut hardware = Hardware::default();
        hardware.registers.set(0, 0x3100);
        hardware.memory.load(0x3100, &[
            'H' as i16, 'e' as i16, 'l' as i16, 'l' as i16, 'o' as i16, ' ' as i16,
            'W' as i16, 'o' as i16, 'r' as i16, 'l' as i16, 'd' as i16, '!' as i16,
            0x0000,
        ]);

        let mut out: Vec<u8> = Vec::new();
        process(0b0000_0000_0010_0010, &mut hardware, &mut out);

        assert_eq!(&out, b"Hello World!");
    }
}
