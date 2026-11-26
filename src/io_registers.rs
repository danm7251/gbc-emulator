pub const JOYP: u16  = 0xFF00;
pub const SB: u16    = 0xFF01;
pub const SC: u16    = 0xFF02;
pub const DIV: u16   = 0xFF04;
pub const TIMA: u16  = 0xFF05;
pub const TMA: u16   = 0xFF06;
pub const TAC: u16   = 0xFF07;
pub const IF: u16    = 0xFF0F;
pub const NR10: u16  = 0xFF10;
pub const NR11: u16  = 0xFF11;
pub const NR12: u16  = 0xFF12;
pub const NR13: u16  = 0xFF13;
pub const NR14: u16  = 0xFF14;
pub const NR21: u16  = 0xFF16;
pub const NR22: u16  = 0xFF17;
pub const NR23: u16  = 0xFF18;
pub const NR24: u16  = 0xFF19;
pub const NR30: u16  = 0xFF1A;
pub const NR31: u16  = 0xFF1B;
pub const NR32: u16  = 0xFF1C;
pub const NR33: u16  = 0xFF1D;
pub const NR34: u16  = 0xFF1E;
pub const NR41: u16  = 0xFF20;
pub const NR42: u16  = 0xFF21;
pub const NR43: u16  = 0xFF22;
pub const NR44: u16  = 0xFF23;
pub const NR50: u16  = 0xFF24;
pub const NR51: u16  = 0xFF25;
pub const NR52: u16  = 0xFF26;
pub const LCDC: u16  = 0xFF40;
pub const STAT: u16  = 0xFF41;
pub const LY: u16    = 0xFF44;
pub const SCY: u16   = 0xFF42;
pub const SCX: u16   = 0xFF43;
pub const LYC: u16   = 0xFF45;
pub const DMA: u16   = 0xFF46;
pub const BGP: u16   = 0xFF47;
pub const OBP0: u16  = 0xFF48;
pub const OBP1: u16  = 0xFF49;
pub const WY: u16    = 0xFF4A;
pub const WX: u16    = 0xFF4B;
pub const KEY0: u16  = 0xFF4C;
pub const KEY1: u16  = 0xFF4D;
pub const VBK: u16   = 0xFF4F;
pub const BOOT: u16  = 0xFF50;
pub const HDMA1: u16 = 0xFF51;
pub const HDMA2: u16 = 0xFF52;
pub const HDMA3: u16 = 0xFF53;
pub const HDMA4: u16 = 0xFF54;
pub const HDMA5: u16 = 0xFF55;
pub const RP: u16    = 0xFF56;
pub const BGPI: u16  = 0xFF68;
pub const BGPD: u16  = 0xFF69;
pub const OBPI: u16  = 0xFF6A;
pub const OBPD: u16  = 0xFF6B;
pub const OPRI: u16  = 0xFF6C;
pub const WBK: u16   = 0xFF70;
pub const PCM12: u16 = 0xFF76;
pub const PCM34: u16 = 0xFF77;

const CGB_ONLY: [u16; 17] = [
    KEY0,
    KEY1,
    VBK,
    HDMA1,
    HDMA2,
    HDMA3,
    HDMA4,
    HDMA5,
    RP,
    BGPI,
    BGPD,
    OBPI,
    OBPD,
    OPRI,
    WBK,
    PCM12,
    PCM34,
];

pub struct IORegisters {
    /// Describes the compatibility mode for register interactions.
    cgb_mode: bool,

    /// Selects the button groups queried by the joypad register.
    /// Its lower bits reflect the currently pressed buttons.
    joyp: u8, // COMPLEX

    /// Serial transfer data.
    /// Holds the byte currently being transmitted or received.
    sb: u8, // COMPLEX

    /// Serial transfer control.
    /// Controls transfer initiation, clock source and transfer speed.
    sc: u8, // COMPLEX

    /// Divider register, exposing the upper eight bits of an internal counter.
    /// Writing to this register resets the internal divider.
    divider: u8, // COMPLEX

    /// Timer counter, incremented according to the timer configuration.
    /// Overflow reloads the value from the timer modulo register.
    timer_counter: u8, // COMPLEX

    /// Timer modulo, providing the reload value for timer counter overflow.
    /// Copied into TIMA when the timer counter overflows.
    timer_modulo: u8,

    /// Timer control, enabling the timer and selecting its input frequency.
    /// Only bits 2–0 are writable.
    timer_control: u8, // COMPLEX

