use crate::{
    bus::Bus,
    cartridge::Cartridge,
};

use super::instruction::Instruction;

pub struct Cpu {
    bus: Bus,
    program_counter: u16,
}

impl Cpu {

    /// Initialise the hardware in a post boot state.
    pub fn init(cartridge: Cartridge) -> Self {
        Cpu {
            bus: Bus { cartridge },
            program_counter: 0x0100,
        }
    }

    pub fn tick(&mut self) -> anyhow::Result<()> {
        let opcode = self.fetch()?;
        let instruction = self.decode(opcode)?;
        println!("{:?}", instruction);

        // Calculate instruction t-cycles
        Ok(())
    }

    fn fetch(&mut self) -> anyhow::Result<u8> {
        let opcode = self.bus.read_bytes(self.program_counter)?;
        self.program_counter += 1;

        Ok(opcode)
    }

    fn decode(&self, value: u8) -> anyhow::Result<Instruction> {
        let instruction = Instruction::try_from(value)?;

        Ok(instruction)
    }

    fn execute() -> anyhow::Result<()> {
        todo!()
    }
}