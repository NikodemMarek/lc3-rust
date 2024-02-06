pub fn sign_extend(mut value: u16, bit_count: u8) -> u16 {
    if (value >> (bit_count - 1)) & 1 != 0 {
        value |= 0xFFFF << bit_count;
    }
    value
}

pub fn imm5(value: u16) -> u16 {
    sign_extend(value & 0b0000_0000_0001_1111, 5)
}

pub fn offset6(value: u16) -> u16 {
    sign_extend(value & 0b0000_0000_0011_1111, 6)
}

pub fn pcoffset9(value: u16) -> u16 {
    sign_extend(value & 0b0000_0001_1111_1111, 9)
}
pub fn pcoffset11(value: u16) -> u16 {
    sign_extend(value & 0b0000_0111_1111_1111, 11)
}

pub fn register_at(value: u16, at: u16) -> u16 {
    (value >> at) & 0b0000_0000_0000_0111
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_extend() {
        assert_eq!(sign_extend(0b0000_0000_0000_0001, 16), 0b0000_0000_0000_0001);
        assert_eq!(sign_extend(0b0000_0000_0000_0001, 4), 0b0000_0000_0000_0001);
    }

    #[test]
    fn test_imm5() {
        assert_eq!(imm5(0b0000_0000_0000_0001), 0b0000_0000_0000_0001);
        assert_eq!(imm5(0b0000_0000_0001_0001), 0b1111_1111_1111_0001);
    }

    #[test]
    fn test_offset6() {
        assert_eq!(offset6(0b0000_0000_0000_0001), 0b0000_0000_0000_0001);
        assert_eq!(offset6(0b0000_0000_0010_0001), 0b1111_1111_1110_0001);
    }

    #[test]
    fn test_pcoffset9() {
        assert_eq!(pcoffset9(0b0000_0000_0000_0001), 0b0000_0000_0000_0001);
        assert_eq!(pcoffset9(0b0000_0001_0000_0001), 0b1111_1111_0000_0001);
    }
    #[test]
    fn test_pcoffset11() {
        assert_eq!(pcoffset11(0b0000_0000_0000_0001), 0b0000_0000_0000_0001);
        assert_eq!(pcoffset11(0b0000_0100_0000_0001), 0b1111_1100_0000_0001);
    }

    #[test]
    fn test_register_at() {
        assert_eq!(register_at(0b0000_1010_0000_0000, 9), 0b0000_0000_0000_0101);
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
