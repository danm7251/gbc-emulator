use std::path::Path;

mod rom;

pub use rom::{
    CartType,
    CgbSupport,
    RomHeader,
    RomImage,
};

pub struct Cartridge {
    pub rom: RomImage
}

impl Cartridge {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        Ok(
            Cartridge {
                rom: RomImage::load(path)?
            }
        )
    }

    pub fn read(&self, addr: u16) -> anyhow::Result<u8> {
        if self.rom.header.cart_type != CartType::RomOnly {
            anyhow::bail!("Cartridge: unsupported cartridge type")
        }

        let data = self.rom.data[addr as usize];

        Ok(data)
    }
}