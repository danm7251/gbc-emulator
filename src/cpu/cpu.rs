use anyhow::Context;

use crate::{
    bus::Bus, cartridge::{Cartridge, CgbSupport},
};

use super::{
    instruction::Instruction,
    registers::{Register16, Register8, Registers},
};

/// Emulates the Game Boy CPU, managing its registers, instruction execution
/// and interactions with the system bus.
pub struct Cpu {
    bus: Bus,
    registers: Registers,
}

impl Cpu {
    /// Initialises the CPU with the appropriate post-boot register state.
    ///
    /// Selects the DMG or CGB register configuration based on the cartridge's
    /// declared hardware compatibility.
    pub fn init(cartridge: Cartridge) -> Self {
        let registers = match cartridge.rom.header.cgb_support {
            CgbSupport::None => Registers::dmg_mode(),
            CgbSupport::Compatible | CgbSupport::Required => Registers::cgb_mode(),
        };

        let bus = Bus { cartridge };

        Cpu {
            bus,
            registers,
        }
    }

    /// Advances the CPU by fetching, decoding and executing the instruction
    /// at the current program counter.
    pub fn tick(&mut self) -> anyhow::Result<()> {
        // Cache PC value for debugging output.
        #[cfg(debug_assertions)]
        let pc = self.registers.pc();

        let opcode = self.fetch_u8()?;

        #[cfg(debug_assertions)]
        println!("[\x1b[1;32mFetched\x1b[0m] : {:#04X}", pc);

        let instruction = self.decode(opcode)?;
        self.execute(&instruction)
    }

    /// Fetches the next byte from memory and advances the program counter.
    fn fetch_u8(&mut self) -> anyhow::Result<u8> {
        let opcode = self.bus.read_bytes(self.registers.pc())?;
        self.registers.increment_pc();

        Ok(opcode)
    }

    /// Fetches the next two bytes from memory as a 16-bit little-endian value.
    fn fetch_u16(&mut self) -> anyhow::Result<u16> {
        let addr_low = self.fetch_u8()?;
        let addr_high = self.fetch_u8()?;
        let addr = ((addr_high as u16) << 8) | (addr_low as u16);

        Ok(addr)
    }

    /// Decodes an opcode into its corresponding instruction.
    fn decode(&self, opcode: u8) -> anyhow::Result<Instruction> {
        let instruction = Instruction::try_from(opcode)
            .with_context(|| format!("[CPU] : Failed to decode opcode {opcode:#04X}"))?;

        #[cfg(debug_assertions)]
        println!("[\x1b[1;34mDecoded\x1b[0m] : {instruction:?}");

        Ok(instruction)
    }

