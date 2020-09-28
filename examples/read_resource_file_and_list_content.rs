use rom_res_rs::*;
use std::io::Cursor;

const GRAPHICS_RES: &[u8] = include_bytes!("GRAPHICS.RES");

fn main() {
    let cursor = Cursor::new(GRAPHICS_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut file_list = resource_file.get_resource_list();
        file_list.sort();
        for file_name in file_list {
            println!("{}", file_name)
        };
    }
}