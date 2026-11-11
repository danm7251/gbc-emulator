use num_enum::TryFromPrimitive;

/// Sharp SM83 instruction set.
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
pub enum Instruction {
    /// Does nothing.
    NOP = 0x00,

    // Stops the CPU until an interrupt occurs.
    STOP = 0x10,

    // Increments a double register by one.
    INC_BC = 0x03,
    INC_DE = 0x13,
    INC_HL = 0x23,
    INC_SP = 0x33,

    // Increments a single register by one.
    INC_B = 0x04,
    INC_D = 0x14,
    INC_H = 0x24,

    // Decrements a single register by one.
    DEC_B = 0x05,
    DEC_D = 0x15,
    DEC_H = 0x25,

    // 16-bit loads.

    // Loads an immediate 16-bit value into a double register.
    LD_BC_n16 = 0x01,
    LD_DE_n16 = 0x11,
    LD_HL_n16 = 0x21,
    LD_SP_n16 = 0x31,

    // 8-bit loads.

    /// Writes an 8-bit value in a register to an address in a double register.
    LD_ADDR_BC_A = 0x02,
    LD_ADDR_DE_A = 0x12,
    LD_ADDR_HL_B = 0x70,
    LD_ADDR_HL_C = 0x71,
    LD_ADDR_HL_D = 0x72,
    LD_ADDR_HL_E = 0x73,
    LD_ADDR_HL_H = 0x74,
    LD_ADDR_HL_L = 0x75,
    LD_ADDR_HL_A = 0x77,

    // Write an 8-bit value in A to an address in HL and then increments/decrements HL.
    LD_ADDR_HL_A_INC = 0x22,
    LD_ADDR_HL_A_DEC = 0x32,

    /// Loads an immediate 8-bit value into B.
    LD_B_n8 = 0x06,

    /// Rotates the bits in A left by one.
    RLCA = 0x07,

    /// Writes the value in the SP to an immediate address.
    LD_a16_SP = 0x08,

    /// Adds the value in BC to HL.
    ADD_HL_BC = 0x09,

    /// Loads the value at the address in BC into A.
    LD_A_ADDR_BC = 0x0A,

    DEC_BC = 0x0B,
    INC_C = 0x0C,
    DEC_C = 0x0D,

    // Loads an immediate 8-bit value into C.
    LD_C_n8 = 0x0E,

    // Rotates the bits in A right by one.
    RRCA = 0x0F,

    /// Unconditionally jumps to an address.
    JP_a16 = 0xC3,
}