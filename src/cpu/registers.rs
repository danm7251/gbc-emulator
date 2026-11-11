pub struct Registers {
    pub program_counter: u16,
}

impl Registers {
    pub fn init_from_boot_state() -> Self {
        Self {
            program_counter: 0x0100,
        }
    }
}