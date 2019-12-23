//! The logic to transform bytes into GameBoy ROM data types

use crate::data::*;
use crate::opcodes::*;
use crate::util::*;

use nom::{
    bytes::complete::take,
    combinator::*,
    error::{context, VerboseError},
    number::complete::{be_u16, le_u16},
    sequence::tuple,
    IResult,
};

pub fn parse_scrolling_graphic<'a>(
    input: &'a [u8],
) -> IResult<&'a [u8], &'a [u8], VerboseError<&'a [u8]>> {
    context("scrolling graphic", take(0x30usize))(input)
}

pub fn parse_game_title<'a>(input: &'a [u8]) -> IResult<&'a [u8], &'a str, VerboseError<&'a [u8]>> {
    context(
        "game title as ASCII",
        map_res(take(0xFusize), std::str::from_utf8),
    )(input)
}

pub fn parse_gbc_byte<'a>(input: &'a [u8]) -> IResult<&'a [u8], bool, VerboseError<&'a [u8]>> {
    let (i, byte) = take(1usize)(input)?;

    Ok((i, byte[0] == 0x80))
}

pub fn parse_rom_type<'a>(input: &'a [u8]) -> IResult<&'a [u8], RomType, VerboseError<&'a [u8]>> {
    let (i, byte) = take(1usize)(input)?;

    Ok((i, byte[0].into()))
}

pub fn parse_new_licensee_code<'a>(
    input: &'a [u8],
) -> IResult<&'a [u8], [u8; 2], VerboseError<&'a [u8]>> {
    context(
        "new licensee code",
        map(take(2usize), |bytes: &'a [u8]| [bytes[0], bytes[1]]),
    )(input)
}

/// 3 is SGB
/// 0 is GB
pub fn parse_sgb_byte<'a>(input: &'a [u8]) -> IResult<&'a [u8], bool, VerboseError<&'a [u8]>> {
    let (i, byte) = take(1usize)(input)?;

    Ok((i, byte[0] == 0x03))
}

pub fn parse_rom_size<'a>(input: &'a [u8]) -> IResult<&'a [u8], u16, VerboseError<&'a [u8]>> {
    context(
        "ROM size byte",
        map_opt(take(1usize), |bytes: &'a [u8]| translate_rom_size(bytes[0])),
    )(input)
}

pub fn parse_ram_size<'a>(input: &'a [u8]) -> IResult<&'a [u8], (u8, u16), VerboseError<&'a [u8]>> {
    context(
        "RAM size byte",
        map_opt(take(1usize), |bytes: &'a [u8]| translate_ram_size(bytes[0])),
    )(input)
}

pub fn parse_jp_byte<'a>(input: &'a [u8]) -> IResult<&'a [u8], bool, VerboseError<&'a [u8]>> {
    let (i, byte) = take(1usize)(input)?;

    Ok((i, byte[0] == 0))
}

pub fn parse_byte<'a>(input: &'a [u8]) -> IResult<&'a [u8], u8, VerboseError<&'a [u8]>> {
    map(take(1usize), |bytes: &'a [u8]| bytes[0])(input)
}

pub fn parse_rom_header<'a>(
    input: &'a [u8],
) -> IResult<&'a [u8], RomHeader<'a>, VerboseError<&'a [u8]>> {
    map(
        tuple((
            context("Rom start", take(0x100usize)),
            context("begin code execution point", take(4usize)),
            parse_scrolling_graphic,
            parse_game_title,
            parse_gbc_byte,
            parse_new_licensee_code,
            parse_sgb_byte,
            parse_rom_type,
            parse_rom_size,
            parse_ram_size,
            parse_jp_byte,
            context("old licensee code", parse_byte),
            context("mask rom version number", parse_byte),
            context("complement", parse_byte),
            context("checksum", be_u16),
        )),
        |(
            _,
            begin_code_execution_point,
            scrolling_graphic,
            game_title,
            gameboy_color,
            licensee_code_new,
            super_gameboy,
            rom_type,
            rom_size,
            (ram_banks, ram_bank_size),
            japanese,
            licensee_code,
            mask_rom_version,
            complement,
            checksum,
        )| {
            RomHeader {
                begin_code_execution_point,
                scrolling_graphic,
                game_title,
                gameboy_color,
                licensee_code_new,
                super_gameboy,
                rom_type,
                rom_size,
                ram_banks,
                ram_bank_size,
                japanese,
                licensee_code,
                mask_rom_version,
                complement,
                checksum,
            }
        },
    )(input)
}

