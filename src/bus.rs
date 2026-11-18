use anyhow::{anyhow, bail};

use crate::{cartridge::Cartridge};

const WRAM_START: u16 = 0xC000;
const WRAM_SIZE: u16 = 0x1000;
const NUM_WRAM_BANKS: usize = 8;
const IF: u16 = 0xFF0F;
const WBK: u16 = 0xFF70;

    pub struct IORegisters {
        interrupt_flag: u8,
        wbk: u8,
    }

    impl IORegisters {
        pub fn init() -> Self {
            Self {
                interrupt_flag: 0x00,
                wbk: 0x00,
            }
        }
    }

/// Provides the memory interface between the CPU and the system's components.
///
/// The bus routes memory accesses to the appropriate component based on the
/// requested address. It is responsible for address decoding, while individual
/// components handle the behaviour of their respective memory regions.
pub struct Bus {
    pub cartridge: Cartridge,
    wram: [[u8; WRAM_SIZE as usize]; NUM_WRAM_BANKS],
    io_registers: IORegisters,
    interrupt_enable: u8,
}

impl Bus {
    pub fn init(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            wram: [[0; WRAM_SIZE as usize]; NUM_WRAM_BANKS],
            io_registers: IORegisters::init(),
            interrupt_enable: 0x00,
        }
    }

    /// Reads a byte from the specified memory address.
    pub fn read_bytes(&self, addr: u16) -> anyhow::Result<u8> {
        match addr {
            0x0000..=0x7FFF => self.cartridge.read(addr),
            0x8000..=0x9FFF => Err(anyhow!("Read at unimplemented VRAM address {addr:#06X} attempted")),
            0xA000..=0xBFFF => Err(anyhow!("Read at unimplemented ERAM address {addr:#06X} attempted")),
            0xC000..=0xDFFF => self.read_wram(addr),
            0xFE00..=0xFE9F => Err(anyhow!("Read at unimplemented OAM address {addr:#06X} attempted")),
            0xFF00..=0xFF7F => self.read_io(addr),
            0xFF80..=0xFFFE => Err(anyhow!("Read at unimplemented HRAM address {addr:#06X} attempted")),
            0xFFFF => {
                let value = self.interrupt_enable;
                println!("[IE Read][{value:#08b}]");
                Ok(value)
            },
            _ => Err(anyhow!("Read to unimplemented address {addr:#06X} attempted")),
        }
    }

    /// Writes a byte to the specified memory address.
    pub fn write_bytes(&mut self, addr: u16, data: u8) -> anyhow::Result<()> {
        match addr {
            0x0000..=0x7FFF => Err(anyhow!("Write at read-only address {addr:#06X} attempted")),
            0x8000..=0x9FFF => Err(anyhow!("Write at unimplemented VRAM address {addr:#06X} attempted")),
            0xA000..=0xBFFF => Err(anyhow!("Write at unimplemented ERAM address {addr:#06X} attempted")),
            0xC000..=0xCFFF => Ok(self.wram[0][(addr - WRAM_START) as usize] = data),
            0xD000..=0xDFFF => self.write_wram(addr, data),
            0xFE00..=0xFE9F => Err(anyhow!("Write at unimplemented OAM address {addr:#06X} attempted")),
            0xFF00..=0xFF7F => self.write_io(addr, data),
            0xFF80..=0xFFFE => Err(anyhow!("Write at unimplemented HRAM address {addr:#06X} attempted")),
            0xFFFF => {
                self.interrupt_enable = data;
                println!("[IE Write][{data:#08b}]");
                Ok(())
            },
            _ => Err(anyhow!("Write at unimplemented address {addr:#06X} attempted")),
        }
    }

    fn read_wram(&self, addr: u16) -> anyhow::Result<u8> {
        match addr {
            0xC000..=0xCFFF => {
                println!("[WRAM Read][Bank 0][{addr:#06X}]");
                Ok(self.wram[0][(addr - WRAM_START) as usize])
            },
            0xD000..=0xDFFF => {
                let wbk = self.read_io(WBK)?;
                println!("[WRAM Read][Bank {wbk}][{addr:#06X}]");
                Ok(self.wram[wbk as usize + 1][(addr - (WRAM_START + WRAM_SIZE)) as usize])
            },
            _ => bail!("Invalid address {addr:#06X} encountered in WRAM read handler")
        }
    }

    fn write_wram(&mut self, addr: u16, data: u8) -> anyhow::Result<()> {
        match addr {
            0xC000..=0xCFFF => {
                println!("[WRAM Write][Bank 0][{addr:#06X}]");
                Ok(self.wram[0][(addr - WRAM_START) as usize] = data)
            },
            0xD000..=0xDFFF => {
                let wbk = self.read_io(WBK)?;
                println!("[WRAM Write][Bank {wbk}][{addr:#06X}]");
                Ok(self.wram[wbk as usize + 1][(addr - (WRAM_START + WRAM_SIZE)) as usize] = data)
            },
            _ => bail!("Invalid address {addr:#06X} encountered in WRAM write handler")
        }
    }

    fn read_io(&self, addr: u16) -> anyhow::Result<u8> {
        let data = match addr {
            0xFF0F => self.io_registers.interrupt_flag,
            0xFF70 => self.io_registers.wbk,
            0xFF00..0xFF7F => bail!("Read at unimplemented IO register {addr:#06X} attempted"),
            _ => bail!("Invalid address {addr:#06X} encountered in IO register read handler")
        };

        println!("[IO Register Read][{addr:#06x}]");
        
        Ok(data)
    }

    fn write_io(&mut self, addr: u16, data: u8) -> anyhow::Result<()> {
        let register_name = match addr {
            0xFF0F => { self.io_registers.interrupt_flag = data; "IF" },
            0xFF70 => { self.io_registers.wbk = data; "WBK" },
            0xFF00..0xFF7F => bail!("Write at unimplemented IO register {addr:#06X} attempted"),
            _ => bail!("Invalid address {addr:#06X} encountered in IO register write handler")
        };

        println!("[IO Register Write][{register_name}][{addr:#06X}]");
        
        Ok(())
    }
}