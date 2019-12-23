//! Types related to the opcodes of the Gameboy parser.

/// An 8 bit register.
///
/// Includes the `DerefHL` variant which represents a memory access at the value
/// contained in the `HL` 16 bit register.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

/// A 16 bit register.
///
/// Does not include `PC`, the program counter, used to indicate which
/// instruction is being executed next.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Register16 {
    /// 16 bit register that's made up of 2 8 bit registers, `B` and `C`.
    BC,
    /// 16 bit register that's made up of 2 8 bit registers, `D` and `E`.
    DE,
    /// 16 bit register that's made up of 2 8 bit registers, `H` and `L`.
    HL,
    /// 16 bit register that's made up of 2 8 bit registers, `A` and `F`.
    AF,
    /// The stack pointer.
    SP,
}

/// Flags relevant to the operation of the instruction.
///
/// This flag is used to specify which flag the Opcode cares about, it is not
/// used to indicate which flags an instruction may set.
///
/// For example, `Opcode::Jp(Some(Flag::NZ), 0x1234)` says that the instruction
/// is a `Jp` (jump) instruction that will jump to location 0x1234, if the `NZ`
/// (not zero) flag is set.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Flag {
    /// The Carry flag.
    C,
    /// The Zero Flag.
    Z,
    /// The inverse of the Carry flag.
    NC,
    /// The inverse of the Zero flag.
    NZ,
}

/// The instructions of the Gameboy.
///
/// The naming tends to be of the form: `ActionDestSrc` when there is ambiguity.
///
/// For example, `Opcode::StoreImm16AddrSp` means that the `SP` register should
/// be stored at the address specified by the immediate 16 bit value.
///
/// These docs don't intend to include complete explanations of the instructions,
/// though the comments below may provide a basic overview.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Opcode {
    /// No operation.
    Nop,
    /// The Gameboy enters a very low-power STOP state, graphics will not continue to draw.
    Stop,
    /// The Gameboy enters a low-power HALT state.
    Halt,
    /// Store an immediate value into a 16 bit register.
    StoreImm16(Register16, u16),
    /// Store an immediate value into an 8 bit register.
    StoreImm8(Register8, u8),
    /// Store A at (HL) and increment or decrement HL; true means inc
    StoreAToHlAddr(bool),
    /// Load A from (HL) and increment or decrement HL; true means inc
    LoadAFromHlAddr(bool),
    /// Store A to the value pointed at by register 16 (must be BC or DE)
    StoreATo16(Register16),
    /// Loads A from value pointed at by register 16 (must be BC or DE)
    LoadAFromReg16Addr(Register16),
    Mov8(Register8, Register8),
    /// Relative jump based on flag to offset
    Jr(Option<Flag>, u8),
    /// Jump based on flag to offset
    Jp(Option<Flag>, u16),
    /// Increment an 8 bit regsiter.
    Inc8(Register8),
    /// Decrement an 8 bit regsiter.
    Dec8(Register8),
    /// Increment a 16 bit regsiter.
    Inc16(Register16),
    /// Decrement a 16 bit regsiter.
    Dec16(Register16),
    /// Push the value in the given register onto the stack.
    Push(Register16),
    /// Pop a value off the stack and load it into the given register.
    Pop(Register16),
    /// Add the given regsiter to the A.
    Add(Register8),
    /// Add the given regsiter to the A with carry.
    Adc(Register8),
    /// Subtract the given regsiter from the A.
    Sub(Register8),
    /// Subtract the given regsiter from the A with carry.
    Sbc(Register8),
    /// Bitwise AND the given register with the A.
    And(Register8),
    /// Bitwise XOR the given register with the A.
    Xor(Register8),
    /// Bitwise OR the given register with the A.
    Or(Register8),
    /// Compare the value of the given register with the A and set flags.
    Cp(Register8),
    /// Add an immediate value to the A.
    Add8(u8),
    /// Add an immediate value to the A with carry.
    Adc8(u8),
    /// Subtract an immediate value from the A.
    Sub8(u8),
    /// Subtract an immediate value from the A with carry.
    Sbc8(u8),
    /// Bitwise AND an immediate value with the A.
    And8(u8),
    /// Bitwise XOR an immediate value with the A.
    Xor8(u8),
    /// Bitwise OR an immediate value with the A.
    Or8(u8),
    /// Compare the immediate value with the A and set flags.
    Cp8(u8),
    /// Add the immediate value to the Program Counter and load it into SP.
    /// TODO: check this explanation
    AddSp8(u8),
    /// Converts the value in A to its BCD form.
    /// TODO: double check this
    Daa,
    /// TODO: document this
    Scf,
    /// Bitwise negate the value in the A.
    Cpl,
    /// TODO: document this (inverse of SCF?)
    Ccf,
    /// Rotate A left.
    Rlca,
    /// Rotate A left through carry.
    Rla,
    /// Rotate A right.
    Rrca,
    /// Rotate A right through carry.
    Rra,
    /// Stores SP at pointer given by immediate 16.
    StoreImm16AddrSp(u16),
    /// Adds a value to HL.
    AddHl(Register16),
    /// Conditionally adjusts the program counter and updates the stack pointer.
    Ret(Option<Flag>),
    /// Non-conditional `Ret` that also enables interrupts.
    Reti,
    /// Disable interrupts.
    Di,
    /// Enable interrupts.
    Ei,
    /// Conditionally update push the program counter onto the stack and adjusts
    /// the program counter.
    Call(Option<Flag>, u16),
    /// Gets the value at memory address HL and jumps to it.
    JpHl,
    /// Contains eight possible values: between 0-8. Value should be multplied
    /// by 8 to determine the reset location.
    /// TODO: consider simplifying this
    Rst(u8),
    /// HL = SP + (PC + i8).
    /// TODO: double check behavior of relative parameters.
    LdHlSp8(i8),
    /// Load the value of HL into SP.
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
    /// # 0xCB instructions
    ///
    /// Rotate register left.
    Rlc(Register8),
    /// Rotate register right.
    Rrc(Register8),
    /// Rotate register right through carry.
    Rr(Register8),
    /// Rotate register left through carry.
    Rl(Register8),
    /// Arithmetic left shift on given register.
    Sla(Register8),
    /// Arithmetic right shift on given register.
    Sra(Register8),
    /// Swap low and high nibble (4 bits).
    Swap(Register8),
    /// Logical Right shift on given register.
    Srl(Register8),
    /// Set flags based on the given bit in register.
    /// u8 is number between 0 and 7 (inclusive).
    Bit(u8, Register8),
    /// Reset the given bit in the register.
    /// u8 is number between 0 and 7 (inclusive)
    Res(u8, Register8),
    /// Set the given bit in the register.
    /// u8 is number between 0 and 7 (inclusive)
    Set(u8, Register8),
}