pub fn parse_instruction(input: &[u8]) -> IResult<&[u8], Opcode, VerboseError<&[u8]>> {
    let (i, byte) = take(1usize)(input)?;
    if byte[0] == 0xCB {
        return parse_cb(i);
    }
    Ok(match byte[0] {
        0x00 => (i, Opcode::Nop),
        0x10 => (i, Opcode::Stop),
        0x20 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::Jr(Some(Flag::NZ), bytes[0]))
        }
        0x30 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::Jr(Some(Flag::NC), bytes[0]))
        }
        0x18 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::Jr(None, bytes[0]))
        }
        0x28 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::Jr(Some(Flag::Z), bytes[0]))
        }
        0x38 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::Jr(Some(Flag::C), bytes[0]))
        }
        0x01 => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::StoreImm16(Register16::BC, short))
        }
        0x11 => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::StoreImm16(Register16::DE, short))
        }
        0x21 => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::StoreImm16(Register16::HL, short))
        }
        0x31 => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::StoreImm16(Register16::SP, short))
        }
        0x02 => (i, Opcode::StoreATo16(Register16::BC)),
        0x12 => (i, Opcode::StoreATo16(Register16::DE)),
        0x22 => (i, Opcode::StoreAToHlAddr(true)),
        0x32 => (i, Opcode::StoreAToHlAddr(false)),
        0x03 => (i, Opcode::Inc16(Register16::BC)),
        0x13 => (i, Opcode::Inc16(Register16::DE)),
        0x23 => (i, Opcode::Inc16(Register16::HL)),
        0x33 => (i, Opcode::Inc16(Register16::SP)),
        0x04 => (i, Opcode::Inc8(Register8::B)),
        0x14 => (i, Opcode::Inc8(Register8::D)),
        0x24 => (i, Opcode::Inc8(Register8::H)),
        0x34 => (i, Opcode::Inc8(Register8::DerefHL)),
        0x05 => (i, Opcode::Dec8(Register8::B)),
        0x15 => (i, Opcode::Dec8(Register8::D)),
        0x25 => (i, Opcode::Dec8(Register8::H)),
        0x35 => (i, Opcode::Dec8(Register8::DerefHL)),
        0x0C => (i, Opcode::Inc8(Register8::C)),
        0x1C => (i, Opcode::Inc8(Register8::E)),
        0x2C => (i, Opcode::Inc8(Register8::L)),
        0x3C => (i, Opcode::Inc8(Register8::A)),
        0x0D => (i, Opcode::Dec8(Register8::C)),
        0x1D => (i, Opcode::Dec8(Register8::E)),
        0x2D => (i, Opcode::Dec8(Register8::L)),
        0x3D => (i, Opcode::Dec8(Register8::A)),
        0x06 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::StoreImm8(Register8::B, bytes[0]))
        }
        0x16 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::StoreImm8(Register8::D, bytes[0]))
        }
        0x26 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::StoreImm8(Register8::H, bytes[0]))
        }
        0x36 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::StoreImm8(Register8::DerefHL, bytes[0]))
        }
        0x0E => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::StoreImm8(Register8::C, bytes[0]))
        }
        0x1E => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::StoreImm8(Register8::E, bytes[0]))
        }
        0x2E => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::StoreImm8(Register8::L, bytes[0]))
        }
        0x3E => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::StoreImm8(Register8::A, bytes[0]))
        }
        0x07 => (i, Opcode::Rlca),
        0x17 => (i, Opcode::Rla),
        0x27 => (i, Opcode::Daa),
        0x37 => (i, Opcode::Scf),
        0x0F => (i, Opcode::Rrca),
        0x1F => (i, Opcode::Rra),
        0x2F => (i, Opcode::Cpl),
        0x3F => (i, Opcode::Ccf),
        0x08 => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::StoreImm16Sp(short))
        },
        0x09 => (i, Opcode::AddHl(Register16::BC)),
        0x19 => (i, Opcode::AddHl(Register16::DE)),
        0x29 => (i, Opcode::AddHl(Register16::HL)),
        0x39 => (i, Opcode::AddHl(Register16::SP)),
        0x0A => (i, Opcode::LoadAFromReg16Addr(Register16::BC)),
        0x1A => (i, Opcode::LoadAFromReg16Addr(Register16::DE)),
        0x2A => (i, Opcode::LoadAFromHlAddr(true)),
        0x3A => (i, Opcode::LoadAFromHlAddr(false)),
        0x0B => (i, Opcode::Dec16(Register16::BC)),
        0x1B => (i, Opcode::Dec16(Register16::DE)),
        0x2B => (i, Opcode::Dec16(Register16::HL)),
        0x3B => (i, Opcode::Dec16(Register16::SP)),
        0x76 => (i, Opcode::Halt),
        0x40..=0x75 | 0x77..=0x7F => {
            let lo4 = byte[0] & 0b0000_1111;
            let hi4 = byte[0] >> 4;
            let operand1 = match lo4 {
                0x0..=0x7 if hi4 == 0x4 => Register8::B,
                0x8..=0xF if hi4 == 0x4 => Register8::C,
                0x0..=0x7 if hi4 == 0x5 => Register8::D,
                0x8..=0xF if hi4 == 0x5 => Register8::E,
                0x0..=0x7 if hi4 == 0x6 => Register8::H,
                0x8..=0xF if hi4 == 0x6 => Register8::L,
                0x0..=0x7 if hi4 == 0x7 => Register8::DerefHL,
                0x8..=0xF if hi4 == 0x7 => Register8::A,
                _ => unreachable!(),
            };

            let operand2 = match lo4 {
                0x0 | 0x8 => Register8::B,
                0x1 | 0x9 => Register8::C,
                0x2 | 0xA => Register8::D,
                0x3 | 0xB => Register8::E,
                0x4 | 0xC => Register8::H,
                0x5 | 0xD => Register8::L,
                0x6 | 0xE => Register8::DerefHL,
                0x7 | 0xF => Register8::A,
                _ => unreachable!(),
            };

            (i, Opcode::Mov8(operand1, operand2))
        }
        0x80..=0xBF => {
            let lo4 = byte[0] & 0b0000_1111;
            let hi4 = byte[0] >> 4;

            let operand = match lo4 {
                0x0 | 0x8 => Register8::B,
                0x1 | 0x9 => Register8::C,
                0x2 | 0xA => Register8::D,
                0x3 | 0xB => Register8::E,
                0x4 | 0xC => Register8::H,
                0x5 | 0xD => Register8::L,
                0x6 | 0xE => Register8::DerefHL,
                0x7 | 0xF => Register8::A,
                _ => unreachable!(),
            };

            (
                i,
                match (hi4, lo4) {
                    (0x8, 0x0..=0x7) => Opcode::Add(operand),
                    (0x8, 0x8..=0xF) => Opcode::Adc(operand),
                    (0x9, 0x0..=0x7) => Opcode::Sub(operand),
                    (0x9, 0x8..=0xF) => Opcode::Sbc(operand),
                    (0xA, 0x0..=0x7) => Opcode::And(operand),
                    (0xA, 0x8..=0xF) => Opcode::Xor(operand),
                    (0xB, 0x0..=0x7) => Opcode::Or(operand),
                    (0xB, 0x8..=0xF) => Opcode::Cp(operand),
                    _ => unreachable!(),
                },
            )
        }
        0xCB => parse_cb(i)?,
        0xC0 => (i, Opcode::Ret(Some(Flag::NZ))),
        0xD0 => (i, Opcode::Ret(Some(Flag::NC))),
        0xC8 => (i, Opcode::Ret(Some(Flag::Z))),
        0xD8 => (i, Opcode::Ret(Some(Flag::C))),
        0xC9 => (i, Opcode::Ret(None)),
        0xD9 => (i, Opcode::Reti),
        // ldh
        0xE0 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::StoreHA(bytes[0]))
        }
        // ldh
        0xF0 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::LoadHA(bytes[0]))
        }
        0xC1 => (i, Opcode::Pop(Register16::BC)),
        0xD1 => (i, Opcode::Pop(Register16::DE)),
        0xE1 => (i, Opcode::Pop(Register16::HL)),
        0xF1 => (i, Opcode::Pop(Register16::AF)),
        0xC5 => (i, Opcode::Push(Register16::BC)),
        0xD5 => (i, Opcode::Push(Register16::DE)),
        0xE5 => (i, Opcode::Push(Register16::HL)),
        0xF5 => (i, Opcode::Push(Register16::AF)),
        0xC2 => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::Jp(Some(Flag::NZ), short))
        }
        0xD2 => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::Jp(Some(Flag::NC), short))
        }
        0xC3 => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::Jp(None, short))
        }
        0xCA => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::Jp(Some(Flag::Z), short))
        }
        0xDA => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::Jp(Some(Flag::C), short))
        }
        // LD (C), A
        0xE2 => (i, Opcode::StoreCA),
        // LD A, (C)
        0xF2 => (i, Opcode::LoadCA),
        // LD (a16), A
        0xEA => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::StoreAAtAddress(short))
        }
        // LD A, (a16)
        0xFA => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::LoadAFromAddress(short))
        }
        0xF3 => (i, Opcode::Di),
        0xFB => (i, Opcode::Ei),
        0xC4 => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::Call(Some(Flag::NZ), short))
        }
        0xD4 => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::Call(Some(Flag::NC), short))
        }
        0xCC => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::Call(Some(Flag::Z), short))
        }
        0xDC => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::Call(Some(Flag::C), short))
        }
        0xCD => {
            let (i, short) = le_u16(i)?;
            (i, Opcode::Call(None, short))
        }
        0xC6 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::Add8(bytes[0]))
        }
        0xD6 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::Sub8(bytes[0]))
        }
        0xE6 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::And8(bytes[0]))
        }
        0xF6 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::Or8(bytes[0]))
        }
        0xCE => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::Adc8(bytes[0]))
        }
        0xDE => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::Sbc8(bytes[0]))
        }
        0xEE => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::Xor8(bytes[0]))
        }
        0xFE => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::Cp8(bytes[0]))
        }
        0xC7 => (i, Opcode::Rst(0)),
        0xCF => (i, Opcode::Rst(1)),
        0xD7 => (i, Opcode::Rst(2)),
        0xDF => (i, Opcode::Rst(3)),
        0xE7 => (i, Opcode::Rst(4)),
        0xEF => (i, Opcode::Rst(5)),
        0xF7 => (i, Opcode::Rst(6)),
        0xFF => (i, Opcode::Rst(7)),
        0xE8 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::AddSp8(bytes[0]))
        }
        0xF8 => {
            let (i, bytes) = take(1usize)(i)?;
            (i, Opcode::LdHlSp8(bytes[0]))
        }
        0xE9 => (i, Opcode::JpHl),
        0xF9 => (i, Opcode::LdSpHl),
        0xD3 | 0xE3 | 0xE4 | 0xF4 | 0xDB | 0xDD | 0xEB | 0xEC | 0xED | 0xFC | 0xFD => {
            unreachable!("TODO: error handling for invalid opcodes")
        } //nom::error::make_error(i, nom::error::ErrorKind::TagBits),
    })
}

