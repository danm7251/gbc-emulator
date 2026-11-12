use num_enum::TryFromPrimitive;

/// Sharp SM83 instruction set.
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
pub enum Instruction {
    /// Does nothing.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    NOP = 0x00,

    /// Loads an immediate unsigned 16-bit value into BC.
    /// - M-cycles: 3
    /// - T-cycles: 12
    /// - Flags: none
    LD_BC_n16 = 0x01,

    /// Loads the value in A into an address at BC.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_ADDR_BC_A = 0x02,

    /// Increments BC by one.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    INC_BC = 0x03,

    /// Increments B by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=0, H=*
    INC_B = 0x04,

    /// Decrements B by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=1, H=*
    DEC_B = 0x05,

    /// Loads an immediate unsigned 8-bit value into B.
    /// - M-cycles: 2
    /// - T-cycles: 8
    /// - Flags: none
    LD_B_n8 = 0x06,

    /// Shifts the bits in A excluding the carry flag left by one circularly.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=0, N=0, H=0, C=*
    RLCA = 0x07,

    /// Loads the value in SP into an immediate address.
    /// - M-cycles: 3
    /// - T-cycles: 20
    /// - Flags: none
    LD_a16_SP = 0x08,

    /// Adds BC to HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: N=0, H=*, C=*
    ADD_HL_BC = 0x09,

    /// Loads the value at the address in BC into A.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_A_ADDR_BC = 0x0A,

    /// Decrements BC by one.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    DEC_BC = 0x0B,

    /// Increments C by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=0. H=*
    INC_C = 0x0C,

    /// Decrements C by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=1, H=*
    DEC_C = 0x0D,

    /// Loads an unsigned 8-bit value into C.
    /// - M-cycles: 2
    /// - T-cycles: 8
    /// - Flags: none
    LD_C_n8 = 0x0E,

    /// Shifts the bits in A excluding the carry flag right by one circularly.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=0, N=0, H=0, C=*
    RRCA = 0x0F,

    /// Takes an unsigned 8-bit value and halts the CPU until an input interrupt.
    /// - M-cycles: 2
    /// - T-cycles: 4
    /// - Flags: none
    STOP = 0x10,

    /// Loads an immediate unsigned 16-bit value into DE.
    /// M-cycles: 3
    /// T-cycles: 12
    /// - Flags: none
    LD_DE_n16 = 0x11,

    /// Loads the value in A into an address at DE.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_ADDR_DE_A = 0x12,

    /// Increments DE by one.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    INC_DE = 0x13,

    /// Increments D by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=0, H=*
    INC_D = 0x14,

    /// Decrements D by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=1, H=*
    DEC_D = 0x15,

    /// Loads an immediate unsigned 8-bit value into D.
    /// - M-cycles: 2
    /// - T-cycles: 8
    /// - Flags: none
    LD_D_n8 = 0x16,

    /// Shifts the bits in A including the carry flag left by one circularly.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=0, N=0, H=0, C=1
    RLA = 0x17,

    /// Unconditionally relative jump, shifts the PC by a signed 8-bit value.
    /// - M-cycles: 2
    /// - T-cycles: 12
    /// - Flags: none
    JR_e8 = 0x18,

    /// Adds the value in DE to HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: N=0, H=*, C=*
    ADD_HL_DE = 0x19,

    /// Loads the value at the address in DE into A.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_A_ADDR_DE = 0x1A,

    /// Decrements DE by one.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    DEC_DE = 0x1B,

    /// Increments E by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=0, H=*
    INC_E = 0x1C,

    /// Decrements E by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=1, H=*
    DEC_E = 0x1D,

    /// Loads an unsigned 8-bit value into E.
    /// - M-cycles: 2
    /// - T-cycles: 8
    /// - Flags: none
    LD_E_n8 = 0x1E,

    /// Shifts the bits in A including the carry flag right by one circularly.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=0, N=0, H=0, C=*
    RRA = 0x1F,

    /// Conditional relative jump, shifts the PC by a signed 8-bit value if the zero flag is not set.
    /// - M-cycles: 2
    /// - T-cycles: 12 (taken) / 8 (not taken)
    /// - Flags: none
    JR_NZ_e8 = 0x20,

    /// Loads an immediate unsigned 16-bit value into HL.
    /// - M-cycles: 3
    /// - T-cycles: 12
    /// - Flags: none
    LD_HL_n16 = 0x21,

    /// Loads the value in A into the address in HL and then increments HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_ADDR_INC_HL_A = 0x22,

