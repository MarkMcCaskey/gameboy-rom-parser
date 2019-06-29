# GameBoy ROM parser

[![Build Status](https://travis-ci.org/MarkMcCaskey/gameboy-rom-parser.svg?branch=master)](https://travis-ci.org/MarkMcCaskey/gameboy-rom-parser)
[![Crates.io Version](https://img.shields.io/crates/v/gameboy-rom.svg)](https://crates.io/crates/gameboy-rom)

A very simple parser to get data out of GB ROMs and perform basic validation.

It's a design goal to make validation generally optional.

## Demonstration

```shell
cargo run --bin gb2json --features="serde_json" -- /path/to/rom/data
```

And [here](https://github.com/MarkMcCaskey/rusty-boy/blob/master/src/cpu/cartridge/mod.rs)'s it in use in a real emulator ([rusty-boy]). 

[rusty-boy]: https://github.com/markmccaskey/rusty-boy