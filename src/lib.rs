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
pub mod opcodes;
pub mod parser;
pub mod util;

pub use data::*;

use opcodes::Opcode;

#[derive(Debug)]
pub struct GameBoyRom<'a> {
    rom_data: &'a [u8],
}

impl<'a> GameBoyRom<'a> {
    /// Create a new instance of the `GameBoyRom`.
    pub fn new(rom_bytes: &'a [u8]) -> Self {
        Self {
            rom_data: rom_bytes,
        }
    }

    /// Parse the ROM header and return a high level type containing its data.
    pub fn parse_header(&self) -> Result<RomHeader, String> {
        parser::parse_rom_header(self.rom_data)
            .map_err(|e| format!("Failed to parse ROM: {:?}", e))
            .map(|(_, rh)| rh)
    }

    pub fn get_instructions_at(&self, address: usize) -> impl Iterator<Item = Opcode> + Sized + 'a {
        OpcodeStreamer::new(self.rom_data, address)
    }
}

pub struct OpcodeStreamer<'rom> {
    rom_data: &'rom [u8],
    current_index: usize,
}

impl<'rom> OpcodeStreamer<'rom> {
    pub(crate) fn new(rom_bytes: &'rom [u8], start: usize) -> Self {
        Self {
            rom_data: rom_bytes,
            current_index: start,
        }
    }
}

impl<'rom> Iterator for OpcodeStreamer<'rom> {
    type Item = Opcode;

    fn next(&mut self) -> Option<Self::Item> {
        match parser::parse_instruction(&self.rom_data[self.current_index..]) {
            Ok((i, op)) => {
                // Compare the pointers to find out how many bytes we read
                let offset = i.as_ptr() as usize - (&self.rom_data[self.current_index..]).as_ptr() as usize;
                self.current_index += offset;

                Some(op)
            }
            Err(_) => None,
        }
    }
}
