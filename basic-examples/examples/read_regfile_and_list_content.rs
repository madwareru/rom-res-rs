use rom_res_rs::*;
use std::io::Cursor;

use rom_loaders_rs::regfile::Registry;

const GRAPHICS_RES: &[u8] = include_bytes!("GRAPHICS.RES");
const PROJECTILE_REG_NAME: &str = "projectiles/projectiles.reg";

fn print_registry_info_from_res(file: &mut ResourceFile<Cursor<&[u8]>>, reg_name: &str) {
    if let Ok(reg_file) = file.get_resource_bytes(reg_name) {
        println!("{}:", reg_name);
        let mut registry = Registry::read_from_bytes(reg_file).unwrap();
        let registry_enumeration = registry.list_all();
        if registry_enumeration.ints.len() > 0 {
            println!("Int entries:");
            for entry_name in registry_enumeration.ints {
                println!("  {}: {}", entry_name, registry.get_int(&entry_name).unwrap());
            }
        }
        if registry_enumeration.floats.len() > 0 {
            println!("Float entries:");
            for entry_name in registry_enumeration.floats {
                println!("  {}: {}", entry_name, registry.get_float(&entry_name).unwrap());
            }
        }
        if registry_enumeration.int_arrays.len() > 0 {
            println!("Int array entries:");
            for entry_name in registry_enumeration.int_arrays {
                println!("  {}: {:?}", entry_name, registry.get_int_slice(&entry_name).unwrap());
            }
        }
        if registry_enumeration.strings.len() > 0 {
            println!("String entries:");
            for entry_name in registry_enumeration.strings {
                println!("  {}: {}", entry_name, registry.get_string(&entry_name).unwrap());
            }
        }
    }
}

fn main() {
    let cursor = Cursor::new(GRAPHICS_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        print_registry_info_from_res(&mut resource_file, PROJECTILE_REG_NAME);
    }
}