use crate::{cartridge::Cartridge};

/// Provides the memory interface between the CPU and the system's components.
///
/// The bus routes memory accesses to the appropriate component based on the
/// requested address. It is responsible for address decoding, while individual
/// components handle the behaviour of their respective memory regions.
pub struct Bus {
    pub cartridge: Cartridge,
}

impl Bus {
    /// Reads a byte from the specified memory address.
    pub fn read_bytes(&self, addr: u16) -> anyhow::Result<u8> {
        match addr {
            0x0000..=0x7FFF => self.cartridge.read(addr),
            _ => anyhow::bail!("[Bus] : Read to unimplemented address {addr:#04X} attempted"),
        }
    }

    /// Writes a byte to the specified memory address.
    pub fn write_bytes(&self, addr: u16, data: u8) -> anyhow::Result<()> {
        match addr {
            0x0000..=0x7FFF => anyhow::bail!("Bus Error: write to read-only address {addr:#04X} attempted"),
            _ => anyhow::bail!("[Bus] : Write to unimplemented address {addr:#04X} attempted"),
        }
    }
}