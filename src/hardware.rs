use crate::{
    memory::{Memory, self},
    registers::{Registers, Flags, ProgramCounter}
};

pub struct Hardware {
    pub program_counter: ProgramCounter,
    pub registers: Registers,
    pub memory: Memory,
    pub flags: Flags,
}
impl Default for Hardware {
    fn default() -> Self {
        Hardware {
            program_counter: ProgramCounter::default(),
            registers: Registers::default(),
            memory: Memory::default(),
            flags: Flags::default(),
        }
    }
}
impl Hardware {
    pub fn load(&mut self, program: &[u16]) {
        self.memory.load(self.program_counter.get(), program);
    }

    pub fn next(&mut self) -> Option<u16> {
        if self.program_counter.get() >= memory::MEMORY_SIZE as u16 {
            return None;
        }

        let address = self.program_counter.next();
        Some(self.memory.get(address))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_program() {
        let mut hardware = Hardware::default();

        hardware.load(&[
            0b1110_0010_1111_1111,
            0b0101_0110_1110_0000,
            0b0101_0100_1010_0000,
            0b0001_0100_1010_1100,
            0b0000_0100_0000_0101,
            0b0110_1000_0100_0000,
            0b0001_0110_1100_0001,
            0b0001_0010_0110_0001,
            0b0001_0100_1011_1111,
            0b0000_1111_1111_1010,
        ]);

        assert!(hardware.memory.get(0x3000) == 0b1110_0010_1111_1111);
        assert!(hardware.memory.get(0x3005) == 0b0110_1000_0100_0000);
        assert!(hardware.memory.get(0x3009) == 0b0000_1111_1111_1010);
        assert!(hardware.memory.get(0x300A) == 0b0000_0000_0000_0000);
    }
}
