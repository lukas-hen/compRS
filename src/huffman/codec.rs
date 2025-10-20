use std::fmt::Debug;
use std::io::{Read, Write};
use std::u32;
use std::{error::Error, result::Result};

use super::bits::BitWriter;
use super::tree::HuffmanTree;

pub fn encode<R: Read>(reader: &mut R) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes = Vec::new();
    let n_read = reader.read_to_end(&mut bytes)?;

    let tree = HuffmanTree::from(bytes.as_slice());
    let lens = tree.get_bitlengths();
    let codes = lengths_to_codes(&lens);

    let mut encoded = Vec::new();

    encoded.write(&n_read.to_be_bytes())?; // Add syms to decode. Platform specific, should hardcode size.
    encoded.write(&lens)?; // Add canonical alphabet lengths.

    {
        // Need to scope the bitwriter to remove the mutable ref before moving out the buf.
        let mut bw = BitWriter::new(&mut encoded);
        for b in bytes.into_iter() {
            //println!("Char: {}, Code: {}, Len: {}", *b as char, codes[*b as usize].code, codes[*b as usize].len);
            bw.write_bits(codes[b as usize].code, codes[b as usize].len)?;
        }
    }

    Ok(encoded)
}

pub fn decode<R: Read>(reader: &mut R) -> Result<Vec<u8>, Box<dyn Error>> {
    // Get n symbols.
    let mut n_syms_buf = [0u8; 8]; // Platform sensitive.
    reader.read(&mut n_syms_buf)?;
    let n_syms = usize::from_be_bytes(n_syms_buf);

    let mut lens: [u8; 256] = [0; 256];
    reader.read(&mut lens)?;

    let alphabet = lengths_to_codes(&lens);
    let codes: Vec<Code> = alphabet.into_iter().filter(|c| c.len > 0).collect();

    const BUFSIZE: usize = 4;
    let mut buf = [0u8; BUFSIZE];

    let mut n = reader.read(&mut buf)?;
    let mut window = u32::from_be_bytes(buf);
    n = reader.read(&mut buf)?;

    let mut current_shift: usize = 0;
    let mut current_byte: usize = 0;
    let mut n_syms_read = 0;

    let mut decoded = Vec::new();

    {
        loop {
            let code = match_code(window, &codes)?;
            n_syms_read += 1;

            window <<= code.len;
            current_shift += code.len as usize;

            decoded.write(&[code.symbol.unwrap() as u8])?;

            if current_shift >= 8 {
                if current_byte == 4 {
                    n = reader.read(&mut buf)?;
                    current_byte = 0;
                }

                let mut new_byte: u32 = buf[current_byte] as u32;
                // window &= (1 << 8) - 1; // clear bits
                new_byte <<= current_shift - 8; // adjust potential offset
                window |= new_byte;

                current_byte += 1;
                current_shift -= 8;
            }

            if n_syms_read == n_syms {
                break;
            }
        }
    }

    Ok(decoded)
}

#[derive(Clone)]
struct Code {
    code: u32,
    len: u8,
    symbol: Option<char>,
}

impl Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({:?}, {}, {:#b})\n", self.symbol, self.len, self.code).as_str())
    }
}

fn lengths_to_codes(lens: &[u8; 256]) -> Vec<Code> {
    // Count number of symbols that have a certain length (in bits).
    let mut bl_count: [u32; 256] = [0; 256];
    for len in lens {
        bl_count[*len as usize] += 1;
    }
    bl_count[0] = 0;

    // Calculate initial values for each unique length (in bits).
    let mut next_code: [u32; 256] = [0; 256];
    let mut code: u32 = 0;

    for bitlen in 1..255 {
        // 255 absolute worst case len.
        code = (code + bl_count[bitlen - 1] as u32) << 1;
        next_code[bitlen] = code;
    }

    // Generate the actual codes.
    let mut codes: Vec<Code> = vec![
        Code {
            code: 0,
            len: 0,
            symbol: None
        };
        256
    ];
    codes.reserve_exact(256);

    for n in 0..255 {
        let bl = lens[n];
        if bl != 0 {
            codes[n] = Code {
                code: next_code[bl as usize],
                len: bl,
                symbol: Some(n as u8 as char),
            };
            next_code[bl as usize] += 1;
        }
    }

    codes
}

fn match_code(window: u32, codes: &[Code]) -> Result<Code, Box<dyn Error>> {
    for code in codes.iter() {
        let c = code.code << (32 - code.len);
        let mask = gen_bitmask_lalign(code.len);
        if (window & mask) == c {
            return Ok(code.clone());
        }
    }

    Err("Could not match code.".into())
}

fn gen_bitmask_lalign(sz: u8) -> u32 {
    if sz == 0 {
        return 0;
    } // Protect from overflow panic.
    let mask = u32::MAX >> 32 - sz; // right aligned
    mask.reverse_bits()
}
