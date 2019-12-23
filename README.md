# GameBoy ROM parser

[![Build Status](https://travis-ci.org/MarkMcCaskey/gameboy-rom-parser.svg?branch=master)](https://travis-ci.org/MarkMcCaskey/gameboy-rom-parser)
[![Crates.io Version](https://img.shields.io/crates/v/gameboy-rom.svg)](https://crates.io/crates/gameboy-rom)

A parser to get data out of GB ROMs and perform basic validation.  It provides a streaming opcode parser as well as high-level types for inspecting the Gameboy ROM's header.

## Demonstration

```shell
cargo run --bin gb2json --features="serde_json" -- /path/to/rom/data
cargo run --bin gbstats -- /path/to/rom/data
```

And [here](https://github.com/MarkMcCaskey/rusty-boy/blob/master/src/cpu/cartridge/mod.rs)'s it in use in a real emulator ([rusty-boy]). 

[rusty-boy]: https://github.com/markmccaskey/rusty-boy
