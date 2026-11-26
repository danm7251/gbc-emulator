use anyhow::Context;

use crate::{
    bus::Bus, cartridge::{CgbSupport},
};

use super::{
    instruction::Instruction,
    registers::{Register16, Register8, Registers},
};

// High memory address.
const HIGH_MEMORY: u16 = 0xFF00;

#[derive(Debug, PartialEq)]
enum IMEState {
    Disabled,
    Pending,
    Enabled,
}

/// Emulates the Game Boy CPU, managing its registers, instruction execution
/// and interactions with the system bus.
pub struct Cpu {
    cycle: usize,
    pub m_cycles_elapsed: usize,
    registers: Registers,
    ime: IMEState,
}

impl Cpu {
    /// Initialises the CPU with the appropriate post-boot register state.
    ///
    /// Selects the DMG or CGB register configuration based on the cartridge's
    /// declared hardware compatibility.
    pub fn init(cgb_support: &CgbSupport) -> Self {
        let registers = match cgb_support {
            CgbSupport::None => Registers::dmg_mode(),
            CgbSupport::Compatible | CgbSupport::Required => Registers::cgb_mode(),
        };

        let ime = IMEState::Disabled;

        Cpu {
            cycle: 0,
            m_cycles_elapsed: 0,
            registers,
            ime,
        }
    }

    /// Advances the CPU by fetching, decoding and executing the instruction
    /// at the current program counter.
    #[tracing::instrument(skip(self, bus), name="cpu")]
    pub fn tick(&mut self, bus: &mut Bus) -> anyhow::Result<()> {
        self.cycle += 1;
        tracing::info!("[Cycle {}]", self.cycle);

        let opcode = self.fetch_u8(bus)?;
        let instruction = self.decode(opcode)?;
        self.execute(bus, &instruction)
    }

    /// Fetches the next byte from memory and advances the program counter.
    #[tracing::instrument(skip(self, bus))]
    fn fetch_u8(&mut self, bus: &mut Bus) -> anyhow::Result<u8> {
        let data = bus.read(self.registers.pc())?;
        tracing::info!("[Fetched {data:#04X} @ {:#06X}]", self.registers.pc());
        self.registers.increment_pc();

        Ok(data)
    }

    /// Fetches the next two bytes from memory as a 16-bit little-endian value.
    #[tracing::instrument(skip(self, bus))]
    fn fetch_u16(&mut self, bus: &mut Bus) -> anyhow::Result<u16> {
        let addr_low = self.fetch_u8(bus)?;
        let addr_high = self.fetch_u8(bus)?;
        let addr = ((addr_high as u16) << 8) | (addr_low as u16);

        Ok(addr)
    }

    /// Decodes an opcode into its corresponding instruction.
    #[tracing::instrument(skip(self, opcode))]
    fn decode(&self, opcode: u8) -> anyhow::Result<Instruction> {
        let instruction = Instruction::try_from(opcode)
            .with_context(|| format!("[CPU] : Failed to decode opcode {opcode:#04X}"))?;

        tracing::info!("[{opcode:#04X} -> {instruction:?}]");

        Ok(instruction)
    }

