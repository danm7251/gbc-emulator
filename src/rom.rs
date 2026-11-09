use std::{fmt::Display, fs::read, ops::Range, path::Path};

use strum::Display;

#[derive(Debug)]
pub struct RomImage {
    pub _data: Vec<u8>,
    pub header: RomHeader,
}

impl RomImage {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<RomImage> {
        let path = path.as_ref();
        let bytes = read(path)?;
        let header = RomHeader::try_from(bytes.as_slice())?;

        Ok(RomImage { _data: bytes, header })
    }
}

#[derive(Debug, Display)]
pub enum CgbSupport {
    None,
    Compatible,
    Required,
}

#[derive(Debug, Display)]
pub enum CartType {
    #[strum(to_string = "ROM only")]
    RomOnly,

    #[strum(to_string = "MBC1")]
    Mbc1,
    #[strum(to_string = "MBC1 + RAM")]
    Mbc1Ram,
    #[strum(to_string = "MBC1 + RAM + Battery")]
    Mbc1RamBattery,

    #[strum(to_string = "MBC2")]
    Mbc2,
    #[strum(to_string = "MBC2 + Battery")]
    Mbc2Battery,

    #[strum(to_string = "ROM + RAM")]
    RomRam,
    #[strum(to_string = "ROM + RAM + Battery")]
    RomRamBattery,

    #[strum(to_string = "MMM01")]
    Mmm01,
    #[strum(to_string = "MMM01 + RAM")]
    Mmm01Ram,
    #[strum(to_string = "MMM01 + RAM + Battery")]
    Mmm01RamBattery,

    #[strum(to_string = "MBC3 + Timer + Battery")]
    Mbc3TimerBattery,
    #[strum(to_string = "MBC3 + Timer + RAM + Battery")]
    Mbc3TimerRamBattery,
    #[strum(to_string = "MBC3")]
    Mbc3,
    #[strum(to_string = "MBC3 + RAM")]
    Mbc3Ram,
    #[strum(to_string = "MBC3 + RAM + Battery")]
    Mbc3RamBattery,

    #[strum(to_string = "MBC5")]
    Mbc5,
    #[strum(to_string = "MBC5 + RAM")]
    Mbc5Ram,
    #[strum(to_string = "MBC5 + RAM + Battery")]
    Mbc5RamBattery,
    #[strum(to_string = "MBC5 + Rumble")]
    Mbc5Rumble,
    #[strum(to_string = "MBC5 + Rumble + RAM")]
    Mbc5RumbleRam,
    #[strum(to_string = "MBC5 + Rumble + RAM + Battery")]
    Mbc5RumbleRamBattery,

    #[strum(to_string = "MBC6")]
    Mbc6,

    #[strum(to_string = "MBC7 + Sensor + Rumble + RAM + Battery")]
    Mbc7SensorRumbleRamBattery,

    #[strum(to_string = "Pocket Camera")]
    PocketCamera,

    #[strum(to_string = "Bandai Tama 5")]
    BandaiTama5,

    #[strum(to_string = "HuC3")]
    HuC3,

    #[strum(to_string = "HuC1 + Ram + Battery")]
    HuC1RamBattery,

    #[strum(to_string = "Unknown ({0:#04X})")]
    Unknown(u8),
}

#[derive(Debug)]
pub struct RomHeader {
    pub title: String,
    pub manufacturer_code: String,
    pub cgb_support: CgbSupport,
    pub cart_type: CartType,
}

impl TryFrom<&[u8]> for RomHeader {
    type Error = anyhow::Error;

    // Does not support legacy DMG header format.
    fn try_from(bytes: &[u8]) -> anyhow::Result<Self> {
        const HEADER_START: usize = 0x100;
        const HEADER_SIZE: usize = 0x50;
        const MIN_ROM_SIZE: usize = HEADER_START + HEADER_SIZE;

        anyhow::ensure!(
            bytes.len() >= MIN_ROM_SIZE,
            "ROM is too small to contain a complete header"
        );

        // An 11-byte field containing the game title, padded with `0x00`.
        const TITLE_ADDR: Range<usize> = 0x134..0x13F;
        let title = String::from_utf8_lossy(&bytes[TITLE_ADDR])
            .trim_end_matches('\0')
            .to_string();

        // A 4-byte manufacturer code.
        const MANUFACTURER_CODE_ADDR: Range<usize> = 0x13F..0x143;
        let manufacturer_code = String::from_utf8_lossy(&bytes[MANUFACTURER_CODE_ADDR])
            .to_string();

        // A 1-byte flag describing CGB feature support.
        const CGB_FLAG_ADDR: usize = 0x143;
        let flag = bytes[CGB_FLAG_ADDR];
        let cgb_support = match flag {
            0x80 => CgbSupport::Compatible,
            0xC0 => CgbSupport::Required,
            0x00 => CgbSupport::None,
            _ => anyhow::bail!("Invalid CGB flag! {flag:#04X}"),
        };

        const CART_TYPE_ADDR: usize = 0x147;
        let flag = bytes[CART_TYPE_ADDR];
        let cart_type = match flag {
            0x00 => CartType::RomOnly,
            0x01 => CartType::Mbc1,
            0x02 => CartType::Mbc1Ram,
            0x03 => CartType::Mbc1RamBattery,
            0x05 => CartType::Mbc2,
            0x06 => CartType::Mbc2Battery,
            0x08 => CartType::RomRam,
            0x09 => CartType::RomRamBattery,
            0x0b => CartType::Mmm01,
            0x0c => CartType::Mmm01Ram,
            0x0d => CartType::Mmm01RamBattery,
            0x0f => CartType::Mbc3TimerBattery,
            0x10 => CartType::Mbc3TimerRamBattery,
            0x11 => CartType::Mbc3,
            0x12 => CartType::Mbc3Ram,
            0x13 => CartType::Mbc3RamBattery,
            0x19 => CartType::Mbc5,
            0x1a => CartType::Mbc5Ram,
            0x1b => CartType::Mbc5RamBattery,
            0x1c => CartType::Mbc5Rumble,
            0x1d => CartType::Mbc5RumbleRam,
            0x1e => CartType::Mbc5RumbleRamBattery,
            0x20 => CartType::Mbc6,
            0x22 => CartType::Mbc7SensorRumbleRamBattery,
            0xfc => CartType::PocketCamera,
            0xfd => CartType::BandaiTama5,
            0xfe => CartType::HuC3,
            0xff => CartType::HuC1RamBattery,
            flag => CartType::Unknown(flag),
        };

        Ok(
            RomHeader {
                title,
                manufacturer_code,
                cgb_support,
                cart_type,
            }
        )
    }
}

impl std::fmt::Display for RomHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ROM Header")?;
        writeln!(f, "----------")?;

        const PADDING_LENGTH: usize = 18;
        let mut write_field = |name: &str, value: &dyn Display| -> std::fmt::Result {
            writeln!(f, "{:<PADDING_LENGTH$}: {}", name, value)
        };

        write_field("Title", &self.title)?;
        write_field("Manufacturer Code", &self.manufacturer_code)?;
        write_field("CGB Support", &self.cgb_support)?;
        write_field("Cartridge Type", &self.cart_type)?;

        Ok(())
    }
}