    /// Executes the specified instruction, applying its effects to the CPU
    /// registers and memory bus.
    fn execute(&mut self, instruction: &Instruction) -> anyhow::Result<()> {
        use Instruction::*;

        match instruction {
            // Miscellaneous
            NOP => Ok(()),

            // 8-bit loads from immediate value to register.
            LD_B_n8 => self.load_r8_n8(Register8::B),
            LD_C_n8 => self.load_r8_n8(Register8::C),
            LD_D_n8 => self.load_r8_n8(Register8::D),
            LD_E_n8 => self.load_r8_n8(Register8::E),
            LD_H_n8 => self.load_r8_n8(Register8::H),
            LD_L_n8 => self.load_r8_n8(Register8::L),
            LD_A_n8 => self.load_r8_n8(Register8::A),

            // 8-bit loads from memory to register.
            LD_B_ADDR_HL => self.load_r8_addr_hl(Register8::B),
            LD_C_ADDR_HL => self.load_r8_addr_hl(Register8::C),
            LD_D_ADDR_HL => self.load_r8_addr_hl(Register8::D),
            LD_E_ADDR_HL => self.load_r8_addr_hl(Register8::E),
            LD_H_ADDR_HL => self.load_r8_addr_hl(Register8::H),
            LD_L_ADDR_HL => self.load_r8_addr_hl(Register8::L),
            LD_A_ADDR_HL => self.load_r8_addr_hl(Register8::A),

            // 8-bit loads from register to memory.
            LD_ADDR_BC_A => self.load_addr_r16_a(Register16::BC),
            LD_ADDR_DE_A => self.load_addr_r16_a(Register16::DE),

            // 8-bit loads from register to memory, with increment/decrement.
            LD_ADDR_INC_HL_A => {
                let addr = self.registers.get_r16(Register16::HL);
                self.registers.set_r16(Register16::HL, addr.wrapping_add(1));
                let data = self.registers.get_r8(Register8::A);
                self.bus.write_bytes(addr, data)
            },
            LD_ADDR_DEC_HL_A => {
                let addr = self.registers.get_r16(Register16::HL);
                self.registers.set_r16(Register16::HL, addr.wrapping_sub(1));
                let data = self.registers.get_r8(Register8::A);
                self.bus.write_bytes(addr, data)
            },

            // 8-bit loads from memory to register.
            LD_A_ADDR_BC => self.load_a_addr_r16(Register16::BC),
            LD_A_ADDR_DE => self.load_a_addr_r16(Register16::DE),

            // 8-bit loads from memory to register, with increment/decrement.
            LD_A_ADDR_INC_HL => {
                let addr = self.registers.get_r16(Register16::HL);
                self.registers.set_r16(Register16::HL, addr.wrapping_add(1));
                let data = self.bus.read_bytes(addr)?;
                *self.registers.get_mut_r8(Register8::A) = data;

                Ok(())
            },
            LD_A_ADDR_DEC_HL => {
                let addr = self.registers.get_r16(Register16::HL);
                self.registers.set_r16(Register16::HL, addr.wrapping_sub(1));
                let data = self.bus.read_bytes(addr)?;
                *self.registers.get_mut_r8(Register8::A) = data;

                Ok(())
            },

            // 8-bit loads from register to memory.
            LD_ADDR_HL_B => self.load_addr_hl_r8(Register8::B),
            LD_ADDR_HL_C => self.load_addr_hl_r8(Register8::C),
            LD_ADDR_HL_D => self.load_addr_hl_r8(Register8::D),
            LD_ADDR_HL_E => self.load_addr_hl_r8(Register8::E),
            LD_ADDR_HL_H => self.load_addr_hl_r8(Register8::H),
            LD_ADDR_HL_L => self.load_addr_hl_r8(Register8::L),
            LD_ADDR_HL_A => self.load_addr_hl_r8(Register8::A),

            // 8-bit loads from register to B register.
            LD_B_B => self.load_r8_r8(Register8::B, Register8::B),
            LD_B_C => self.load_r8_r8(Register8::B, Register8::C),
            LD_B_D => self.load_r8_r8(Register8::B, Register8::D),
            LD_B_E => self.load_r8_r8(Register8::B, Register8::E),
            LD_B_H => self.load_r8_r8(Register8::B, Register8::H),
            LD_B_L => self.load_r8_r8(Register8::B, Register8::L),
            LD_B_A => self.load_r8_r8(Register8::B, Register8::A),

            // 8-bit loads from register to C register.
            LD_C_B => self.load_r8_r8(Register8::C, Register8::B),
            LD_C_C => self.load_r8_r8(Register8::C, Register8::C),
            LD_C_D => self.load_r8_r8(Register8::C, Register8::D),
            LD_C_E => self.load_r8_r8(Register8::C, Register8::E),
            LD_C_H => self.load_r8_r8(Register8::C, Register8::H),
            LD_C_L => self.load_r8_r8(Register8::C, Register8::L),
            LD_C_A => self.load_r8_r8(Register8::C, Register8::A),

            // 8-bit loads from register to D register.
            LD_D_B => self.load_r8_r8(Register8::D, Register8::B),
            LD_D_C => self.load_r8_r8(Register8::D, Register8::C),
            LD_D_D => self.load_r8_r8(Register8::D, Register8::D),
            LD_D_E => self.load_r8_r8(Register8::D, Register8::E),
            LD_D_H => self.load_r8_r8(Register8::D, Register8::H),
            LD_D_L => self.load_r8_r8(Register8::D, Register8::L),
            LD_D_A => self.load_r8_r8(Register8::D, Register8::A),

            // 8-bit loads from register to E register.      
            LD_E_B => self.load_r8_r8(Register8::E, Register8::B),
            LD_E_C => self.load_r8_r8(Register8::E, Register8::C),
            LD_E_D => self.load_r8_r8(Register8::E, Register8::D),
            LD_E_E => self.load_r8_r8(Register8::E, Register8::E),
            LD_E_H => self.load_r8_r8(Register8::E, Register8::H),
            LD_E_L => self.load_r8_r8(Register8::E, Register8::L),
            LD_E_A => self.load_r8_r8(Register8::E, Register8::A),

            // 8-bit loads from register to H register.           
            LD_H_B => self.load_r8_r8(Register8::H, Register8::B),
            LD_H_C => self.load_r8_r8(Register8::H, Register8::C),
            LD_H_D => self.load_r8_r8(Register8::H, Register8::D),
            LD_H_E => self.load_r8_r8(Register8::H, Register8::E),
            LD_H_H => self.load_r8_r8(Register8::H, Register8::H),
            LD_H_L => self.load_r8_r8(Register8::H, Register8::L),
            LD_H_A => self.load_r8_r8(Register8::H, Register8::A),

            // 8-bit loads from register to L register.
            LD_L_B => self.load_r8_r8(Register8::L, Register8::B),
            LD_L_C => self.load_r8_r8(Register8::L, Register8::C),
            LD_L_D => self.load_r8_r8(Register8::L, Register8::D),
            LD_L_E => self.load_r8_r8(Register8::L, Register8::E),
            LD_L_H => self.load_r8_r8(Register8::L, Register8::H),
            LD_L_L => self.load_r8_r8(Register8::L, Register8::L),
            LD_L_A => self.load_r8_r8(Register8::L, Register8::A),
            
            // 8-bit loads from register to A register.
            LD_A_B => self.load_r8_r8(Register8::A, Register8::B),
            LD_A_C => self.load_r8_r8(Register8::A, Register8::C),
            LD_A_D => self.load_r8_r8(Register8::A, Register8::D),
            LD_A_E => self.load_r8_r8(Register8::A, Register8::E),
            LD_A_H => self.load_r8_r8(Register8::A, Register8::H),
            LD_A_L => self.load_r8_r8(Register8::A, Register8::L),
            LD_A_A => self.load_r8_r8(Register8::A, Register8::A),

            // 16-bit loads from immediate value to register.
            LD_BC_n16 => self.load_r16_n16(Register16::BC),
            LD_DE_n16 => self.load_r16_n16(Register16::DE),
            LD_HL_n16 => self.load_r16_n16(Register16::HL),
            LD_SP_n16 => self.load_r16_n16(Register16::SP),

            // Jumps
            JP_a16 => {
                let addr = self.fetch_u16()?;
                self.registers.set_pc(addr);

                Ok(())
            }

            XOR_A_A => {
                self.registers.a = 0x00;
                self.registers.f.z = true;
                self.registers.f.n = false;
                self.registers.f.h = false;
                self.registers.f.c = false;

                Ok(())
            }

            _ => anyhow::bail!("Failed to execute instruction {:?}", instruction)
        }
    }

