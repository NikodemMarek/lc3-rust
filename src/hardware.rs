use crate::{
    memory::{Memory, self},
    registers::{Registers, Flags, ProgramCounter}
};

enum MemoryMappedRegisters {
    KBSR = 0xFE00,
    KBDR = 0xFE02,
}

pub struct Hardware<R, W> {
    pub program_counter: ProgramCounter,
    pub registers: Registers,
    pub memory: Memory,
    pub flags: Flags,

    pub io: (R, W),
}
impl Default for Hardware<std::io::Stdin, std::io::Stdout> {
    fn default() -> Self {
        Hardware {
            program_counter: ProgramCounter::default(),
            registers: Registers::default(),
            memory: Memory::default(),
            flags: Flags::default(),
            io: (std::io::stdin(), std::io::stdout()),
        }
    }
}
impl<R: std::io::Read, W> Hardware<R, W> {
    #[allow(dead_code)]
    pub fn default_with_io(io: (R, W)) -> Self {
        Hardware {
            program_counter: ProgramCounter::default(),
            registers: Registers::default(),
            memory: Memory::default(),
            flags: Flags::default(),
            io,
        }
    }

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

    fn handle_keyboard(&mut self) {
        let mut buf = [0; 1];
        self.io.0.read(&mut buf).unwrap();
        let c = buf[0];
        if c == 0 {
            self.memory.set(MemoryMappedRegisters::KBSR as u16, 0);
        } else {
            self.memory.set(MemoryMappedRegisters::KBDR as u16, c as i16);
            self.memory.set(MemoryMappedRegisters::KBSR as u16, 1 << 15);
        }
    }

    pub fn get_memory(&mut self, address: u16) -> i16 {
        if address == MemoryMappedRegisters::KBSR as u16 {
            self.handle_keyboard();
        }

        self.memory.get(address)
    }
    pub fn get_memory_with_offset(&mut self, offset: i16) -> i16 {
        self.get_memory((self.program_counter.get() as i16 + offset).try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

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

    #[test]
    fn handle_keyboard() {
        let mut hardware = utils::setup_test_with_input("H");
        hardware.handle_keyboard();

        assert_eq!(hardware.memory.get(MemoryMappedRegisters::KBDR as u16), 'H' as i16);
        assert_eq!(hardware.memory.get(MemoryMappedRegisters::KBSR as u16), 1 << 15);
    }
}
