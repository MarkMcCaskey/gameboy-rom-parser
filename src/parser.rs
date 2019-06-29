//! The logic to transform bytes into GameBoy ROM data types

use crate::data::*;
use crate::util::*;

use nom::{
    bytes::complete::take,
    combinator::*,
    error::{context, VerboseError},
    number::complete::be_u16,
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
