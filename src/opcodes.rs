#[derive(Debug, Clone, Copy)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    DerefHL,
}

#[derive(Debug, Clone, Copy)]
pub enum Register16 {
    BC,
    DE,
    HL,
    AF,
    SP,
}

#[derive(Debug, Clone, Copy)]
pub enum Flag {
    C,
    Z,
    NC,
    NZ,
}

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Nop,
    Stop,
    Halt,
    StoreImm16(Register16, u16),
    StoreImm8(Register8, u8),
    /// Store A at (HL) and increment or decrement HL; true means inc
    StoreAToHl(bool),
    /// Load A from (HL) and increment or decrement HL; true means inc
    LoadAFromHl(bool),
    /// Store A to the value pointed at by register 16 (must be BC or DE)
    StoreATo16(Register16),
    /// Loads A from value pointed at by register 16 (must be BC or DE)
    LoadAFrom16(Register16),
    Mov8(Register8, Register8),
    /// Relative jump based on flag to offset
    Jr(Option<Flag>, u8),
    /// Jump based on flag to offset
    Jp(Option<Flag>, u16),
    Inc8(Register8),
    Dec8(Register8),
    Inc16(Register16),
    Dec16(Register16),
    Push(Register16),
    Pop(Register16),
    Add(Register8),
    Adc(Register8),
    Sub(Register8),
    Sbc(Register8),
    And(Register8),
    Xor(Register8),
    Or(Register8),
    Cp(Register8),
    Add8(u8),
    Adc8(u8),
    Sub8(u8),
    Sbc8(u8),
    And8(u8),
    Xor8(u8),
    Or8(u8),
    Cp8(u8),
    AddSp8(u8),
    Daa,
    Scf,
    Cpl,
    Ccf,
    Rlca,
    Rla,
    Rrca,
    Rra,
    /// Stores SP at pointer given by immediate 16
    StoreImm16Sp(u16),
    /// Adds a value to HL
    AddHl(Register16),
    Ret(Option<Flag>),
    Reti,
    Di,
    Ei,
    Call(Option<Flag>, u16),
    /// gets the value at memory address HL and jumps to it
    JpHl,
    /// 8 possible values; TODO document this
    Rst(u8),
    /// HL = SP + u8
    LdHlSp8(u8),
    LdSpHl,
    /// stores A in (u8)
    StoreHA(u8),
    /// loads A from (u8)
    LoadHA(u8),
    /// stores A in (C)
    StoreCA,
    /// Loads A from (C)
    LoadCA,
    /// LD (a16), A
    StoreAAtAddress(u16),
    /// LD A, (a16)
    LoadAFromAddress(u16),
    /// 0xCB instructions
    Rlc(Register8),
    Rrc(Register8),
    Rr(Register8),
    Rl(Register8),
    Sla(Register8),
    Sra(Register8),
    Swap(Register8),
    Srl(Register8),
    /// u8 is number between 0 and 7 (inclusive)
    Bit(u8, Register8),
    /// u8 is number between 0 and 7 (inclusive)
    Res(u8, Register8),
    /// u8 is number between 0 and 7 (inclusive)
    Set(u8, Register8),
}
