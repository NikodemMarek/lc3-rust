use crate::{
    memory::Memory,
    registers::{Registers, Flags, ProgramCounter}
};

pub struct Hardware {
    program_counter: ProgramCounter,
    registers: Registers,
    memory: Memory,
    flags: Flags,
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
