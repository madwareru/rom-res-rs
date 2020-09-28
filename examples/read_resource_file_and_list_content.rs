use rom_res_rs::*;
use std::io::Cursor;

const MUSIC_RES: &[u8] = include_bytes!("MUSIC.RES");

fn main() {
    let cursor = Cursor::new(MUSIC_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut file_list = resource_file.get_resource_list();
        file_list.sort();
        for file_name in file_list {
            println!("{}", file_name)
        };
    }
}