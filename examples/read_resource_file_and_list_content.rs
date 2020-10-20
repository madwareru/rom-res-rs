use rom_res_rs::*;
use std::io::Cursor;

const SCENARIO_RES: &[u8] = include_bytes!("SCENARIO.RES");

fn main() {
    let cursor = Cursor::new(SCENARIO_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut file_list = resource_file.get_resource_list();
        for file_name in file_list {
            println!("{}", file_name)
        };
    }
}