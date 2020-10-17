use rom_res_rs::*;
use std::io::Cursor;

use rom_loaders_rs::regfile::Registry;

const GRAPHICS_RES: &[u8] = include_bytes!("GRAPHICS.RES");

fn main() {
    let cursor = Cursor::new(GRAPHICS_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(reg_file) = resource_file.get_resource_bytes("units/units.reg"){
            let registry = Registry::read_from_bytes(reg_file).unwrap();
            let registry_enumeration = registry.list_all();
            println!("Int entry names:");
            for entry_name in registry_enumeration.ints {
                println!("    {}", entry_name);
            }
            println!("Float entry names:");
            for entry_name in registry_enumeration.floats {
                println!("    {}", entry_name);
            }
            println!("Strings entry names:");
            for entry_name in registry_enumeration.strings {
                println!("    {}", entry_name);
            }
            println!("Int array entry names:");
            for entry_name in registry_enumeration.int_arrays {
                println!("    {}", entry_name);
            }
        }
    }
}