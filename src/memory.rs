const MEMORY_SIZE: usize = 65535;

struct Memory([u16; MEMORY_SIZE]);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_to_memory() {
        let mut memory = Memory::default();

        memory.set(0b0000_0000_0000_0000, 0b0000_0000_1111_0000);
        memory.set(0b0000_0000_1000_0000, 0b1000_1000_1000_1000);

        assert!(memory.get(0b0000_0000_1000_0000) == 0b1000_1000_1000_1000);
        assert!(memory.get(0b0000_0000_0000_0000) == 0b0000_0000_1111_0000);
    }
}
