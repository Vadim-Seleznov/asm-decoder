use std::io::{self, Read};

/*
Instruction set 
Mnemonic 	                            |Encoding (binary)
MOVI Imm — move constant to D register 	|   0IIIIIII
ADD R, R — add two registers modulo 256 |	1000RRRR
SUB R, R — sub two registers modulo 256 | 	1001RRRR
MUL R, R — mul two registers modulo 256 |	1010RRRR
DIV R, R — div two registers modulo 256 |	1011RRRR
IN R — read register R from keyboard 	|   110000RR
OUT R — write register R to screen 	    |   110001RR
*/

const REGS: [&str; 4]= ["A", "B", "C", "D"];

fn decode(byte: u8) -> Option<String> {
    if (byte >> 7) == 0 {
        let imm = byte & 0x7F;
        return Some(format!("MOVI {}", imm));
    }

    let op = (byte >> 4) & 0x0F;
    let r1 = (byte >> 2) & 0x03;
    let r2 = byte & 0x03;

    let mnemonic: Option<&str> = match op {
        0b1000 => Some("ADD"),
        0b1001 => Some("SUB"),
        0b1010 => Some("MUL"),
        0b1011 => Some("DIV"),
        _ => None,
    };

    if let Some(mn) = mnemonic {
        return Some(format!("{} {}, {}", mn, REGS[r1 as usize], REGS[r2 as usize]));
    }

    match op {
        0b1100 => {
            let sub = (byte >> 2) & 0x0F;

            let r = byte & 0x03;

            match sub {
                0b0000 => Some(format!("IN {}", REGS[r as usize])),
                0b0001 => Some(format!("OUT {}", REGS[r as usize])),
                _ => None,
            }
        }
        _ => None,
    }
}

fn parse_hex_byte(s: &str) -> Option<u8> {
    let hex = if s.starts_with("0x") || s.starts_with("0X") {
        &s[2..]
    } else { s };

    u8::from_str_radix(hex, 16).ok()
}

// I know that code is trash, but this is my 3 rust program :)
fn main() {
    let mut input: String = String::new();

    if io::stdin().read_to_string(&mut input).is_err() {
        println!("Wrong input: {input}");
        return;
    }

    let tokens: Vec<&str> = input.split_whitespace().collect();

    for token in tokens {
        let byte = match parse_hex_byte(token) {
            Some(b) => b,
            None => {
                println!("ERROR: Can not parse {token} into hex");
                return;
            }
        };

        match decode(byte) {
            Some(instr) => println!("{}", instr),
            None => {
                println!(
                    "ERROR: While decoding: {byte} (probably number is to big
                        or there is no such opcode)"
                );
                return;
            }
        }
    }
}