pub struct Flags {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

/// Representations of all 8-bit registers except the flag register.
#[derive(Clone, Copy, Debug)]
pub enum Register8 {
    B, C, D, E, H, L, A,
}

/// Representations of all 16-bit registers except the program counter.
#[derive(Clone, Copy, Debug)]
pub enum Register16 {
    BC,
    DE,
    HL,
    SP,
}

pub struct Registers {
    a: u8,
    flags: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    sp: u16,
    pc: u16,
}

impl Registers {
    pub fn dmg_mode() -> Self {
        Self {
            a: 0x11,
            flags: Flags {
                zero: true,
                subtract: false,
                half_carry: false,
                carry: false
            },

            b: 0x00, // Depends on Cartridge license codes.
            c: 0x00,
            d: 0x00,
            e: 0x08,
            h: 0x00, // Depends on the value of register B.
            l: 0x7C, // Depends on the value of register B.

            sp: 0xFFFE,
            pc: 0x0100,
        }
    }

    pub fn cgb_mode() -> Self {
        Self {
            a: 0x11,
            flags: Flags {
                zero: true,
                subtract: false,
                half_carry: false,
                carry: false
            },

            b: 0x00,
            c: 0x00,
            d: 0xFF,
            e: 0x56,
            h: 0x00,
            l: 0x0D,

            sp: 0xFFFE,
            pc: 0x100,
        }
    }

    pub fn get_z_flag(&self) -> bool {
        self.flags.zero
    }

    pub fn get_n_flag(&self) -> bool {
        self.flags.subtract
    }

    pub fn get_h_flag(&self) -> bool {
        self.flags.half_carry
    }

    pub fn get_c_flag(&self) -> bool {
        self.flags.carry
    }

    pub fn set_z_flag(&mut self, value: bool) {
        self.flags.zero = value;
    }

    pub fn set_n_flag(&mut self, value: bool) {
        self.flags.subtract = value;
    }

    pub fn set_h_flag(&mut self, value: bool) {
        self.flags.half_carry = value;
    }

    pub fn set_c_flag(&mut self, value: bool) {
        self.flags.carry = value;
    }

    pub fn get_r8(&self, r: Register8) -> u8 {
        match r {
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::H => self.h,
            Register8::L => self.l,
            Register8::A => self.a,
        }
    }

    pub fn set_r8(&mut self, r: Register8, value: u8) {
        let r_ptr = match r {
            Register8::B => &mut self.b,
            Register8::C => &mut self.c,
            Register8::D => &mut self.d,
            Register8::E => &mut self.e,
            Register8::H => &mut self.h,
            Register8::L => &mut self.l,
            Register8::A => &mut self.a,
        };

        *r_ptr = value;

        tracing::info!("[{:?} Set][{:#04X}]", r, value);
    }

    pub fn get_r16(&self, r: Register16) -> u16 {
        let [high_reg, low_reg] = match r {
            Register16::BC => [self.b, self.c],
            Register16::DE => [self.d, self.e],
            Register16::HL => [self.h, self.l],
            Register16::SP => {
                return self.sp;
            },
        };

        u16::from_be_bytes([high_reg, low_reg])
    }

    pub fn set_r16(&mut self, r: Register16, value: u16) {
        let [high_byte, low_byte] = value.to_be_bytes();

        let [high_reg, low_reg] = match r {
            Register16::BC => [&mut self.b, &mut self.c],
            Register16::DE => [&mut self.d, &mut self.e],
            Register16::HL => [&mut self.h, &mut self.l],
            Register16::SP => {
                self.sp = value;
                return;
            },
        };

        *high_reg = high_byte;
        *low_reg = low_byte;

        tracing::info!("[{:?} Set][{:#06X}]", r, value);
    }


    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
        tracing::info!("[PC Increment][{:#06X}]", self.pc);
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
        tracing::info!("[PC Set][{:#06X}]", self.pc);
    }
}