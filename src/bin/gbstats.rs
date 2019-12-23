//! A hacky little program to count the frequency of Gameboy instructions.
//! Please note that this doesn't handle relative jumps, has no awareness of ROM
//! banks, and double counts instructions if they were arrived at through a
//! different entry-point.
//!
//! This program is for API demonstration purposes only.
use std::io::Read;

use gameboy_rom::{GameBoyRom, opcodes::Opcode};
use std::collections::*;

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
    // seen jump locations
    let mut seen_locations: HashSet<usize> = HashSet::new();
    // locations we want to analyze
    let mut to_inspect: VecDeque<usize> = VecDeque::new();
    // instructions we've seen
    let mut seen_instructions: HashMap<Opcode, usize> = HashMap::new();

    // init start location
    seen_locations.insert(0x100);
    to_inspect.push_back(0x100);

    while let Some(loc) = to_inspect.pop_front() {
        for o in gbr.get_instructions_at(loc) {
            match o {
                Opcode::Call(_, address) | Opcode::Jp(_, address) =>  {
                    let adj_adr = address as usize;
                    if !seen_locations.contains(&adj_adr) {
                        seen_locations.insert(address as _);
                        to_inspect.push_back(address as _);
                    }
                }
                _ => (),
            }
            let ent = seen_instructions.entry(o.clone()).or_default();
            *ent += 1;
        }
    }

    let mut vec = seen_instructions.into_iter().collect::<Vec<(Opcode, usize)>>();
    vec.sort_by(|(_, a), (_, b)| b.cmp(&a));

    println!("The top 10 most common instructions were:");
    for i in 0..10 {
        println!("{:>2}. {:?} appearing {} times", (i + 1), vec[i].0, vec[i].1);
    }

    println!("");
    println!("The top 10 least common instructions were:");
    for i in 0..10 {
        let elem = vec[vec.len() - i - 1];
        println!("{:>2}. {:?} appearing {} times", (i + 1), elem.0, elem.1);
    }
}
