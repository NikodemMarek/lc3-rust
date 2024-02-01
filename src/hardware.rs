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
    pub fn load(&mut self, program: &[i16]) {
        self.program_counter.set(program[0].try_into().unwrap());
        self.memory.load(self.program_counter.get(), &program[1..]);
    }

    pub fn next(&mut self) -> Option<u16> {
        if self.program_counter.get() >= memory::MEMORY_SIZE as u16 {
            return None;
        }

        let address = self.program_counter.next();
        Some(self.memory.get(address) as u16)
    }

    pub fn get_offset(&self, offset: i16) -> i16 {
        self.memory.get((self.program_counter.get() as i16 + offset).try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_program() {
        let mut hardware = Hardware::default();

        hardware.load(&[
            0x3000,
            0b1110_0010_1111_1111u16 as i16,
            0b0101_0110_1110_0000u16 as i16,
            0b0101_0100_1010_0000u16 as i16,
            0b0001_0100_1010_1100u16 as i16,
            0b0000_0100_0000_0101u16 as i16,
            0b0110_1000_0100_0000u16 as i16,
            0b0001_0110_1100_0001u16 as i16,
            0b0001_0010_0110_0001u16 as i16,
            0b0001_0100_1011_1111u16 as i16,
            0b0000_1111_1111_1010u16 as i16,
        ]);

        assert!(hardware.memory.get(0x3000) == 0b1110_0010_1111_1111u16 as i16);
        assert!(hardware.memory.get(0x3005) == 0b0110_1000_0100_0000u16 as i16);
        assert!(hardware.memory.get(0x3009) == 0b0000_1111_1111_1010u16 as i16);
        assert!(hardware.memory.get(0x300A) == 0b0000_0000_0000_0000u16 as i16);
    }
}
