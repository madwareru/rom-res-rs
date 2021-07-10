use rom_res_rs::*;
use std::io::{Cursor};
use print_hex_rs::print_hex;

const GRAPHICS_RES: &[u8] = include_bytes!("GRAPHICS.RES");

fn main() {
    let cursor = Cursor::new(GRAPHICS_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(pal) = resource_file.get_resource_bytes("projectiles/projectiles.pal") {
            print_hex(pal);
        }
    }
}