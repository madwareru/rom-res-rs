use rom_res_rs::*;
use std::io::{Cursor};
use rom_loaders_rs::alm::AlmMap;

const SCENARIO_RES: &[u8] = include_bytes!("SCENARIO.RES");

fn main() {
    let cursor = Cursor::new(SCENARIO_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(alm) = resource_file.get_resource_bytes("10.alm") {
            let mut cursor = Cursor::new(alm);
            let alm_map = AlmMap::read(&mut cursor).unwrap();
            println!("map content:\n {:#?}", alm_map);
        }
    }
}