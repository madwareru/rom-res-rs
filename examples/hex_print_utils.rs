const HEX_CHARS: &[char] = &[
    '0','1','2','3',
    '4','5','6','7',
    '8','9','A','B',
    'C','D','E','F'
];
const WORD_SIZE: usize = 4;
pub const CHUNK_SIZE: usize = 0x20;
const WIDTH_EXPECTED: usize = CHUNK_SIZE * 2 + CHUNK_SIZE / WORD_SIZE + 1;

pub fn print_hex_chunk(chunk: &[u8]) {
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

pub fn print_addr_offset_4(offset: usize) {
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

pub fn print_cp866_chunk(chunk: &[u8]) {
    for &b in chunk {
        let decoded = cp866_rs::decode_byte(b);
        if decoded.is_ascii_alphanumeric() {
            print!("{}", decoded);
        } else {
            print!(".");
        }
    }
}

pub fn print_hex(bytes: &[u8]) {
    let mut bytes = &bytes[..];
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

fn main() {}