/// Extra math functions
pub fn parse_cb(input: &[u8]) -> IResult<&[u8], Opcode, VerboseError<&[u8]>> {
    const CB_TABLE: [Opcode; 256] = [
        Opcode::Rlc(Register8::B),
        Opcode::Rlc(Register8::C),
        Opcode::Rlc(Register8::E),
        Opcode::Rlc(Register8::D),
        Opcode::Rlc(Register8::L),
        Opcode::Rlc(Register8::H),
        Opcode::Rlc(Register8::DerefHL),
        Opcode::Rlc(Register8::A),
        Opcode::Rrc(Register8::B),
        Opcode::Rrc(Register8::C),
        Opcode::Rrc(Register8::E),
        Opcode::Rrc(Register8::D),
        Opcode::Rrc(Register8::L),
        Opcode::Rrc(Register8::H),
        Opcode::Rrc(Register8::DerefHL),
        Opcode::Rrc(Register8::A),
        // 0x1X
        Opcode::Rl(Register8::B),
        Opcode::Rl(Register8::C),
        Opcode::Rl(Register8::D),
        Opcode::Rl(Register8::E),
        Opcode::Rl(Register8::H),
        Opcode::Rl(Register8::L),
        Opcode::Rl(Register8::DerefHL),
        Opcode::Rl(Register8::A),
        Opcode::Rr(Register8::B),
        Opcode::Rr(Register8::C),
        Opcode::Rr(Register8::D),
        Opcode::Rr(Register8::E),
        Opcode::Rr(Register8::H),
        Opcode::Rr(Register8::L),
        Opcode::Rr(Register8::DerefHL),
        Opcode::Rr(Register8::A),
        // 0x2X
        Opcode::Sla(Register8::B),
        Opcode::Sla(Register8::C),
        Opcode::Sla(Register8::D),
        Opcode::Sla(Register8::E),
        Opcode::Sla(Register8::H),
        Opcode::Sla(Register8::L),
        Opcode::Sla(Register8::DerefHL),
        Opcode::Sla(Register8::A),
        Opcode::Sra(Register8::B),
        Opcode::Sra(Register8::C),
        Opcode::Sra(Register8::D),
        Opcode::Sra(Register8::E),
        Opcode::Sra(Register8::H),
        Opcode::Sra(Register8::L),
        Opcode::Sra(Register8::DerefHL),
        Opcode::Sra(Register8::A),
        // 0x3X
        Opcode::Swap(Register8::B),
        Opcode::Swap(Register8::C),
        Opcode::Swap(Register8::D),
        Opcode::Swap(Register8::E),
        Opcode::Swap(Register8::H),
        Opcode::Swap(Register8::L),
        Opcode::Swap(Register8::DerefHL),
        Opcode::Swap(Register8::A),
        Opcode::Srl(Register8::B),
        Opcode::Srl(Register8::C),
        Opcode::Srl(Register8::D),
        Opcode::Srl(Register8::E),
        Opcode::Srl(Register8::H),
        Opcode::Srl(Register8::L),
        Opcode::Srl(Register8::DerefHL),
        Opcode::Srl(Register8::A),
        // 0x4X
        Opcode::Bit(0, Register8::B),
        Opcode::Bit(0, Register8::C),
        Opcode::Bit(0, Register8::D),
        Opcode::Bit(0, Register8::E),
        Opcode::Bit(0, Register8::H),
        Opcode::Bit(0, Register8::L),
        Opcode::Bit(0, Register8::DerefHL),
        Opcode::Bit(0, Register8::A),
        Opcode::Bit(1, Register8::B),
        Opcode::Bit(1, Register8::C),
        Opcode::Bit(1, Register8::D),
        Opcode::Bit(1, Register8::E),
        Opcode::Bit(1, Register8::H),
        Opcode::Bit(1, Register8::L),
        Opcode::Bit(1, Register8::DerefHL),
        Opcode::Bit(1, Register8::A),
        // 0x5X
        Opcode::Bit(2, Register8::B),
        Opcode::Bit(2, Register8::C),
        Opcode::Bit(2, Register8::D),
        Opcode::Bit(2, Register8::E),
        Opcode::Bit(2, Register8::H),
        Opcode::Bit(2, Register8::L),
        Opcode::Bit(2, Register8::DerefHL),
        Opcode::Bit(2, Register8::A),
        Opcode::Bit(3, Register8::B),
        Opcode::Bit(3, Register8::C),
        Opcode::Bit(3, Register8::D),
        Opcode::Bit(3, Register8::E),
        Opcode::Bit(3, Register8::H),
        Opcode::Bit(3, Register8::L),
        Opcode::Bit(3, Register8::DerefHL),
        Opcode::Bit(3, Register8::A),
        // 0x6X
        Opcode::Bit(4, Register8::B),
        Opcode::Bit(4, Register8::C),
        Opcode::Bit(4, Register8::D),
        Opcode::Bit(4, Register8::E),
        Opcode::Bit(4, Register8::H),
        Opcode::Bit(4, Register8::L),
        Opcode::Bit(4, Register8::DerefHL),
        Opcode::Bit(4, Register8::A),
        Opcode::Bit(5, Register8::B),
        Opcode::Bit(5, Register8::C),
        Opcode::Bit(5, Register8::D),
        Opcode::Bit(5, Register8::E),
        Opcode::Bit(5, Register8::H),
        Opcode::Bit(5, Register8::L),
        Opcode::Bit(5, Register8::DerefHL),
        Opcode::Bit(5, Register8::A),
        // 0x7X
        Opcode::Bit(6, Register8::B),
        Opcode::Bit(6, Register8::C),
        Opcode::Bit(6, Register8::D),
        Opcode::Bit(6, Register8::E),
        Opcode::Bit(6, Register8::H),
        Opcode::Bit(6, Register8::L),
        Opcode::Bit(6, Register8::DerefHL),
        Opcode::Bit(6, Register8::A),
        Opcode::Bit(7, Register8::B),
        Opcode::Bit(7, Register8::C),
        Opcode::Bit(7, Register8::D),
        Opcode::Bit(7, Register8::E),
        Opcode::Bit(7, Register8::H),
        Opcode::Bit(7, Register8::L),
        Opcode::Bit(7, Register8::DerefHL),
        Opcode::Bit(7, Register8::A),
        // 0x8X
        Opcode::Res(0, Register8::B),
        Opcode::Res(0, Register8::C),
        Opcode::Res(0, Register8::D),
        Opcode::Res(0, Register8::E),
        Opcode::Res(0, Register8::H),
        Opcode::Res(0, Register8::L),
        Opcode::Res(0, Register8::DerefHL),
        Opcode::Res(0, Register8::A),
        Opcode::Res(1, Register8::B),
        Opcode::Res(1, Register8::C),
        Opcode::Res(1, Register8::D),
        Opcode::Res(1, Register8::E),
        Opcode::Res(1, Register8::H),
        Opcode::Res(1, Register8::L),
        Opcode::Res(1, Register8::DerefHL),
        Opcode::Res(1, Register8::A),
        // 0x9X
        Opcode::Res(2, Register8::B),
        Opcode::Res(2, Register8::C),
        Opcode::Res(2, Register8::D),
        Opcode::Res(2, Register8::E),
        Opcode::Res(2, Register8::H),
        Opcode::Res(2, Register8::L),
        Opcode::Res(2, Register8::DerefHL),
        Opcode::Res(2, Register8::A),
        Opcode::Res(3, Register8::B),
        Opcode::Res(3, Register8::C),
        Opcode::Res(3, Register8::D),
        Opcode::Res(3, Register8::E),
        Opcode::Res(3, Register8::H),
        Opcode::Res(3, Register8::L),
        Opcode::Res(3, Register8::DerefHL),
        Opcode::Res(3, Register8::A),
        // 0xAX
        Opcode::Res(4, Register8::B),
        Opcode::Res(4, Register8::C),
        Opcode::Res(4, Register8::D),
        Opcode::Res(4, Register8::E),
        Opcode::Res(4, Register8::H),
        Opcode::Res(4, Register8::L),
        Opcode::Res(4, Register8::DerefHL),
        Opcode::Res(4, Register8::A),
        Opcode::Res(5, Register8::B),
        Opcode::Res(5, Register8::C),
        Opcode::Res(5, Register8::D),
        Opcode::Res(5, Register8::E),
        Opcode::Res(5, Register8::H),
        Opcode::Res(5, Register8::L),
        Opcode::Res(5, Register8::DerefHL),
        Opcode::Res(5, Register8::A),
        // 0xBX
        Opcode::Res(6, Register8::B),
        Opcode::Res(6, Register8::C),
        Opcode::Res(6, Register8::D),
        Opcode::Res(6, Register8::E),
        Opcode::Res(6, Register8::H),
        Opcode::Res(6, Register8::L),
        Opcode::Res(6, Register8::DerefHL),
        Opcode::Res(6, Register8::A),
        Opcode::Res(7, Register8::B),
        Opcode::Res(7, Register8::C),
        Opcode::Res(7, Register8::D),
        Opcode::Res(7, Register8::E),
        Opcode::Res(7, Register8::H),
        Opcode::Res(7, Register8::L),
        Opcode::Res(7, Register8::DerefHL),
        Opcode::Res(7, Register8::A),
        // 0xCX
        Opcode::Set(0, Register8::B),
        Opcode::Set(0, Register8::C),
        Opcode::Set(0, Register8::D),
        Opcode::Set(0, Register8::E),
        Opcode::Set(0, Register8::H),
        Opcode::Set(0, Register8::L),
        Opcode::Set(0, Register8::DerefHL),
        Opcode::Set(0, Register8::A),
        Opcode::Set(1, Register8::B),
        Opcode::Set(1, Register8::C),
        Opcode::Set(1, Register8::D),
        Opcode::Set(1, Register8::E),
        Opcode::Set(1, Register8::H),
        Opcode::Set(1, Register8::L),
        Opcode::Set(1, Register8::DerefHL),
        Opcode::Set(1, Register8::A),
        // 0xDX
        Opcode::Set(2, Register8::B),
        Opcode::Set(2, Register8::C),
        Opcode::Set(2, Register8::D),
        Opcode::Set(2, Register8::E),
        Opcode::Set(2, Register8::H),
        Opcode::Set(2, Register8::L),
        Opcode::Set(2, Register8::DerefHL),
        Opcode::Set(2, Register8::A),
        Opcode::Set(3, Register8::B),
        Opcode::Set(3, Register8::C),
        Opcode::Set(3, Register8::D),
        Opcode::Set(3, Register8::E),
        Opcode::Set(3, Register8::H),
        Opcode::Set(3, Register8::L),
        Opcode::Set(3, Register8::DerefHL),
        Opcode::Set(3, Register8::A),
        // 0xEX
        Opcode::Set(4, Register8::B),
        Opcode::Set(4, Register8::C),
        Opcode::Set(4, Register8::D),
        Opcode::Set(4, Register8::E),
        Opcode::Set(4, Register8::H),
        Opcode::Set(4, Register8::L),
        Opcode::Set(4, Register8::DerefHL),
        Opcode::Set(4, Register8::A),
        Opcode::Set(5, Register8::B),
        Opcode::Set(5, Register8::C),
        Opcode::Set(5, Register8::D),
        Opcode::Set(5, Register8::E),
        Opcode::Set(5, Register8::H),
        Opcode::Set(5, Register8::L),
        Opcode::Set(5, Register8::DerefHL),
        Opcode::Set(5, Register8::A),
        // 0xFX
        Opcode::Set(6, Register8::B),
        Opcode::Set(6, Register8::C),
        Opcode::Set(6, Register8::D),
        Opcode::Set(6, Register8::E),
        Opcode::Set(6, Register8::H),
        Opcode::Set(6, Register8::L),
        Opcode::Set(6, Register8::DerefHL),
        Opcode::Set(6, Register8::A),
        Opcode::Set(7, Register8::B),
        Opcode::Set(7, Register8::C),
        Opcode::Set(7, Register8::D),
        Opcode::Set(7, Register8::E),
        Opcode::Set(7, Register8::H),
        Opcode::Set(7, Register8::L),
        Opcode::Set(7, Register8::DerefHL),
        Opcode::Set(7, Register8::A),
    ];

    let (i, byte) = take(1usize)(input)?;
    Ok((i, CB_TABLE[byte[0] as usize]))
}
