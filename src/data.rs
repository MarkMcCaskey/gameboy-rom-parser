//! Data types that the parser can produce
//! If you want to do things with a GameBoy ROM, use the types defined here

/// The ROM type as a convenient enum
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

/// Metadata about the ROM
pub struct RomHeader<'a> {
    /// Logo at the start, should match Nintendo Logo
    pub scrolling_graphic: &'a [u8],
    /// up to 10 ASCII characters
    pub game_title: &'a str,
    /// gbc bit
    pub gameboy_color: bool,
    /// 2 ASCII hex digits in
    pub licensee_code_new: u8,
    /// sgb bit
    pub super_gameboy: bool,
    /// how the data after the header will be parsed
    pub rom_type: RomType,
    /// How many 16KB ROM banks to use
    pub rom_size: u8,
    /// How many RAM banks are available on the cart
    pub ram_banks: u8,
    /// The size of the RAM bank in bytes (normal values are 2kB and 8kB)
    pub ram_bank_size: u16,
    /// jp bit
    pub japanese: bool,
    pub licensee_code: u8,
    pub mask_rom_version: u8,
    pub complement: u8,
    pub checksum: u16,
}

impl<'a> RomHeader<'a> {
    /// checks that the ROM header is internally consistent.
    /// warning: this doesn't guarantee that the entire ROM header is well formed
    /// TODO: consider parsing into unvalidated form (just segmented bytes and then doing a translation step...)
    pub fn validate(&self) -> Result<(), HeaderValidationError> {
        // TODO: look into if international copyright law actually protects these 48 bytes
        // for now just validate proxy metrics of the logo
        const XOR_RESULT: u8 = 134;
        const SUM_RESULT: u16 = 5446;
        const OR_RESULT: u8 = 255;
        const AND_RESULT: u8 = 0;
        if self
            .scrolling_graphic
            .iter()
            .map(|x| *x as u16)
            .sum::<u16>()
            != SUM_RESULT
            || self.scrolling_graphic.iter().fold(0, |a, b| a | b) != OR_RESULT
            || self.scrolling_graphic.iter().fold(0, |a, b| a & b) != AND_RESULT
            || self.scrolling_graphic.iter().fold(0, |a, b| a ^ b) != XOR_RESULT
        {
            return Err(HeaderValidationError::ScrollingLogoMismatch);
        }
        if self.super_gameboy && self.licensee_code != 0x33 {
            return Err(HeaderValidationError::SuperGameBoyOldLicenseeCodeMismatch);
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum HeaderValidationError {
    /// SGB requires the old licensee code to be 0x33
    SuperGameBoyOldLicenseeCodeMismatch,
    /// Apparent mismatch on scrolling logo
    ScrollingLogoMismatch,
}
