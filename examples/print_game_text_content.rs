use rom_res_rs::*;
use std::io::Cursor;

const MAIN_RES: &[u8] = include_bytes!("MAIN.RES");

fn main() {
    let cursor = Cursor::new(MAIN_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(bytes) = resource_file.get_resource_bytes("text/battle/m10/event01.txt") {
            let decoded_text = cp866_rs::decode_bytes(bytes);
            println!("{}", decoded_text);
        }
    }
}