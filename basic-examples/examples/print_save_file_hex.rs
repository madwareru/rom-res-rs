use print_hex_rs::print_hex;
use rom_loaders_rs::regfile::Registry;

const SAVE_FILE: &[u8] = include_bytes!("game9999.sav");

fn main() {
    print_hex(SAVE_FILE);
    let reg_subslice = &SAVE_FILE[0x5DFC..];
    if let Ok(reg_file) = Registry::read_from_bytes(reg_subslice) {
        let registry_enumeration = reg_file.list_all();
        if registry_enumeration.ints.len() > 0 {
            println!("Int entries:");
            for entry_name in registry_enumeration.ints {
                println!("  {}: {}", entry_name, reg_file.get_int(&entry_name).unwrap());
            }
        }
        if registry_enumeration.floats.len() > 0 {
            println!("Float entries:");
            for entry_name in registry_enumeration.floats {
                println!("  {}: {}", entry_name, reg_file.get_float(&entry_name).unwrap());
            }
        }
        let mut reg_file = reg_file;
        if registry_enumeration.int_arrays.len() > 0 {
            println!("Int array entries:");
            for entry_name in registry_enumeration.int_arrays {
                println!("  {}: {:?}", entry_name, reg_file.get_int_slice(&entry_name).unwrap());
            }
        }
        if registry_enumeration.strings.len() > 0 {
            println!("String entries:");
            for entry_name in registry_enumeration.strings {
                println!("  {}: {}", entry_name, reg_file.get_string(&entry_name).unwrap());
            }
        }
    }
}