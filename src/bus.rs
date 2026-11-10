use std::range::RangeInclusive;

use crate::{cartridge::Cartridge};

// 16-bit address bus
// 32-bit data bus

pub struct Bus {
    pub cartridge: Cartridge, // Replace with [`Cartridge`] type.
}

impl Bus {
    // Todo: Implement once memory banking finished.
    pub fn read_bytes(&self, addr: u16) -> anyhow::Result<u8> {
        match addr {
            0x0000..0x7FFF => self.cartridge.read(addr),
            _ => anyhow::bail!("Unmapped address")
        }
    }

    // Todo: Implement once memory banking finished.
    pub fn write_bytes(&self) {
        todo!()
    }
}