    /// Interrupt request flags for VBlank, LCD STAT, timer, serial and joypad.
    /// Writing clears or sets individual interrupt requests.
    interrupt_flag: u8, // COMPLEX

    /// Sweep control for audio channel 1.
    /// Configures frequency sweep pace, direction and shift.
    nr10: u8, // COMPLEX

    /// Duty cycle and sound length for audio channel 1.
    /// The upper bits select waveform duty and the lower six bits set length.
    nr11: u8, // COMPLEX

    /// Volume envelope for audio channel 1.
    /// Controls initial volume, envelope direction and sweep period.
    nr12: u8, // COMPLEX

    /// Frequency low byte for audio channel 1.
    /// Combined with NR14 to determine the channel frequency.
    nr13: u8, // COMPLEX

    /// Frequency high bits, length enable and trigger for audio channel 1.
    /// Writing bit 7 triggers the channel.
    nr14: u8, // COMPLEX

    /// Duty cycle and sound length for audio channel 2.
    /// The upper bits select waveform duty and the lower six bits set length.
    nr21: u8, // COMPLEX

    /// Volume envelope for audio channel 2.
    /// Controls initial volume, envelope direction and sweep period.
    nr22: u8, // COMPLEX

    /// Frequency low byte for audio channel 2.
    /// Combined with NR24 to determine the channel frequency.
    nr23: u8, // COMPLEX

    /// Frequency high bits, length enable and trigger for audio channel 2.
    /// Writing bit 7 triggers the channel.
    nr24: u8, // COMPLEX

    /// Enables or disables the wave channel's DAC.
    /// Bit 7 controls DAC power.
    nr30: u8, // COMPLEX

    /// Sound length for the wave channel.
    /// Determines the duration of wave playback.
    nr31: u8, // COMPLEX

    /// Output level for the wave channel.
    /// Selects mute, full volume or reduced volume.
    nr32: u8, // COMPLEX

    /// Frequency low byte for the wave channel.
    /// Combined with NR34 to determine the channel frequency.
    nr33: u8, // COMPLEX

    /// Frequency high bits, length enable and trigger for the wave channel.
    /// Writing bit 7 triggers the channel.
    nr34: u8, // COMPLEX

    /// Sound length for the noise channel.
    /// Determines the duration of noise playback.
    nr41: u8, // COMPLEX

    /// Volume envelope for the noise channel.
    /// Controls initial volume, envelope direction and sweep period.
    nr42: u8, // COMPLEX

    /// Noise polynomial counter.
    /// Controls the noise frequency and waveform characteristics.
    nr43: u8, // COMPLEX

    /// Length enable and trigger for the noise channel.
    /// Writing bit 7 triggers the channel.
    nr44: u8, // COMPLEX

    /// Master volume and VIN routing for both audio outputs.
    /// Controls left and right output volume independently.
    nr50: u8, // COMPLEX

    /// Routes individual audio channels to the left and right outputs.
    /// Each bit enables a channel on one of the two outputs.
    nr51: u8, // COMPLEX

    /// Audio master control and channel status.
    /// Controls APU power and reports which channels are active.
    nr52: u8, // COMPLEX

    /// LCD control register.
    /// Controls LCD enable, background, window and object rendering.
    lcdc: u8, // COMPLEX

    /// LCD status register.
    /// Contains PPU mode, coincidence status and LCD STAT interrupt enables.
    stat: u8, // COMPLEX

    /// Vertical background scrolling offset.
    /// Determines the vertical position of the background viewport.
    scy: u8,

    /// Horizontal background scrolling offset.
    /// Determines the horizontal position of the background viewport.
    scx: u8,

    /// Current PPU scanline.
    /// Updated by the PPU and read-only to the CPU.
    ly: u8, // COMPLEX

    /// LY comparison value.
    /// Compared against LY to generate the STAT coincidence condition.
    lyc: u8, // COMPLEX

    /// OAM DMA source address.
    /// Writing starts a DMA transfer from the selected page of memory.
    dma: u8, // COMPLEX

    /// Background palette data.
    /// Maps the four DMG background colour indices to display shades.
    bgp: u8,

    /// Object palette 0 data.
    /// Maps the four DMG object colour indices to display shades.
    obp0: u8,

    /// Object palette 1 data.
    /// Maps the four DMG object colour indices to display shades.
    obp1: u8,

