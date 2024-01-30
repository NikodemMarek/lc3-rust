pub struct ProgramCounter(u16);
impl Default for ProgramCounter {
    fn default() -> Self {
        ProgramCounter(0x3000)
    }
}
impl ProgramCounter {
    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn next(&mut self) -> u16 {
        self.0 += 1;
        self.0 - 1
    }
}

const GENERAL_REGISTERS: usize = 8;

pub struct Registers([i16; GENERAL_REGISTERS]);
impl Default for Registers {
    fn default() -> Self {
        Registers([0; GENERAL_REGISTERS])
    }
}
impl Registers {
    pub fn get(&self, register: u16) -> i16 {
        if register >= GENERAL_REGISTERS as u16 {
            panic!("register address out of range");
        }

        self.0[register as usize]
    }
    pub fn set(&mut self, register: u16, value: i16) {
        if register >= GENERAL_REGISTERS as u16 {
            panic!("register address out of range");
        }

        self.0[register as usize] = value;
    }
}

pub struct Flags([bool; 3]);
impl Default for Flags {
    fn default() -> Self {
        Flags([false; 3])
    }
}
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

    pub fn set_negative(&mut self) {
        self.0 = [true, false, false];
    }
    pub fn set_zero(&mut self) {
        self.0 = [false, true, false];
    }
    pub fn set_positive(&mut self) {
        self.0 = [false, false, true];
    }

    pub fn set(&mut self, value: i16) {
        match value {
            ..=-1 => self.set_negative(),
            0 => self.set_zero(),
            0.. => self.set_positive(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_to_registers() {
        let mut registers = Registers::default();

        registers.set(0b0000_0000_0000_0000, 0b0000_0000_1111_0000u16 as i16);
        registers.set(0b0000_0000_0000_0101, 0b1000_1000_1000_1000u16 as i16);

        assert!(registers.get(0b0000_0000_0000_0101) == 0b1000_1000_1000_1000u16 as i16);
        assert!(registers.get(0b0000_0000_0000_0000) == 0b0000_0000_1111_0000u16 as i16);
    }

    #[test]
    fn set_flags() {
        let mut flags = Flags::default();

        flags.set(0b1111_0000_1111_0000u16 as i16);
        assert!(flags.is_negative());

        flags.set(0b0000_0000_0000_0000u16 as i16);
        assert!(flags.is_zero());

        flags.set(0b0000_1111_1111_0000u16 as i16);
        assert!(flags.is_positive());
    }
}
