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
        0x22 => (i, Opcode::StoreAToHl(true)),
        0x32 => (i, Opcode::StoreAToHl(false)),
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
        rest => unimplemented!("TODO: 0x{:X}", rest),
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
