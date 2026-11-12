pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

pub enum Register8 {
    B,
    C,
    D,
    E,
    H,
    L,
    A,
}

pub enum Register16 {
    BC,
    DE,
    HL,
    SP,
}

pub struct Registers {
    pub a: u8,
    pub f: Flags,

    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    sp: u16,
    pc: u16,
}

impl Registers {
    pub fn dmg_mode() -> Self {
        Self {
            a: 0x11,
            f: Flags {
                z: true,
                n: false,
                h: false,
                c: false
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
            f: Flags {
                z: true,
                n: false,
                h: false,
                c: false
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

    pub fn get_mut_r8(&mut self, r: Register8) -> &mut u8 {
        match r {
            Register8::B => &mut self.b,
            Register8::C => &mut self.c,
            Register8::D => &mut self.d,
            Register8::E => &mut self.e,
            Register8::H => &mut self.h,
            Register8::L => &mut self.l,
            Register8::A => &mut self.a,
        }
    }

    pub fn get_r16(&self, r: Register16) -> u16 {
        match r {
            Register16::BC => self.bc(),
            Register16::DE => self.de(),
            Register16::HL => self.hl(),
            Register16::SP => self.sp(),
        }
    }

    pub fn set_r16(&mut self, r: Register16, value: u16) {
        match r {
            Register16::BC => self.set_bc(value),
            Register16::DE => self.set_de(value),
            Register16::HL => self.set_hl(value),
            Register16::SP => self.set_sp(value),
        }
    }

    fn bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }

    fn set_bc(&mut self, value: u16) {
        let [high, low] = value.to_be_bytes();
        self.b = high;
        self.c = low;
    }

    fn de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }

    fn set_de(&mut self, value: u16) {
        let [high, low] = value.to_be_bytes();
        self.d = high;
        self.e = low;
    }

    fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }

    fn set_hl(&mut self, value: u16) {
        let [high, low] = value.to_be_bytes();
        self.h = high;
        self.l = low;
    }

    pub fn sp(&self) -> u16 {
        self.sp
    }

    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }


    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }
}