    /// Executes the specified instruction, applying its effects to the CPU
    /// registers and memory bus.
    #[tracing::instrument(skip(self, bus, instruction))]
    fn execute(&mut self, bus: &mut Bus, instruction: &Instruction) -> anyhow::Result<()> {
        use Instruction::*;

        match instruction {
            // Miscellaneous
            NOP => { self.m_cycles_elapsed += 1; },

            // Interrupts
            DI => {
                self.ime = IMEState::Disabled;
                self.m_cycles_elapsed += 1;
                tracing::info!("[IME Set][{:?}]", self.ime);
            },
            EI => { 
                self.ime = IMEState::Pending;
                self.m_cycles_elapsed += 1;
                tracing::info!("[IME Set][{:?}]", self.ime);

                self.tick(bus)?;

                // If no other instruction (such as DI) has modified the IME, then enable it.
                if self.ime == IMEState::Pending {
                    self.ime = IMEState::Enabled;
                    tracing::info!("[IME Set][{:?}]", self.ime);
                }
            },

            // 8-bit loads from immediate value to register.
            LD_B_n8 => self.load_r8_n8(bus, Register8::B)?,
            LD_C_n8 => self.load_r8_n8(bus, Register8::C)?,
            LD_D_n8 => self.load_r8_n8(bus, Register8::D)?,
            LD_E_n8 => self.load_r8_n8(bus, Register8::E)?,
            LD_H_n8 => self.load_r8_n8(bus, Register8::H)?,
            LD_L_n8 => self.load_r8_n8(bus, Register8::L)?,
            LD_A_n8 => self.load_r8_n8(bus, Register8::A)?,

            // 8-bit loads from memory to register.
            LD_B_ADDR_HL => self.load_r8_addr_hl(bus, Register8::B)?,
            LD_C_ADDR_HL => self.load_r8_addr_hl(bus, Register8::C)?,
            LD_D_ADDR_HL => self.load_r8_addr_hl(bus, Register8::D)?,
            LD_E_ADDR_HL => self.load_r8_addr_hl(bus, Register8::E)?,
            LD_H_ADDR_HL => self.load_r8_addr_hl(bus, Register8::H)?,
            LD_L_ADDR_HL => self.load_r8_addr_hl(bus, Register8::L)?,
            LD_A_ADDR_HL => self.load_r8_addr_hl(bus, Register8::A)?,

            // 8-bit loads from register to memory.
            LD_ADDR_BC_A => self.load_addr_r16_a(bus, Register16::BC)?,
            LD_ADDR_DE_A => self.load_addr_r16_a(bus, Register16::DE)?,

            // 8-bit loads from register to memory, with increment/decrement.
            LD_ADDR_INC_HL_A => {
                let addr = self.registers.get_r16(Register16::HL);
                self.registers.set_r16(Register16::HL, addr.wrapping_add(1));
                let data = self.registers.get_r8(Register8::A);
                bus.write(addr, data)?;
                self.m_cycles_elapsed += 2;
            },
            LD_ADDR_DEC_HL_A => {
                let addr = self.registers.get_r16(Register16::HL);
                self.registers.set_r16(Register16::HL, addr.wrapping_sub(1));
                let data = self.registers.get_r8(Register8::A);
                bus.write(addr, data)?;
                self.m_cycles_elapsed += 2;
            },

            // 8-bit loads from memory to register.
            LD_A_ADDR_BC => self.load_a_addr_r16(bus, Register16::BC)?,
            LD_A_ADDR_DE => self.load_a_addr_r16(bus, Register16::DE)?,

            // 8-bit loads from memory to register, with increment/decrement.
            LD_A_ADDR_INC_HL => {
                let addr = self.registers.get_r16(Register16::HL);
                self.registers.set_r16(Register16::HL, addr.wrapping_add(1));
                let data = bus.read(addr)?;
                self.registers.set_r8(Register8::A, data);
                self.m_cycles_elapsed += 2;
            },
            LD_A_ADDR_DEC_HL => {
                let addr = self.registers.get_r16(Register16::HL);
                self.registers.set_r16(Register16::HL, addr.wrapping_sub(1));
                let data = bus.read(addr)?;
                self.registers.set_r8(Register8::A, data);
                self.m_cycles_elapsed += 2;
            },

            // 8-bit loads from register to memory.
            LD_ADDR_HL_B => self.load_addr_hl_r8(bus, Register8::B)?,
            LD_ADDR_HL_C => self.load_addr_hl_r8(bus, Register8::C)?,
            LD_ADDR_HL_D => self.load_addr_hl_r8(bus, Register8::D)?,
            LD_ADDR_HL_E => self.load_addr_hl_r8(bus, Register8::E)?,
            LD_ADDR_HL_H => self.load_addr_hl_r8(bus, Register8::H)?,
            LD_ADDR_HL_L => self.load_addr_hl_r8(bus, Register8::L)?,
            LD_ADDR_HL_A => self.load_addr_hl_r8(bus, Register8::A)?,

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

            // 8-bit loads to high memory.
            LDH_a8_A => {
                let value = self.registers.get_r8(Register8::A);
                let addr_offset = self.fetch_u8(bus)?;
                let addr = HIGH_MEMORY + addr_offset as u16;
                bus.write(addr, value)?;
                self.m_cycles_elapsed += 3;
            }
            LDH_A_a8 => {
                let addr_offset = self.fetch_u8(bus)?;
                let addr = HIGH_MEMORY + addr_offset as u16;
                let value = bus.read(addr)?;
                self.registers.set_r8(Register8::A, value);
                self.m_cycles_elapsed += 3;
            }
            LDH_ADDR_C_A => {
                let value = self.registers.get_r8(Register8::A);
                let addr_offset = self.registers.get_r8(Register8::C);
                let addr = HIGH_MEMORY + addr_offset as u16;
                bus.write(addr, value)?;
                self.m_cycles_elapsed += 2;
            }
            LDH_A_ADDR_C => {
                let addr_offset = self.registers.get_r8(Register8::A);
                let addr = HIGH_MEMORY + addr_offset as u16;
                let value = self.registers.get_r8(Register8::C);
                bus.write(addr, value)?;
                self.m_cycles_elapsed += 2;
            }

            // 16-bit loads from immediate value to register.
            LD_BC_n16 => self.load_r16_n16(bus, Register16::BC)?,
            LD_DE_n16 => self.load_r16_n16(bus, Register16::DE)?,
            LD_HL_n16 => self.load_r16_n16(bus, Register16::HL)?,
            LD_SP_n16 => self.load_r16_n16(bus, Register16::SP)?,

            // 8-bit register increments.
            INC_B => self.inc_r8(Register8::B),
            INC_C => self.inc_r8(Register8::C),
            INC_D => self.inc_r8(Register8::D),
            INC_E => self.inc_r8(Register8::E),
            INC_H => self.inc_r8(Register8::H),
            INC_L => self.inc_r8(Register8::L),
            INC_A => self.inc_r8(Register8::A),

            // 8-bit memory increment.
            INC_ADDR_HL => {
                let addr = self.registers.get_r16(Register16::HL);
                let data = bus.read(addr)?;
                bus.write(addr, data.wrapping_add(1))?;
                self.m_cycles_elapsed += 3;
            },

            // 8-bit register decrements.
            DEC_B => self.dec_r8(Register8::B),
            DEC_C => self.dec_r8(Register8::C),
            DEC_D => self.dec_r8(Register8::D),
            DEC_E => self.dec_r8(Register8::E),
            DEC_H => self.dec_r8(Register8::H),
            DEC_L => self.dec_r8(Register8::L),
            DEC_A => self.dec_r8(Register8::A),

            // 8-bit memory decrement.
            DEC_ADDR_HL => {
                let addr = self.registers.get_r16(Register16::HL);
                let data = bus.read(addr)?;
                bus.write(addr, data.wrapping_sub(1))?;
                self.m_cycles_elapsed += 3;
            },

            // 16-bit register increments.
            INC_BC => self.inc_r16(Register16::BC),
            INC_DE => self.inc_r16(Register16::DE),
            INC_HL => self.inc_r16(Register16::HL),
            INC_SP => self.inc_r16(Register16::SP),

            // 16-bit register decrements.
            DEC_BC => self.dec_r16(Register16::BC),
            DEC_DE => self.dec_r16(Register16::DE),
            DEC_HL => self.dec_r16(Register16::HL),
            DEC_SP => self.dec_r16(Register16::SP),

            // 16-bit register addition.
            ADD_HL_BC => self.add_hl_r16(Register16::BC),
            ADD_HL_DE => self.add_hl_r16(Register16::DE),
            ADD_HL_HL => self.add_hl_r16(Register16::HL),
            ADD_HL_SP => self.add_hl_r16(Register16::SP),

            // 8-bit comparisons.
            CP_A_B => self.compare_a_r8(Register8::B),
            CP_A_C => self.compare_a_r8(Register8::C),
            CP_A_D => self.compare_a_r8(Register8::D),
            CP_A_E => self.compare_a_r8(Register8::E),
            CP_A_H => self.compare_a_r8(Register8::H),
            CP_A_L => self.compare_a_r8(Register8::L),
            CP_A_A => self.compare_a_r8(Register8::A),

            // 8-bit memory comparison.
            CP_A_ADDR_HL => {
                let addr = self.registers.get_r16(Register16::HL);
                let value = bus.read(addr)?;
                self.compare_a_u8(value);
                self.m_cycles_elapsed += 2;
            }

            // 8-bit immediate value comparison.
            CP_A_n8 => {
                let value = self.fetch_u8(bus)?;
                self.compare_a_u8(value);
                self.m_cycles_elapsed += 2;
            }

            // Relative jumps.
            JR_e8    => self.jump_relative(bus, Condition::None)?,
            JR_NZ_e8 => self.jump_relative(bus, Condition::ZeroUnset)?,
            JR_Z_e8  => self.jump_relative(bus, Condition::ZeroSet)?,
            JR_NC_e8 => self.jump_relative(bus, Condition::CarryUnset)?,
            JR_C_e8  => self.jump_relative(bus, Condition::CarrySet)?,

            // Direct jumps.
            JP_a16    => self.jump_direct(bus, Condition::None)?,
            JP_NZ_a16 => self.jump_direct(bus, Condition::ZeroUnset)?,
            JP_Z_a16  => self.jump_direct(bus, Condition::ZeroSet)?,
            JP_NC_a16 => self.jump_direct(bus, Condition::CarryUnset)?,
            JP_C_a16  => self.jump_direct(bus, Condition::CarrySet)?,
            JP_HL     => {
                let value = self.registers.get_r16(Register16::HL);
                self.registers.set_pc(value);
            },

            XOR_A_A => {
                self.registers.set_r8(Register8::A, 0x00);
                self.registers.set_z_flag(true);
                self.registers.set_n_flag(false);
                self.registers.set_h_flag(false);
                self.registers.set_c_flag(false);
            }

            _ => {
                let e = format!("Failed to execute instruction {instruction:?}");
                tracing::error!(e);
                anyhow::bail!(e)
            }
        }

        Ok(())
    }

