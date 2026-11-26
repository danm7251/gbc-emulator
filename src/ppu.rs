use crate::{bus::Bus, io_registers::LY};

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const SCANLINE_DOTS: usize = 456;
pub const MAX_SCANLINES: u8 = 153;

pub struct Ppu {
    cgb_mode: bool,
    dot: usize,
    scanline: u8,
    frame_buffer: [[u32; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Ppu {
    pub fn dmg_mode() -> Self {
        Self {
            cgb_mode: false,
            dot: 0,
            scanline: 0,
            frame_buffer: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    #[tracing::instrument(skip(self, bus), name="ppu")]
    pub fn tick(&mut self, bus: &mut Bus) -> anyhow::Result<()> {
        self.dot += 1;

        if self.dot >= SCANLINE_DOTS {
            self.dot = 0;

            let last_scanline = bus.read(LY)?;
            let next_scanline = if last_scanline == MAX_SCANLINES {
                0
            } else {
                last_scanline + 1
            };

            bus.write(LY, next_scanline)?;
            self.scanline = next_scanline;
        }

        tracing::info!("Scanline: {}, dot: {}", self.scanline, self.dot);

        Ok(())
    }
}