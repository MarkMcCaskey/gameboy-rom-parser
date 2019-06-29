//! A parser for Gameboy ROMS. Provides high-level useful data types like `RomHeader`
//! and `RomType`. Basic validation is provided through the `validate` method
//! on `RomHeader`.
//!
//! NOTE: this crate does nothing with the data before or after the ROM header right now.
//! The bytes there are not validated as valid Gameboy machine code.
//!
//! Mostly based on info from http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf .
//! Information from other places is and other places is called out in comments in the relevant files

pub mod data;
pub mod parser;
pub mod util;

pub use data::*;

/// top level function to parse the ROM
/// returns the parsed header and the rest of the bytes
pub fn parse_rom(rom_data: &[u8]) -> Result<(RomHeader, &[u8]), String> {
    parser::parse_rom_header(rom_data)
        .map_err(|e| format!("Failed to parse ROM: {:?}", e))
        .map(|(rest, rh)| (rh, rest))
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