    fn load_r8_n8(&mut self, bus: &mut Bus, r: Register8) -> anyhow::Result<()> {
        let data = self.fetch_u8(bus)?;
        self.registers.set_r8(r, data);
        self.m_cycles_elapsed += 2;

        Ok(())
    }

    fn load_addr_r16_a(&mut self, bus: &mut Bus, r16: Register16) -> anyhow::Result<()> {
        let addr = self.registers.get_r16(r16);
        let data = self.registers.get_r8(Register8::A);
        bus.write(addr, data)?;
        self.m_cycles_elapsed += 2;

        Ok(())
    }

    fn load_a_addr_r16(&mut self, bus: &mut Bus, r16: Register16) -> anyhow::Result<()> {
        let addr = self.registers.get_r16(r16);
        let data = bus.read(addr)?;
        self.registers.set_r8(Register8::A, data);
        self.m_cycles_elapsed += 2;

        Ok(())
    }

    fn load_r8_addr_hl(&mut self, bus: &mut Bus, r: Register8) -> anyhow::Result<()> {
        let addr = self.registers.get_r16(Register16::HL);
        let data = bus.read(addr)?;
        self.registers.set_r8(r, data);
        self.m_cycles_elapsed += 2;

        Ok(())
    }

    fn load_addr_hl_r8(&mut self, bus: &mut Bus, r: Register8) -> anyhow::Result<()> {
        let addr = self.registers.get_r16(Register16::HL);
        let data = self.registers.get_r8(r);
        bus.write(addr, data)?;
        self.m_cycles_elapsed += 2;

        Ok(())
    }

