use rom_res_rs::*;
use std::io::{Cursor};
use rom_loaders_rs::data_bin::DataBinContent;

const WORLD_RES: &[u8] = include_bytes!("WORLD.RES");

fn main() {
    let cursor = Cursor::new(WORLD_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(data_bin) = resource_file.get_resource_bytes("data/data.bin") {
            let mut cursor = Cursor::new(data_bin);
            let data_bin_content = DataBinContent::read(&mut cursor);
            println!("data bin content:\n {:?}", data_bin_content);
        }
    }
}