    /// Increments HL by one.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    INC_HL = 0x23,

    /// Increments H by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=0, H=*
    INC_H = 0x24,

    /// Decrements H by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=1, H=*
    DEC_H = 0x25,

    /// Loads an immediate unsigned 8-bit value into H.
    /// - M-cycles: 2
    /// - T-cycles: 8
    /// - Flags: none
    LD_H_n8 = 0x26,

    /// Corrects A after a BCD addition or subtraction.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, H=0, C=*
    DAA = 0x27,

    /// Conditional relative jump, shifts the PC by a signed 8-bit value if the zero flag is set.
    /// - M-cycles: 2
    /// - T-cycles: 12 (taken) / 8 (not taken)
    /// - Flags: none
    JR_Z_e8 = 0x28,

    /// Adds the value in HL to HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: N=0, H=*, C=*
    ADD_HL_HL = 0x29,

    /// Loads the value at the address in HL into the A and then increments HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_A_ADDR_INC_HL = 0x2A,

    /// Decrements HL by one.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    DEC_HL = 0x2B,

    /// Increments L by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=0, H=*
    INC_L = 0x2C,

    /// Decrements L by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=1, H=*
    DEC_L = 0x2D,

    /// Loads an immediate unsigned 8-bit value into L.
    /// - M-cycles: 2
    /// - T-cycles: 8
    /// - Flags: none
    LD_L_n8 = 0x2E,

    /// Inverts all bits in A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: N=1, H=1
    CPL = 0x2F,

    /// Conditional relative jump, shifts the PC by a signed 8-bit value if the carry flag is not set.
    /// - M-cycles: 2
    /// - T-cycles: 12 (taken) / 8 (not taken)
    /// - Flags: none
    JR_NC_e8 = 0x30,

    /// Loads an immediate unsigned 16-bit value into SP.
    /// - M-cycles: 3
    /// - T-cycles: 12
    /// - Flags: none
    LD_SP_n16 = 0x31,

    /// Loads the value in A into the address in HL and then decrements HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_ADDR_DEC_HL_A = 0x32,

    /// Increments SP by one.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    INC_SP = 0x33,

    /// Increments the value at the address in HL by one.
    /// - M-cycles: 1
    /// - T-cycles: 12
    /// - Flags: Z=*, N=0, H=*
    INC_ADDR_HL = 0x34,

    /// Decrements the value at the address in HL by one.
    /// - M-cycles: 1
    /// - T-cycles: 12
    /// - Flags: Z=*, N=1, H=*
    DEC_ADDR_HL = 0x35,

    /// Loads an immediate unsigned 8-bit value into the address in HL.
    /// - M-cycles: 2
    /// - T-cycles: 12
    /// - Flags: none
    LD_ADDR_HL_n8 = 0x36,

    /// Sets the carry flag to one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: N=0, H=0, C=1
    SCF = 0x37,

    /// Conditional relative jump, shifts the PC by an immediate signed 8-bit value if the carry flag is set.
    /// - M-cycles: 2
    /// - T-cycles: 12 (taken) / 8 (not taken)
    /// - Flags: none
    JR_C_e8 = 0x38,

    /// Adds the value in SP to HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: N=0, H=*, C=*
    ADD_HL_SP = 0x39,

    /// Loads the value at the address in HL into the A and then decrements HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_A_ADDR_DEC_HL = 0x3A,

    /// Decrements SP by one.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    DEC_SP = 0x3B,

    /// Increments A by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=0, H=*
    INC_A = 0x3C,

    /// Decrements A by one.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: Z=*, N=1, H=*
    DEC_A = 0x3D,

    /// Loads an immediate unsigned 8-bit value into A.
    /// - M-cycles: 2
    /// - T-cycles: 8
    /// - Flags: none
    LD_A_n8 = 0x3E,

    /// Inverts the carry flag.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: N=0, H=0, C=*
    CCF = 0x3F,

    /// Loads the value in B into B.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_B_B = 0x40,

    /// Loads the value in C into B.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_B_C = 0x41,

    /// Loads the value in D into B.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_B_D = 0x42,

    /// Loads the value in E into B.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_B_E = 0x43,

    /// Loads the value in H into B.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_B_H = 0x44,

    /// Loads the value in L into B.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_B_L = 0x45,

    /// Loads the value at the address in HL into B.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_B_ADDR_HL = 0x46,

    /// Loads the value in A into B.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_B_A = 0x47,

    /// Loads the value in B into C.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_C_B = 0x48,

