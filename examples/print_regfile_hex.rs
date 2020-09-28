use rom_res_rs::*;
use std::io::Cursor;

mod hex_print_utils;
use hex_print_utils::*;

const GRAPHICS_RES: &[u8] = include_bytes!("GRAPHICS.RES");

fn main() {
    let cursor = Cursor::new(GRAPHICS_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(reg_file) = resource_file.get_resource_bytes("units/units.reg"){
            print_hex(reg_file);
        }
    }
}