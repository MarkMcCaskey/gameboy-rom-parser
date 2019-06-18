//! Mostly based on info from http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf (An amazing GameBoy resource with very few mistakes)
// ROM HEADER

pub mod data;
pub mod parser;
pub mod util;

pub use data::*;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