    fn load_r8_r8(&mut self, r1: Register8, r2: Register8) {
        let data = self.registers.get_r8(r2);
        self.registers.set_r8(r1, data);
        self.m_cycles_elapsed += 1;
    }

    fn load_r16_n16(&mut self, bus: &mut Bus, r: Register16) -> anyhow::Result<()> {
        let data = self.fetch_u16(bus)?;
        self.registers.set_r16(r, data);
        self.m_cycles_elapsed += 3;

        Ok(())
    }

    fn inc_r8(&mut self, r: Register8) {
        // Checks if the lower nibble will overflow.
        if (self.registers.get_r8(r) & 0x0F) == 0x0F {
            self.registers.set_h_flag(true);
        }

        let new_value = self.registers.get_r8(r).wrapping_add(1);
        if new_value == 0 { self.registers.set_z_flag(true); }
        self.registers.set_n_flag(false);
        self.registers.set_r8(r, new_value);
        self.m_cycles_elapsed += 1;
    }

    fn dec_r8(&mut self, r: Register8) {
        // Checks if the lower nibble will underflow.
        if (self.registers.get_r8(r) & 0x0F) == 0 {
            self.registers.set_h_flag(true);
        }

        let new_value = self.registers.get_r8(r).wrapping_sub(1);
        if new_value == 0 { self.registers.set_z_flag(true); }
        self.registers.set_n_flag(true);
        self.registers.set_r8(r, new_value);
        self.m_cycles_elapsed += 1;
    }