    /// Loads the value in C into C.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_C_C = 0x49,

    /// Loads the value in D into C.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_C_D = 0x4A,

    /// Loads the value in E into C.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_C_E = 0x4B,

    /// Loads the value in H into C.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_C_H = 0x4C,

    /// Loads the value in L into C.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_C_L = 0x4D,

    /// Loads the value at the address in HL into C.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_C_ADDR_HL = 0x4E,

    /// Loads the value in A into C.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_C_A = 0x4F,

    /// Loads the value in B into D.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_D_B = 0x50,

    /// Loads the value in C into D.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_D_C = 0x51,

    /// Loads the value in D into D.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_D_D = 0x52,

    /// Loads the value in E into D.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_D_E = 0x53,

    /// Loads the value in H into D.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_D_H = 0x54,

    /// Loads the value in L into D.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_D_L = 0x55,

    /// Loads the value at the address in HL into D.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_D_ADDR_HL = 0x56,

    /// Loads the value in A into D.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_D_A = 0x57,

    /// Loads the value in B into E.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_E_B = 0x58,

    /// Loads the value in C into E.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_E_C = 0x59,

    /// Loads the value in D into E.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_E_D = 0x5A,

    /// Loads the value in E into E.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_E_E = 0x5B,

    /// Loads the value in H into E.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_E_H = 0x5C,

    /// Loads the value in L into E.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_E_L = 0x5D,

    /// Loads the value at the address in HL into E.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_E_ADDR_HL = 0x5E,

    /// Loads the value in A into E.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_E_A = 0x5F,

    /// Loads the value in B into H.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_H_B = 0x60,

    /// Loads the value in C into H.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_H_C = 0x61,

    /// Loads the value in D into H.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_H_D = 0x62,

    /// Loads the value in E into H.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_H_E = 0x63,

    /// Loads the value in H into H.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_H_H = 0x64,

    /// Loads the value in L into H.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_H_L = 0x65,

    /// Loads the value at the address in HL into H.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_H_ADDR_HL = 0x66,

    /// Loads the value in A into H.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_H_A = 0x67,

    /// Loads the value in B into L.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_L_B = 0x68,

    /// Loads the value in C into L.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_L_C = 0x69,

    /// Loads the value in D into L.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_L_D = 0x6A,

    /// Loads the value in E into L.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_L_E = 0x6B,

    /// Loads the value in H into L.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_L_H = 0x6C,

    /// Loads the value in L into L.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_L_L = 0x6D,

    /// Loads the value at the address in HL into L.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_L_ADDR_HL = 0x6E,

    /// Loads the value in A into L.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_L_A = 0x6F,

    /// Loads the value in B into the address in HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_ADDR_HL_B = 0x70,

    /// Loads the value in C into the address in HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_ADDR_HL_C = 0x71,

    /// Loads the value in D into the address in HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_ADDR_HL_D = 0x72,

    /// Loads the value in E into the address in HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_ADDR_HL_E = 0x73,

    /// Loads the value in H into the address in HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_ADDR_HL_H = 0x74,

    /// Loads the value in L into the address in HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_ADDR_HL_L = 0x75,

    /// Suspends execution until an interrupt occurs.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    HALT = 0x76,

    /// Loads the value in A into the address in HL.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_ADDR_HL_A = 0x77,

    /// Loads the value in B into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_A_B = 0x78,

    /// Loads the value in C into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_A_C = 0x79,

    /// Loads the value in D into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_A_D = 0x7A,

    /// Loads the value in E into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_A_E = 0x7B,

    /// Loads the value in H into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_A_H = 0x7C,

    /// Loads the value in L into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_A_L = 0x7D,

    /// Loads the value at the address in HL into A.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// - Flags: none
    LD_A_ADDR_HL = 0x7E,

    /// Loads the value in A into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    LD_A_A = 0x7F,

    /// Adds B, excluding the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADD_A_B = 0x80,

    /// Adds C, excluding the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADD_A_C = 0x81,

    /// Adds D, excluding the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADD_A_D = 0x82,

    /// Adds E, excluding the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADD_A_E = 0x83,

    /// Adds H, excluding the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADD_A_H = 0x84,

    /// Adds L, excluding the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADD_A_L = 0x85,

    /// Adds the value at the address in HL, excluding the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// Flags: Z=*, N=0, H=*, C=*
    ADD_A_ADDR_HL = 0x86,

    /// Adds A, excluding the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADD_A_A = 0x87,

    /// Adds B, including the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADC_A_B = 0x88,

