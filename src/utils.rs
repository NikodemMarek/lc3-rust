pub fn imm5(value: u16) -> i16 {
    let imm5 = value & 0b0000_0000_0011_1111;
    if imm5 & 0b0000_0000_0001_0000 == 0b0000_0000_0000_0000 {
        imm5 as i16
    } else {
        (imm5 | 0b1111_1111_1110_0000) as i16
    }
}

pub fn offset6(value: u16) -> i16 {
    let offset6 = value & 0b0000_0000_0011_1111;
    if offset6 & 0b0000_0000_0010_0000 == 0b0000_0000_0000_0000 {
        offset6 as i16
    } else {
        (offset6 | 0b1111_1111_1100_0000) as i16
    }
}

pub fn pcoffset9(value: u16) -> i16 {
    let pcoffset9 = value & 0b0000_0001_1111_1111;
    if pcoffset9 & 0b0000_0001_0000_0000 == 0b0000_0000_0000_0000 {
        pcoffset9 as i16
    } else {
        (pcoffset9 | 0b1111_1110_0000_0000) as i16
    }
}
pub fn pcoffset11(value: u16) -> i16 {
    let pcoffset11 = value & 0b0000_0111_1111_1111;
    if pcoffset11 & 0b0000_0100_0000_0000 == 0b0000_0000_0000_0000 {
        pcoffset11 as i16
    } else {
        (pcoffset11 | 0b1111_1000_0000_0000) as i16
    }
}

pub fn register_at(value: u16, at: u16) -> u16 {
    (value >> at) & 0b0000_0000_0000_0111
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imm5() {
        assert!(imm5(0b0000_0000_0000_0001) as u16 == 0b0000_0000_0000_0001);
        assert!(imm5(0b0000_0000_0001_0001) as u16 == 0b1111_1111_1111_0001);
    }

    #[test]
    fn test_offset6() {
        assert!(offset6(0b0000_0000_0000_0001) as u16 == 0b0000_0000_0000_0001);
        assert!(offset6(0b0000_0000_0010_0001) as u16 == 0b1111_1111_1110_0001);
    }

    #[test]
    fn test_pcoffset9() {
        assert!(pcoffset9(0b0000_0000_0000_0001) as u16 == 0b0000_0000_0000_0001);
        assert!(pcoffset9(0b0000_0001_0000_0001) as u16 == 0b1111_1111_0000_0001);
    }
    #[test]
    fn test_pcoffset11() {
        assert!(pcoffset11(0b0000_0000_0000_0001) as u16 == 0b0000_0000_0000_0001);
        assert!(pcoffset11(0b0000_0100_0000_0001) as u16 == 0b1111_1100_0000_0001);
    }

    #[test]
    fn test_register_at() {
        assert!(register_at(0b0000_1010_0000_0000, 9) == 0b0000_0000_0000_0101);
    }
}

#[allow(dead_code)]
pub fn setup_default_test() -> crate::hardware::Hardware<&'static [u8], Vec<u8>> {
    crate::hardware::Hardware::default_with_io(("".as_bytes(), Vec::new()))
}
#[allow(dead_code)]
pub fn setup_test_with_input(input: &'static str) -> crate::hardware::Hardware<&'static [u8], Vec<u8>> {
    crate::hardware::Hardware::default_with_io((&input.as_bytes(), Vec::new()))
}