    fn load_r8_n8(&mut self, r: Register8) -> anyhow::Result<()> {
        let data = self.fetch_u8()?;
        let r = self.registers.get_mut_r8(r);

        *r = data;

        Ok(())
    }

    fn load_addr_r16_a(&mut self, r16: Register16) -> anyhow::Result<()> {
        let addr = self.registers.get_r16(r16);
        let data = self.registers.get_r8(Register8::A);
        self.bus.write_bytes(addr, data)
    }

    fn load_a_addr_r16(&mut self, r16: Register16) -> anyhow::Result<()> {
        let addr = self.registers.get_r16(r16);
        let data = self.bus.read_bytes(addr)?;
        let r = self.registers.get_mut_r8(Register8::A);

        *r = data;

        Ok(())
    }

    fn load_r8_addr_hl(&mut self, r: Register8) -> anyhow::Result<()> {
        let addr = self.registers.get_r16(Register16::HL);
        let data = self.bus.read_bytes(addr)?;
        let r = self.registers.get_mut_r8(r);

        *r = data;

        Ok(())
    }

    fn load_addr_hl_r8(&mut self, r: Register8) -> anyhow::Result<()> {
        let addr = self.registers.get_r16(Register16::HL);
        let data = self.registers.get_r8(r);
        self.bus.write_bytes(addr, data)
    }

    fn load_r8_r8(&mut self, r1: Register8, r2: Register8) -> anyhow::Result<()> {
        let data = self.registers.get_r8(r2);
        let r = self.registers.get_mut_r8(r1);

        *r = data;

        Ok(())
    }

    fn load_r16_n16(&mut self, r: Register16) -> anyhow::Result<()> {
        let data = self.fetch_u16()?;
        self.registers.set_r16(r, data);

        Ok(())
    }
}