    /// Window vertical position.
    /// Determines the scanline at which the window begins rendering.
    wy: u8,

    /// Window horizontal position.
    /// Determines the horizontal position at which the window begins rendering.
    wx: u8,

    /// CGB compatibility mode configuration.
    /// Controls CPU and hardware compatibility behaviour on CGB systems.
    key0: u8, // COMPLEX

    /// CGB speed switch control.
    /// Controls double-speed mode preparation and reports current speed.
    key1: u8, // COMPLEX

    /// CGB VRAM bank selection.
    /// Selects which VRAM bank is accessible through the CPU address space.
    vbk: u8, // COMPLEX

    /// Boot ROM disable register.
    /// Writing a non-zero value permanently unmaps the boot ROM until reset.
    boot_rom_disable: u8, // COMPLEX

    /// CGB VRAM DMA source address high byte.
    /// Together with HDMA2, specifies the transfer source.
    hdma1: u8, // COMPLEX

    /// CGB VRAM DMA source address low byte.
    /// Only the upper four bits are used.
    hdma2: u8, // COMPLEX

    /// CGB VRAM DMA destination address high byte.
    /// Together with HDMA4, specifies the VRAM destination.
    hdma3: u8, // COMPLEX

    /// CGB VRAM DMA destination address low byte.
    /// Only the upper four bits are used.
    hdma4: u8, // COMPLEX

    /// CGB VRAM DMA control.
    /// Starts general-purpose or HBlank-synchronised DMA transfers.
    hdma5: u8, // COMPLEX

    /// CGB infrared communication control.
    /// Controls the infrared LED and receiver.
    rp: u8, // COMPLEX

    /// CGB background palette index.
    /// Selects a colour palette RAM entry and controls index auto-increment.
    bgpi: u8, // COMPLEX

    /// CGB background palette data.
    /// Provides access to the selected background palette RAM entry.
    bgpd: u8, // COMPLEX

    /// CGB object palette index.
    /// Selects an object palette RAM entry and controls index auto-increment.
    obpi: u8, // COMPLEX

    /// CGB object palette data.
    /// Provides access to the selected object palette RAM entry.
    obpd: u8, // COMPLEX

    /// CGB object priority mode.
    /// Selects between DMG-style and CGB-style object priority behaviour.
    opri: u8, // COMPLEX

    /// CGB WRAM bank selection.
    /// Selects the switchable WRAM bank mapped into 0xD000–0xDFFF.
    wbk: u8, // COMPLEX

    /// CGB audio channel 1 and 2 PCM sample data.
    /// Read-only register exposing the current channel output levels.
    pcm12: u8, // COMPLEX

    /// CGB audio channel 3 and 4 PCM sample data.
    /// Read-only register exposing the current channel output levels.
    pcm34: u8, // COMPLEX
}

impl IORegisters {
    pub fn dmg_mode() -> Self {
        Self {
            cgb_mode: false,

            joyp: 0xCF,

            sb: 0x00,
            sc: 0x7E,

            // DMG post-boot divider value.
            divider: 0xAB,
            timer_counter: 0x00,
            timer_modulo: 0x00,
            timer_control: 0xF8,

            interrupt_flag: 0xE1,

            nr10: 0x80,
            nr11: 0xBF,
            nr12: 0xF3,
            nr13: 0xFF,
            nr14: 0xBF,

            nr21: 0x3F,
            nr22: 0x00,
            nr23: 0xFF,
            nr24: 0xBF,

            nr30: 0x7F,
            nr31: 0xFF,
            nr32: 0x9F,
            nr33: 0xFF,
            nr34: 0xBF,

            nr41: 0xFF,
            nr42: 0x00,
            nr43: 0x00,
            nr44: 0xBF,

            nr50: 0x77,
            nr51: 0xF3,
            nr52: 0xF1,

            lcdc: 0x91,
            stat: 0x85,
            scy: 0x00,
            scx: 0x00,
            ly: 0x00,
            lyc: 0x00,
            dma: 0xFF,

            bgp: 0xFC,

            // DMG boot ROM leaves these registers uninitialised.
            // Zero is a deterministic emulator default.
            obp0: 0x00,
            obp1: 0x00,

            wy: 0x00,
            wx: 0x00,

            // These registers are unavailable in DMG mode.
            key0: 0xFF,
            key1: 0xFF,
            vbk: 0xFF,

            boot_rom_disable: 0x01,

            hdma1: 0xFF,
            hdma2: 0xFF,
            hdma3: 0xFF,
            hdma4: 0xFF,
            hdma5: 0xFF,

            rp: 0xFF,

            bgpi: 0xFF,
            bgpd: 0xFF,
            obpi: 0xFF,
            obpd: 0xFF,

            opri: 0xFF,
            wbk: 0xFF,

            pcm12: 0xFF,
            pcm34: 0xFF,
        }
    }

