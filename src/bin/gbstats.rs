use std::io::Read;

use gameboy_rom::{GameBoyRom, opcodes::Opcode};

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    let rom_file_path = if let Some(arg) = args.next() {
        arg
    } else {
        eprintln!("Must supply a path to a gameboy ROM");
        std::process::exit(-1);
    };
    let mut file = std::fs::File::open(rom_file_path).expect("gameboy rom file");
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).expect("read bytes from file");

    let gbr = GameBoyRom::new(bytes.as_slice());
    let intro_section = gbr.get_instructions_at(0x100);

    for o in intro_section {
        dbg!(o);
    }
}
