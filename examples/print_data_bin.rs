use rom_res_rs::*;
use std::io::Cursor;

const WORLD_RES: &[u8] = include_bytes!("WORLD.RES");
const HEX_CHARS: &[char] = &[
    '0','1','2','3',
    '4','5','6','7',
    '8','9','A','B',
    'C','D','E','F'
];
const WORD_SIZE: usize = 4;
const CHUNK_SIZE: usize = 0x20;
const WIDTH_EXPECTED: usize = CHUNK_SIZE * 2 + CHUNK_SIZE / WORD_SIZE + 1;

fn print_hex_chunk(chunk: &[u8]) {
    let mut chunk = chunk;
    let mut width = 0 as usize;
    while chunk.len() > 0 {
        let sub_chunk = if chunk.len() >= WORD_SIZE {
            &chunk[..WORD_SIZE]
        } else {
            chunk
        };
        for b in sub_chunk {
            let low = b & 0xF;
            let high = b / 0x10;
            print!("{}{}", HEX_CHARS[high as usize], HEX_CHARS[low as usize]);
            width += 2;
        }
        print!(" ");
        width += 1;
        chunk = if chunk.len() >= WORD_SIZE {
            &chunk[WORD_SIZE..]
        } else {
            &chunk[chunk.len()..]
        };
    }
    for _ in 0..WIDTH_EXPECTED-width {
        print!(" ");
    }
}

fn print_addr_offset_4(offset: usize) {
    let b = (offset / 0x1000000) & 0xFF;
    let low = b & 0xF;
    let high = b / 0x10;
    print!("{}{}", HEX_CHARS[high as usize], HEX_CHARS[low as usize]);
    let b = (offset / 0x10000) & 0xFF;
    let low = b & 0xF;
    let high = b / 0x10;
    print!("{}{}", HEX_CHARS[high as usize], HEX_CHARS[low as usize]);
    let b = (offset / 0x100) & 0xFF;
    let low = b & 0xF;
    let high = b / 0x10;
    print!("{}{}", HEX_CHARS[high as usize], HEX_CHARS[low as usize]);
    let b = offset & 0xFF;
    let low = b & 0xF;
    let high = b / 0x10;
    print!("{}{}> ", HEX_CHARS[high as usize], HEX_CHARS[low as usize]);
}

fn print_cp866_chunk(chunk: &[u8]) {
    for &b in chunk {
        let decoded = cp866_rs::decode_byte(b);
        if decoded.is_ascii_alphanumeric() {
            print!("{}", decoded);
        } else {
            print!(".");
        }
    }
}

fn main() {
    let cursor = Cursor::new(WORLD_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(data_bin) = resource_file.get_resource_bytes("data/data.bin"){
            let mut bytes = data_bin;
            let mut offset = 0 as usize;
            while bytes.len() > 0 {
                let chunk = if bytes.len() >= CHUNK_SIZE{
                    &bytes[..CHUNK_SIZE]
                } else {
                    bytes
                };
                print_addr_offset_4(offset);
                print_hex_chunk(chunk);
                print_cp866_chunk(chunk);
                println!();
                bytes = if bytes.len() >= CHUNK_SIZE {
                    &bytes[CHUNK_SIZE..]
                } else {
                    &bytes[bytes.len()..]
                };
                offset += CHUNK_SIZE;
            }
        }
    }
}