    #[tracing::instrument(skip(self, addr), name="io_read")]
    pub fn read(&self, addr: u16) -> anyhow::Result<u8> {
        let (data, register_name) = match addr {
            JOYP => (self.joyp, "JOYP"),
            SB => (self.sb, "SB"),
            SC => (self.sc, "SC"),
            DIV => (self.divider, "DIV"),
            TIMA => (self.timer_counter, "TIMA"),
            TMA => (self.timer_modulo, "TMA"),
            TAC => (self.timer_control, "TAC"),
            IF => (self.interrupt_flag, "IF"),
            NR10 => (self.nr10, "NR10"),
            NR11 => (self.nr11, "NR11"),
            NR12 => (self.nr12, "NR12"),
            NR13 => (self.nr13, "NR13"),
            NR14 => (self.nr14, "NR14"),
            NR21 => (self.nr21, "NR21"),
            NR22 => (self.nr22, "NR22"),
            NR23 => (self.nr23, "NR23"),
            NR24 => (self.nr24, "NR24"),
            NR30 => (self.nr30, "NR30"),
            NR31 => (self.nr31, "NR31"),
            NR32 => (self.nr32, "NR32"),
            NR33 => (self.nr33, "NR33"),
            NR34 => (self.nr34, "NR34"),
            NR41 => (self.nr41, "NR41"),
            NR42 => (self.nr42, "NR42"),
            NR43 => (self.nr43, "NR43"),
            NR44 => (self.nr44, "NR44"),
            NR50 => (self.nr50, "NR50"),
            NR51 => (self.nr51, "NR51"),
            NR52 => (self.nr52, "NR52"),
            LCDC => (self.lcdc, "LCDC"),
            STAT => (self.stat, "STAT"),
            LY => (self.ly, "LY"),
            SCY => (self.scy, "SCY"),
            SCX => (self.scx, "SCX"),
            LYC => (self.lyc, "LYC"),
            DMA => (self.dma, "DMA"),
            BGP => (self.bgp, "BGP"),
            OBP0 => (self.obp0, "OBP0"),
            OBP1 => (self.obp1, "OBP1"),
            WY => (self.wy, "WY"),
            WX => (self.wx, "WX"),
            KEY0 => (self.key0, "KEY0"),
            KEY1 => (self.key1, "KEY1"),
            VBK => (self.vbk, "VBK"),
            BOOT => (self.boot_rom_disable, "BOOT"),
            HDMA1 => (self.hdma1, "HDMA1"),
            HDMA2 => (self.hdma2, "HDMA2"),
            HDMA3 => (self.hdma3, "HDMA3"),
            HDMA4 => (self.hdma4, "HDMA4"),
            HDMA5 => (self.hdma5, "HDMA5"),
            RP => (self.rp, "RP"),
            BGPI => (self.bgpi, "BGPI"),
            BGPD => (self.bgpd, "BGPD"),
            OBPI => (self.obpi, "OBPI"),
            OBPD => (self.obpd, "OBPD"),
            OPRI => (self.opri, "OPRI"),
            WBK => (self.wbk, "WBK"),
            PCM12 => (self.pcm12, "PCM12"),
            PCM34 => (self.pcm34, "PCM34"),
            0xFF00..0xFF7F => {
                let err_msg = format!("[Unimplemented IO register read @ {addr:#06X}]");
                tracing::error!("{err_msg}");
                anyhow::bail!(err_msg)
            },
            _ => {
                let err_msg = format!("Invalid address {addr:#06X} encountered in IO register read handler");
                tracing::error!("{err_msg}");
                anyhow::bail!(err_msg)
            }
        };

        if !self.cgb_mode && CGB_ONLY.contains(&addr) {
            tracing::warn!("[CGB Only {register_name} @ {addr:#06X} register read in DMG mode");
            return Ok(data)
        }

        tracing::info!("[{register_name} Read][{data:#04X} @ {addr:#06X}]");
        
        Ok(data)
    }

