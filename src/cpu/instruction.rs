/// Sharp SM83 instruction set.
#[derive(Debug)]
pub enum Instruction {
    NOP,
}

impl TryFrom<u8> for Instruction {
    type Error = anyhow::Error;

    fn try_from(opcode: u8) -> anyhow::Result<Self> {
        use Instruction::*;

        let result = match opcode {
            0x00 => NOP,
            _ => anyhow::bail!("Unknown Instruction")
        };

        Ok(result)
    }
}