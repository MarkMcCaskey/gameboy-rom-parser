//! A parser for Gameboy ROMS.
//!
//! This crate provides a streaming Gameboy instruction parser as well as some
//! high-level types like `RomHeader` and `RomType`.
//!
//! Basic validation is provided through the `validate` method on `RomHeader`.
//!
//! Header logic based on info from the [GB CPU Manual].
//!
//! Opcode parsing logic was created with this [opcode table] as a reference.
//!
//! Information from other places is and other places is called out in comments in the relevant files
//!
//! [GB CPU Manual]: http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
//! [opcode table]: https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html

pub mod data;
mod opcodes;
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
