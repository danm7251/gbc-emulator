use anyhow::Context;

use crate::{
    bus::Bus,
    cartridge::Cartridge,
};

use super::{
    instruction::Instruction,
    registers::Registers,
};

pub struct Cpu {
    bus: Bus,
    registers: Registers,
}

impl Cpu {

    /// Initialise the hardware in a post boot state.
    pub fn init(cartridge: Cartridge) -> Self {
        Cpu {
            bus: Bus { cartridge },
            registers: Registers::init_from_boot_state(),
        }
    }

    /// Advance the CPU by one instruction.
    pub fn tick(&mut self) -> anyhow::Result<()> {
        let opcode = self.fetch()?;
        let instruction = self.decode(opcode)?;
        println!("{:?}", instruction);

        Ok(())
    }

    fn fetch(&mut self) -> anyhow::Result<u8> {
        let opcode = self.bus.read_bytes(self.registers.program_counter)?;
        self.registers.program_counter += 1;

        Ok(opcode)
    }

    fn decode(&self, opcode: u8) -> anyhow::Result<Instruction> {
        let instruction = Instruction::try_from(opcode).with_context(|| format!("CPU: failed to decode opcode {opcode:#04X}"))?;

        Ok(instruction)
    }

    fn execute() -> anyhow::Result<()> {
        todo!()
    }
}