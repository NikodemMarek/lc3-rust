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

pub struct Registers([u16; GENERAL_REGISTERS]);
impl Default for Registers {
    fn default() -> Self {
        Registers([0; GENERAL_REGISTERS])
    }
}
impl Registers {
    pub fn get(&self, register: u16) -> u16 {
        if register >= GENERAL_REGISTERS as u16 {
            panic!("register address out of range");
        }

        self.0[register as usize]
    }
    pub fn set(&mut self, register: u16, value: u16) {
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
