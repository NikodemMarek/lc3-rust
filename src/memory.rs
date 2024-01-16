const MEMORY_SIZE: usize = 0xFFFF;

pub struct Memory([u16; MEMORY_SIZE]);
impl Default for Memory {
    fn default() -> Self {
        Memory([0; MEMORY_SIZE])
    }
}
impl Memory {
    pub fn get(&self, address: u16) -> u16 {
        self.0[address as usize]
    }
    pub fn set(&mut self, address: u16, value: u16) {
        self.0[address as usize] = value;
    }

    pub fn load(&mut self, start: u16, program: &[u16]) {
        let start = start as usize;
        if program.len() >= MEMORY_SIZE - start {
            panic!("could not load, program too big")
        }

        self.0[start..start + program.len()].copy_from_slice(program);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_to_memory() {
        let mut memory = Memory::default();

        memory.set(0x3000, 0b0000_0000_1111_0000);
        memory.set(0xF0F0, 0b1000_1000_1000_1000);

        assert!(memory.get(0xF0F0) == 0b1000_1000_1000_1000);
        assert!(memory.get(0x3000) == 0b0000_0000_1111_0000);
    }

    #[test]
    fn load_program_with_offset() {
        let mut memory = Memory::default();

        memory.load(0x3000, &[
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

        assert!(memory.get(0x3000) == 0b1110_0010_1111_1111);
        assert!(memory.get(0x3005) == 0b0110_1000_0100_0000);
        assert!(memory.get(0x3009) == 0b0000_1111_1111_1010);
        assert!(memory.get(0x300A) == 0b0000_0000_0000_0000);
    }
}
