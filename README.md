## Decoder for custom tiny ASM architecture in RUST

This project I made just to learn some ASM + Rust, and this is decoder for my custom
8-bit ASM architecture. (actually its not mine, but Konstantin Vladimirov from MIPT)

## EXAMPLE OF USAGE:

in: 0x70 0xc7 0xc1 0x87

out:

MOVI 112

OUT D

IN B

ADD B, D
