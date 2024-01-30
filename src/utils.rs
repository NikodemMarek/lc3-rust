pub fn pcoffset9(value: u16) -> u16 {
    let pcoffset9 = value & 0b0000_0001_1111_1111;
    if pcoffset9 & 0b0000_0001_0000_0000 == 0b0000_0000_0000_0000 {
        pcoffset9
    } else {
        pcoffset9 | 0b1111_1110_0000_0000
    }
}
pub fn pcoffset11(value: u16) -> u16 {
    let pcoffset11 = value & 0b0000_0111_1111_1111;
    if pcoffset11 & 0b0000_0100_0000_0000 == 0b0000_0000_0000_0000 {
        pcoffset11
    } else {
        pcoffset11 | 0b1111_1000_0000_0000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcoffset9() {
        assert!(pcoffset9(0b0000_0000_0000_0001) == 0b0000_0000_0000_0001);
        assert!(pcoffset9(0b0000_0001_0000_0001) == 0b1111_1111_0000_0001);
    }
    #[test]
    fn test_pcoffset11() {
        assert!(pcoffset11(0b0000_0000_0000_0001) == 0b0000_0000_0000_0001);
        assert!(pcoffset11(0b0000_0100_0000_0001) == 0b1111_1100_0000_0001);
    }
}