    fn inc_r16(&mut self, r: Register16) {
        let new_value = self.registers.get_r16(r).wrapping_add(1);
        self.registers.set_r16(r, new_value);
        self.m_cycles_elapsed += 2;
    }

    fn dec_r16(&mut self, r: Register16) {
        let new_value = self.registers.get_r16(r).wrapping_sub(1);
        self.registers.set_r16(r, new_value);
        self.m_cycles_elapsed += 2;
    }

    fn add_hl_r16(&mut self, r: Register16) {
        let rhs = self.registers.get_r16(r);
        let value = self.registers.get_r16(Register16::HL).wrapping_add(rhs);
        self.registers.set_r16(Register16::HL, value);
        self.m_cycles_elapsed += 2;
    }

    fn compare_a_r8(&mut self, r: Register8) {
        let r = self.registers.get_r8(r);
        self.compare_a_u8(r);
        self.m_cycles_elapsed += 1;
    }

    fn compare_a_u8(&mut self, value: u8) {
        let a = self.registers.get_r8(Register8::A);
        let result = a.wrapping_sub(value);
        self.registers.set_z_flag(result == 0);
        self.registers.set_n_flag(true);
        self.registers.set_h_flag((a & 0x0F) < (value & 0x0F));
        self.registers.set_c_flag(a < value);
    }

    fn jump_relative(&mut self, bus: &mut Bus, condition: Condition) -> anyhow::Result<()> {
        let offset = self.fetch_u8(bus)? as i8;
        let valid = self.eval_flag_condition(condition);
        if valid {
            self.registers.set_pc(self.registers.pc().wrapping_add_signed(offset as i16));
        }

        Ok(())
    }

    fn jump_direct(&mut self,  bus: &mut Bus, condition: Condition) -> anyhow::Result<()> {
        let addr = self.fetch_u16(bus)?;
        let valid = self.eval_flag_condition(condition);
        if valid { self.registers.set_pc(addr); }

        Ok(())
    }

    fn eval_flag_condition(&self, condition: Condition) -> bool {
        match condition {
            Condition::None => true,
            Condition::ZeroSet => self.registers.get_z_flag(),
            Condition::ZeroUnset => !self.registers.get_z_flag(),
            Condition::CarrySet => self.registers.get_c_flag(),
            Condition::CarryUnset => !self.registers.get_c_flag(),
        }
    }
}

enum Condition {
    None,
    ZeroSet,
    ZeroUnset,
    CarrySet,
    CarryUnset,
}