use std::io::Read;

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

    match gameboy_rom::parse_rom(&bytes[..]) {
        Ok((rh, _)) => {
            println!("{}", serde_json::to_string_pretty(&rh).unwrap());
            let validation_result = rh.validate();
            println!(
                "ROM passes validation check? {}",
                if let Err(err) = rh.validate() {
                    format!("NO: {:?}", err)
                } else {
                    "YES".to_string()
                }
            );
        }
        Err(e) => {
            eprintln!("Failed to parse ROM: {:?}", e);
            std::process::exit(-1);
        }
    }
}
