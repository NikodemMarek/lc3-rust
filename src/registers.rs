pub struct ProgramCounter(u16);
impl Default for ProgramCounter {
    fn default() -> Self {
        ProgramCounter(0)
    }
}
impl ProgramCounter {
    fn get(&self) -> u16 {
        self.0
    }
}

pub struct Registers([u16; 8]);
impl Default for Registers {
    fn default() -> Self {
        Registers([0, 0, 0, 0, 0, 0, 0, 0])
    }
}
impl Registers {
    fn get(&self, register: u16) -> u16 {
        if register > 7 {
            panic!("register address out of range");
        }

        self.0[register as usize]
    }
    fn set(&mut self, register: u16, value: u16) {
        if register > 7 {
            panic!("register address out of range");
        }

        self.0[register as usize] = value;
    }
}

pub struct Flags([bool; 3]);
impl Flags {
    pub fn is_negative(&self) -> bool {
        self.0[0]
    }
    pub fn is_zero(&self) -> bool {
        self.0[1]
    }
    pub fn is_positive(&self) -> bool {
        self.0[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_to_registers() {
        let mut registers = Registers::default();

        registers.set(0b0000_0000_0000_0000, 0b0000_0000_1111_0000);
        registers.set(0b0000_0000_0000_0101, 0b1000_1000_1000_1000);

        assert!(registers.get(0b0000_0000_0000_0101) == 0b1000_1000_1000_1000);
        assert!(registers.get(0b0000_0000_0000_0000) == 0b0000_0000_1111_0000);
    }
}