    /// Adds C, including the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADC_A_C = 0x89,

    /// Adds D, including the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADC_A_D = 0x8A,

    /// Adds E, including the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADC_A_E = 0x8B,

    /// Adds H, including the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADC_A_H = 0x8C,

    /// Adds L, including the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADC_A_L = 0x8D,

    /// Adds the value at the address in HL, including the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// Flags: Z=*, N=0, H=*, C=*
    ADC_A_ADDR_HL = 0x8E,

    /// Adds A, including the carry flag, to A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=*, C=*
    ADC_A_A = 0x8F,

    /// Subtracts B, excluding the carry flag, from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    SUB_A_B = 0x90,

    /// Subtracts C, excluding the carry flag, from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    SUB_A_C = 0x91,

    /// Subtracts D, excluding the carry flag, from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    SUB_A_D = 0x92,

    /// Subtracts E, excluding the carry flag, from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    SUB_A_E = 0x93,

    /// Subtracts H, excluding the carry flag, from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    SUB_A_H = 0x94,

    /// Subtracts L, excluding the carry flag, from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    SUB_A_L = 0x95,

    /// Subtracts the value at the address in HL, excluding the carry flag, from A.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// Flags: Z=*, N=1, H=*, C=*
    SUB_A_ADDR_HL = 0x96,

    /// Subtracts A, excluding the carry flag, from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=1, N=1, H=0, C=0
    SUB_A_A = 0x97,
    
    /// Subtracts B and the carry flag from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    SBC_A_B = 0x98,

    /// Subtracts C and the carry flag from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    SBC_A_C = 0x99,

    /// Subtracts D and the carry flag from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    SBC_A_D = 0x9A,

    /// Subtracts E and the carry flag from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    SBC_A_E = 0x9B,

    /// Subtracts H and the carry flag from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    SBC_A_H = 0x9C,

    /// Subtracts L and the carry flag from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    SBC_A_L = 0x9D,

    /// Subtracts the value at the address in HL and the carry flag from A.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// Flags: Z=*, N=1, H=*, C=*
    SBC_A_ADDR_HL = 0x9E,

    /// Subtracts A and the carry flag from A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    SBC_A_A = 0x9F,

    /// Bitwise ANDs A and B into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=1, C=0
    AND_A_B = 0xA0,

    /// Bitwise ANDs A and C into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=1, C=0
    AND_A_C = 0xA1,

    /// Bitwise ANDs A and D into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=1, C=0
    AND_A_D = 0xA2,

    /// Bitwise ANDs A and E into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=1, C=0
    AND_A_E = 0xA3,

    /// Bitwise ANDs A and H into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=1, C=0
    AND_A_H = 0xA4,

    /// Bitwise ANDs A and L into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=1, C=0
    AND_A_L = 0xA5,

    /// Bitwise ANDs A and the value at the address in HL, into A.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// Flags: Z=*, N=0, H=1, C=0
    AND_A_ADDR_HL = 0xA6,

    /// Bitwise A AND A into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=1, C=0
    AND_A_A = 0xA7,

    /// Bitwise XORs A and B into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=0, C=0
    XOR_A_B = 0xA8,

    /// Bitwise XORs A and C into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=0, C=0
    XOR_A_C = 0xA9,

    /// Bitwise XORs A and D into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=0, C=0
    XOR_A_D = 0xAA,

    /// Bitwise XORs A and E into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=0, C=0
    XOR_A_E = 0xAB,

    /// Bitwise XORs A and H into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=0, C=0
    XOR_A_H = 0xAC,

    /// Bitwise XORs A and L into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=0, C=0
    XOR_A_L = 0xAD,

    /// Bitwise XORs A and the value at the address in HL, into A.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// Flags: Z=*, N=0, H=0, C=0
    XOR_A_ADDR_HL = 0xAE,

    /// Bitwise XORs A and A into A, which always sets A to zero.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=1, N=0, H=0, C=0
    XOR_A_A = 0xAF,

    /// Bitwise ORs B with A, into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=0, C=0
    OR_A_B = 0xB0,

    /// Bitwise ORs C with A, into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=0, C=0
    OR_A_C = 0xB1,

    /// Bitwise ORs D with A, into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=0, C=0
    OR_A_D = 0xB2,

    /// Bitwise ORs E with A into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=0, C=0
    OR_A_E = 0xB3,

    /// Bitwise ORs H with A, into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=0, C=0
    OR_A_H = 0xB4,

