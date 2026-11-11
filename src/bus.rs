use crate::{cartridge::Cartridge};

// 16-bit address bus
// 32-bit data bus

/// Implements the memory map between the CPU and other components.
pub struct Bus {
    pub cartridge: Cartridge, // Replace with [`Cartridge`] type.
}

impl Bus {
    pub fn read_bytes(&self, addr: u16) -> anyhow::Result<u8> {
        match addr {
            0x0000..=0x7FFF => self.cartridge.read(addr),
            _ => anyhow::bail!("Bus: read to unimplemented address attempted")
        }
    }

    pub fn write_bytes(&self, addr: u16) -> anyhow::Result<()> {
        match addr {
            0x0000..=0x7FFF => anyhow::bail!("Bus Error: write to read-only address attempted"),
            _ => anyhow::bail!("Bus: read to unimplemented address attempted")
        }
    }
}