use rom_res_rs::*;
use std::io::Cursor;

mod hex_print_utils;
use hex_print_utils::*;

const WORLD_RES: &[u8] = include_bytes!("WORLD.RES");

fn main() {
    let cursor = Cursor::new(WORLD_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(data_bin) = resource_file.get_resource_bytes("data/data.bin"){
            print_hex(data_bin);
        }
    }
}