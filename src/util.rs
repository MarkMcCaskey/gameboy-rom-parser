//! Utility functions for parsing
// extra info from http://gbdev.gg8.se/wiki/articles/The_Cartridge_Header

/// Takes in the ROM size byte and outputs the number of ROM banks
pub fn translate_rom_size(input: u8) -> Option<u16> {
    match input {
        0 => Some(2),
        1 => Some(4),
        2 => Some(8),
        3 => Some(16),
        4 => Some(32),
        5 => Some(64),
        6 => Some(128),
        7 => Some(256),
        8 => Some(512),
        0x52 => Some(72),
        0x53 => Some(80),
        0x54 => Some(96),
        _ => None,
    }
}

/// Takes in the RAM size byte and outputs the number of RAM banks
/// and the size of each RAM bank in bytes
/// Standard values for RAM bank size are 2kB and 8kB
pub fn translate_ram_size(input: u8) -> Option<(u8, u16)> {
    const TWO_KB: u16 = 2 << 11;
    const EIGHT_KB: u16 = 2 << 13;
    match input {
        0 => Some((0, 0)),
        1 => Some((1, TWO_KB)),
        2 => Some((1, EIGHT_KB)),
        3 => Some((4, EIGHT_KB)),
        4 => Some((16, EIGHT_KB)),
        5 => Some((8, EIGHT_KB)),
        _ => None,
    }
}
