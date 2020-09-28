use rom_res_rs::*;
use std::io::Cursor;

const WORLD_RES: &[u8] = include_bytes!("WORLD.RES");

fn main() {
    let cursor = Cursor::new(WORLD_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut file_list = resource_file.get_resource_list();
        file_list.sort();
        for file_name in file_list {
            println!("{}", file_name)
        };
    }
}