    #[tracing::instrument(skip(self, addr, data), name="io_write")]
    pub fn write(&mut self, addr: u16, data: u8) -> anyhow::Result<()> {
        if !self.cgb_mode && CGB_ONLY.contains(&addr) {
            tracing::warn!("[CGB Only IO register write in DMG mode attempted @ {addr:#06X}");
            return Ok(())
        }

        let register_name = match addr {
            JOYP => { self.joyp = data; "JOYP" },
            SB => { self.sb = data; "SB" },
            SC => { self.sc = data; "SC" },
            DIV => { self.divider = data; "DIV" },
            TIMA => { self.timer_counter = data; "TIMA" },
            TMA => { self.timer_modulo = data; "TMA" },
            TAC => { self.timer_control = data; "TAC" },
            IF => { self.interrupt_flag = data; "IF" },
            NR10 => { self.nr10 = data; "NR10" },
            NR11 => { self.nr11 = data; "NR11" },
            NR12 => { self.nr12 = data; "NR12" },
            NR13 => { self.nr13 = data; "NR13" },
            NR14 => { self.nr14 = data; "NR14" },
            NR21 => { self.nr21 = data; "NR21" },
            NR22 => { self.nr22 = data; "NR22" },
            NR23 => { self.nr23 = data; "NR23" },
            NR24 => { self.nr24 = data; "NR24" },
            NR30 => { self.nr30 = data; "NR30" },
            NR31 => { self.nr31 = data; "NR31" },
            NR32 => { self.nr32 = data; "NR32" },
            NR33 => { self.nr33 = data; "NR33" },
            NR34 => { self.nr34 = data; "NR34" },
            NR41 => { self.nr41 = data; "NR41" },
            NR42 => { self.nr42 = data; "NR42" },
            NR43 => { self.nr43 = data; "NR43" },
            NR44 => { self.nr44 = data; "NR44" },
            NR50 => { self.nr50 = data; "NR50" },
            NR51 => { self.nr51 = data; "NR51" },
            NR52 => { self.nr52 = data; "NR52" },
            LCDC => { self.lcdc = data; "LCDC" },
            STAT => { self.stat = data; "STAT" },
            LY => { self.ly = data; "LY" },
            SCY => { self.scy = data; "SCY" },
            SCX => { self.scx = data; "SCX" },
            LYC => { self.lyc = data; "LYC" },
            DMA => { self.dma = data; "DMA" },
            BGP => { self.bgp = data; "BGP" },
            OBP0 => { self.obp0 = data; "OBP0" },
            OBP1 => { self.obp1 = data; "OBP1" },
            WY => { self.wy = data; "WY" },
            WX => { self.wx = data; "WX" },
            KEY0 => { self.key0 = data; "KEY0" },
            KEY1 => { self.key1 = data; "KEY1" },
            VBK => { self.vbk = data; "VBK" },
            BOOT => { self.boot_rom_disable = data; "BOOT" },
            HDMA1 => { self.hdma1 = data; "HDMA1" },
            HDMA2 => { self.hdma2 = data; "HDMA2" },
            HDMA3 => { self.hdma3 = data; "HDMA3" },
            HDMA4 => { self.hdma4 = data; "HDMA4" },
            HDMA5 => { self.hdma5 = data; "HDMA5" },
            RP => { self.rp = data; "RP" },
            BGPI => { self.bgpi = data; "BGPI" },
            BGPD => { self.bgpd = data; "BGPD" },
            OBPI => { self.obpi = data; "OBPI" },
            OBPD => { self.obpd = data; "OBPD" },
            OPRI => { self.opri = data; "OPRI" },
            WBK => { self.wbk = data; "WBK" },
            PCM12 => { self.pcm12 = data; "PCM12" },
            PCM34 => { self.pcm34 = data; "PCM34" },
            0xFF00..0xFF7F => {
                let err_msg = format!("[Unimplemented IO register write @ {addr:#06X}]");
                tracing::error!("{err_msg}");
                anyhow::bail!(err_msg)
            },
            _ => {
                let err_msg = format!("Invalid address {addr:#06X} encountered in IO register write handler");
                tracing::error!("{err_msg}");
                anyhow::bail!(err_msg)
            }
        };

        tracing::info!("[{register_name} Write][{data:#04X} @ {addr:#06x}]");
        
        Ok(())
    }
}