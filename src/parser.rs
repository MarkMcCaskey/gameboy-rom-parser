//! The logic to transform bytes into GameBoy ROM data types

use crate::data::*;
use crate::util::*;

use nom::{
    branch::*,
    bytes::complete::{escaped, is_not, tag, take},
    character::complete::hex_digit1,
    combinator::*,
    error::context,
    multi::{many0, many1},
    number::complete::{be_u16, hex_u32},
    sequence::{delimited, preceded, tuple},
    IResult,
};

pub fn parse_scrolling_graphic<'a>(input: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    take(0x30usize)(input)
}

pub fn parse_game_title<'a>(input: &'a [u8]) -> IResult<&'a [u8], &'a str> {
    map_res(
        cut(context("game title as ASCII", take(0x10usize))),
        std::str::from_utf8,
    )(input)
}

pub fn parse_gbc_byte<'a>(input: &'a [u8]) -> IResult<&'a [u8], bool> {
    let (i, byte) = take(1usize)(input)?;

    Ok((i, byte[0] == 0x80))
}

pub fn parse_rom_type<'a>(input: &'a [u8]) -> IResult<&'a [u8], RomType> {
    let (i, byte) = take(1usize)(input)?;

    Ok((i, byte[0].into()))
}

pub fn parse_new_licensee_code<'a>(input: &'a [u8]) -> IResult<&'a [u8], u8> {
    map(map_parser(take(2usize), hex_u32), |n| n as u8)(input)
}

/// 3 is SGB
/// 0 is GB
pub fn parse_sgb_byte<'a>(input: &'a [u8]) -> IResult<&'a [u8], bool> {
    let (i, byte) = take(1usize)(input)?;

    Ok((i, byte[0] == 0x03))
}

pub fn parse_rom_size<'a>(input: &'a [u8]) -> IResult<&'a [u8], u8> {
    map_opt(take(1usize), |bytes: &'a [u8]| translate_rom_size(bytes[0]))(input)
}

pub fn parse_ram_size<'a>(input: &'a [u8]) -> IResult<&'a [u8], (u8, u16)> {
    map_opt(take(1usize), |bytes: &'a [u8]| translate_ram_size(bytes[0]))(input)
}

pub fn parse_jp_byte<'a>(input: &'a [u8]) -> IResult<&'a [u8], bool> {
    let (i, byte) = take(1usize)(input)?;

    Ok((i, byte[0] == 0))
}

pub fn parse_byte<'a>(input: &'a [u8]) -> IResult<&'a [u8], u8> {
    map(take(1usize), |bytes: &'a [u8]| bytes[0])(input)
}

pub fn parse_rom_header<'a>(input: &'a [u8]) -> IResult<&'a [u8], RomHeader<'a>> {
    map(
        tuple((
            parse_scrolling_graphic,
            parse_game_title,
            parse_gbc_byte,
            parse_new_licensee_code,
            parse_sgb_byte,
            parse_rom_type,
            parse_rom_size,
            parse_ram_size,
            parse_jp_byte,
            // old licensee code
            parse_byte,
            // mask rom version number usually 0x00
            parse_byte,
            // complement check
            parse_byte,
            // checksum
            be_u16,
        )),
        |(
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
