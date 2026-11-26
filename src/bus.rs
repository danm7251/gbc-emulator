use anyhow::{anyhow, bail};

use crate::{cartridge::Cartridge, io_registers::{IORegisters, WBK}};

const WRAM_START: u16 = 0xC000;
const WRAM_SIZE: u16 = 0x1000;
const NUM_WRAM_BANKS: usize = 8;

/// Provides the memory interface between the CPU and the system's components.
///
/// The bus routes memory accesses to the appropriate component based on the
/// requested address. It is responsible for address decoding, while individual
/// components handle the behaviour of their respective memory regions.
pub struct Bus {
    pub cartridge: Cartridge,
    cgb_mode: bool,
    wram: [[u8; WRAM_SIZE as usize]; NUM_WRAM_BANKS],
    io_registers: IORegisters,
    hram: [u8; 0x7F],
    interrupt_enable: u8,
}

impl Bus {
    pub fn init(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            cgb_mode: false, // Shouldn't be hardcoded
            wram: [[0; WRAM_SIZE as usize]; NUM_WRAM_BANKS],
            io_registers: IORegisters::dmg_mode(),
            hram: [0; 0x7F],
            interrupt_enable: 0x00,
        }
    }

    /// Reads a byte from the specified memory address.
    pub fn read(&self, addr: u16) -> anyhow::Result<u8> {
        match addr {
            0x0000..=0x7FFF => self.cartridge.read(addr),
            0x8000..=0x9FFF => {
                let e = anyhow!("Read at unimplemented VRAM address {addr:#06X} attempted");
                tracing::error!("{e}");
                Err(e)
            },
            0xA000..=0xBFFF => {
                let e = anyhow!("Read at unimplemented ERAM address {addr:#06X} attempted");
                tracing::error!("{e}");
                Err(e)
            },
            0xC000..=0xDFFF => self.read_wram(addr),
            0xFE00..=0xFE9F => {
                let e = anyhow!("Read at unimplemented OAM address {addr:#06X} attempted");
                tracing::error!("{e}");
                Err(e)
            },
            0xFF00..=0xFF7F => self.io_registers.read(addr),
            0xFF80..=0xFFFE => self.read_hram(addr),
            0xFFFF => {
                let value = self.interrupt_enable;
                tracing::info!("[IE Read][{value:#08b}]");
                Ok(value)
            },
            _ => {
                let e = anyhow!("Read to unimplemented address {addr:#06X} attempted");
                tracing::error!("{e}");
                Err(e)
            },
        }
    }

    /// Writes a byte to the specified memory address.
    pub fn write(&mut self, addr: u16, data: u8) -> anyhow::Result<()> {
        match addr {
            0x0000..=0x7FFF => {
                let e = anyhow!("Write at read-only address {addr:#06X} attempted");
                tracing::error!("{e}");
                Err(e)
            },
            0x8000..=0x9FFF => {
                let e = anyhow!("Write at unimplemented VRAM address {addr:#06X} attempted");
                tracing::error!("{e}");
                Err(e)
            },
            0xA000..=0xBFFF => {
                let e = anyhow!("Write at unimplemented ERAM address {addr:#06X} attempted");
                tracing::error!("{e}");
                Err(e)
            },
            0xC000..=0xDFFF => self.write_wram(addr, data),
            0xFE00..=0xFE9F => {
                let e = anyhow!("Write at unimplemented OAM address {addr:#06X} attempted");
                tracing::error!("{e}");
                Err(e)
            },
            0xFF00..=0xFF7F => self.io_registers.write(addr, data),
            0xFF80..=0xFFFE => self.write_hram(addr, data),
            0xFFFF => {
                self.interrupt_enable = data;
                tracing::info!("[IE Write][{data:#08b}]");
                Ok(())
            },
            _ => {
                let e = anyhow!("Write at unimplemented address {addr:#06X} attempted");
                tracing::error!("{e}");
                Err(e)
            }
        }
    }

    #[tracing::instrument(skip(self))]
    fn read_wram(&self, addr: u16) -> anyhow::Result<u8> {
        match addr {
            0xC000..=0xCFFF => {
                tracing::info!("[Bank 0][{addr:#06X}]");
                Ok(self.wram[0][(addr - WRAM_START) as usize])
            },
            0xD000..=0xDFFF => {
                let wbk: u8;
                if !self.cgb_mode {
                    wbk = 1;
                } else {
                    wbk = self.io_registers.read(WBK)?;
                }

                tracing::info!("[Bank {wbk}][{addr:#06X}]");
                Ok(self.wram[wbk as usize + 1][(addr - (WRAM_START + WRAM_SIZE)) as usize])
            },
            _ => {
                let err_msg = format!("Invalid address {addr:#06X} encountered in WRAM register read handler");
                tracing::error!("{err_msg}");
                bail!(err_msg)
            }
        }
    }

    #[tracing::instrument(skip(self, addr, data))]
    fn write_wram(&mut self, addr: u16, data: u8) -> anyhow::Result<()> {
        match addr {
            0xC000..=0xCFFF => {
                tracing::info!("[Bank 0][{data:#04X} @ {addr:#06X}]");
                Ok(self.wram[0][(addr - WRAM_START) as usize] = data)
            },
            0xD000..=0xDFFF => {
                let wbk: u8;
                if !self.cgb_mode {
                    wbk = 1;
                } else {
                    wbk = self.io_registers.read(WBK)?;
                }

                tracing::info!("[Bank {wbk}][{data:#04X} @ {addr:#06X}]");
                Ok(self.wram[wbk as usize + 1][(addr - (WRAM_START + WRAM_SIZE)) as usize] = data)
            },
            _ => {
                let err_msg = format!("Invalid address {addr:#06X} encountered in WRAM write handler");
                tracing::error!("{err_msg}");
                bail!(err_msg)
            }
        }
    }

    #[tracing::instrument(skip(self))]
    fn read_hram(&self, addr: u16) -> anyhow::Result<u8> {
        match addr {
            0xFF80..=0xFFFE => {
                tracing::info!("[{addr:#06X}]");
                Ok(self.hram[(addr - 0xFF80) as usize])
            },
            _ => {
                let err_msg = format!("Invalid address {addr:#06X} encountered in HRAM read handler");
                tracing::error!("{err_msg}");
                bail!(err_msg)
            }
        }
    }

    #[tracing::instrument(skip(self, addr, data))]
    fn write_hram(&mut self, addr: u16, data: u8) -> anyhow::Result<()> {
        match addr {
            0xFF80..=0xFFFE => {
                tracing::info!("[{data:#04X} @ {addr:#06X}]");
                Ok(self.hram[(addr - 0xFF80) as usize] = data)
            },
            _ => {
                let err_msg = format!("Invalid address {addr:#06X} encountered in WRAM write handler");
                tracing::error!("{err_msg}");
                bail!(err_msg)
            }
        }
    }
}