    /// Bitwise ORs L with A, into A.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=0, C=0
    OR_A_L = 0xB5,

    /// Bitwise ORs A with the value at the address in HL, into A.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// Flags: Z=*, N=0, H=0, C=0
    OR_A_ADDR_HL = 0xB6,

    /// Bitwise ORs A with A, which leaves A unchanged.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=0, H=0, C=0
    OR_A_A = 0xB7,

    /// Compares B with A by subtracting B from A and discarding the result.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    CP_A_B = 0xB8,

    /// Compares C with A by subtracting C from A and discarding the result.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    CP_A_C = 0xB9,

    /// Compares D with A by subtracting D from A and discarding the result.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    CP_A_D = 0xBA,

    /// Compares E with A by subtracting E from A and discarding the result.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    CP_A_E = 0xBB,

    /// Compares H with A by subtracting H from A and discarding the result.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    CP_A_H = 0xBC,

    /// Compares L with A by subtracting L from A and discarding the result.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=*, N=1, H=*, C=*
    CP_A_L = 0xBD,

    /// Compares the value at the address in HL with A by subtracting it from A and discarding the result.
    /// - M-cycles: 1
    /// - T-cycles: 8
    /// Flags: Z=*, N=1, H=*, C=*
    CP_A_ADDR_HL = 0xBE,

    /// Compares A with A, which always results in zero.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// Flags: Z=1, N=1, H=0, C=0
    CP_A_A = 0xBF,

    /// Returns if the zero flag is not set.
    /// - M-cycles: 1
    /// - T-cycles: 20 (taken) / 8 (not taken)
    /// - Flags: none
    RET_NZ = 0xC0,

    /// Pops a 16-bit value from the stack into BC.
    /// - M-cycles: 1
    /// - T-cycles: 12
    /// - Flags: none
    POP_BC = 0xC1,

    /// Jumps to an immediate 16-bit address if the zero flag is not set.
    /// - M-cycles: 3
    /// - T-cycles: 16 (taken) / 12 (not taken)
    /// - Flags: none
    JP_NZ_a16 = 0xC2,

    /// Jumps to an immediate 16-bit address.
    /// - M-cycles: 3
    /// - T-cycles: 16
    /// - Flags: none
    JP_a16 = 0xC3,

    /// Calls a subroutine if the zero flag is not set.
    /// - M-cycles: 3
    /// - T-cycles: 24 (taken) / 12 (not taken)
    /// - Flags: none
    CALL_NZ_a16 = 0xC4,

    /// Pushes the value in BC to the stack.
    /// - M-cycles: 1
    /// - T-cycles: 16
    /// - Flags: none
    PUSH_BC = 0xC5,

    /// Adds an immediate unsigned 8-bit value to A.
    /// - M-cycles: 2
    /// - T-cycles: 8
    /// - Flags: Z=*, N=0, H=*, C=*
    ADD_A_n8 = 0xC6,

    /// ???
    /// - M-cycles: 1
    /// - T-cycles: 16
    /// - Flags: none
    RST_00 = 0xC7,

    /// Returns if the zero flag is set.
    /// - M-cycles: 2
    /// - T-cycles: 20 (taken) / 8 (not taken)
    /// - Flags: none
    RET_Z = 0xC8,

    /// Returns unconditionally.
    /// - M-cycles: 1
    /// - T-cycles: 16
    /// - Flags: none
    RET = 0xC9,

    /// Jumps to an immediate address if the zero flag is set.
    /// - M-cycles: 3
    /// - T-cycles: 16 (taken) / 12 (not taken)
    /// - Flags: none
    JP_Z_a16 = 0xCA,

    /// Signals a CB-prefix opcode.
    /// - M-cycles: 1
    /// - T-cycles: 4
    /// - Flags: none
    PREFIX = 0xCB,

    /// Calls a subroutine if the zero flag is set.
    /// - M-cycles: 3
    /// - T-cycles: 24 (taken) / 12 (not taken)
    /// - Flags: none
    CALL_Z_a16 = 0xCC,

    /// Calls a subroutine.
    /// - M-cycles: 3
    /// - T-cycles: 24
    /// - Flags: none
    CALL_a16 = 0xCD,

    /// Adds an immediate 8-bit value to A.
    /// - M-cycles: 2
    /// - T-cycles: 8
    /// - Flags: Z=*, N=0, H=*, C=*
    ADC_A_n8 = 0xCE,

    /// ???
    /// - M-cycles: 1
    /// - T-cycles: 16
    /// - Flags: none
    RST_08 = 0xCF,
}