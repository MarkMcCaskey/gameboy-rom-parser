// ROM HEADER

use nom::{
    branch::*,
    bytes::complete::{escaped, is_not, tag, take},
    character::complete::hex_digit1,
    combinator::*,
    error::context,
    multi::{many0, many1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

pub enum RomType {
    RomOnly,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery,
    Mbc2,
    Mbc2Battery,
    RomRam,
    RomRamBattery,
    Mmm01,
    Mmm01Sram,
    Mmm01SramBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery,
    Mbc3,
    Mbc3Ram,
    Mbc3RamBattery,
    Mbc5,
    Mbc5Ram,
    Mbc5RamBattery,
    Mbc5Rumble,
    Mbc5RumbleSram,
    Mbc5RumbleSramBattery,
    PocketCamera,
    Tama5,
    Huc3,
    Huc1,
    Other(u8),
}

impl From<u8> for RomType {
    fn from(byte: u8) -> RomType {
        match byte {
            0x00 => RomType::RomOnly,
            0x01 => RomType::Mbc1,
            0x02 => RomType::Mbc1Ram,
            0x03 => RomType::Mbc1RamBattery,
            0x05 => RomType::Mbc2,
            0x06 => RomType::Mbc2Battery,
            0x08 => RomType::RomRam,
            0x09 => RomType::RomRamBattery,
            0x0B => RomType::Mmm01,
            0x0C => RomType::Mmm01Sram,
            0x0D => RomType::Mmm01SramBattery,
            0x0F => RomType::Mbc3TimerBattery,
            0x10 => RomType::Mbc3TimerRamBattery,
            0x11 => RomType::Mbc3,
            0x12 => RomType::Mbc3Ram,
            0x13 => RomType::Mbc3RamBattery,
            0x19 => RomType::Mbc5,
            0x1A => RomType::Mbc5Ram,
            0x1B => RomType::Mbc5RamBattery,
            0x1C => RomType::Mbc5Rumble,
            0x1D => RomType::Mbc5RumbleSram,
            0x1E => RomType::Mbc5RumbleSramBattery,
            0x1F => RomType::PocketCamera,
            0xFD => RomType::Tama5,
            0xFE => RomType::Huc3,
            0xFF => RomType::Huc1,
            otherwise => RomType::Other(otherwise),
        }
    }
}

pub struct RomHeader<'a> {
    scrolling_graphic: &'a [u8],
    game_title: &'a str,
    gameboy_color: bool,
    /// 2 ASCII hex digits in
    license_code_new: [u8; 2],
    super_gameboy: bool,
    rom_type: RomType,
    rom_size: usize,
    ram_size: usize,
    japanese: bool,
    licensee_code: u8,
    mask_rom_version: u8,
    complement: u8,
    checksum: u16,
}

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

pub fn parse_new_licensee_code<'a>(input: &'a [u8]) -> IResult<&'a [u8], RomType> {
    map(tuple((hex_digit1, hex_digit1)), |(b1, b2)| [b1, b2])
}

pub fn parse_rom_header<'a>(input: &'a [u8]) -> IResult<&'a [u8], RomHeader<'a>> {
    map(
        tuple((
            parse_scrolling_graphic,
            parse_game_title,
            parse_gbc_byte,
            parse_rom_type,
            parse_new_licensee_code,
        )),
        |(scrolling_graphic, game_title, gameboy_color, rom_type, new_licensee_code)| RomHeader {
            scrolling_graphic,
            game_title,
            gameboy_color,
            rom_type,
            new_licensee_code,
        },
    )
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
