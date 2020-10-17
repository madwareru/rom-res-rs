use rom_res_rs::*;
use std::io::Cursor;

use rom_loaders_rs::multimedia::WavContent;
use print_hex_rs::print_hex;

const SFX_RES: &[u8] = include_bytes!("SFX.RES");

fn main() {
    let cursor = Cursor::new(SFX_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(data_bin) = resource_file.get_resource_bytes("monsters/orc/dead.wav"){
            print_hex(data_bin);

            let mut cursor = Cursor::new(data_bin);
            let wav = WavContent::read(&mut cursor).unwrap();
            println!("channels: {}", wav.fmt.channels);
            println!("format: {}", wav.fmt.format);
            println!("data rate: {}", wav.fmt.data_rate);
            println!("sampling rate: {}", wav.fmt.sampling_rate);
            println!("bytes per sample: {}", wav.fmt.bytes_per_sample);
            println!("bits per sample: {}", wav.fmt.bits_per_sample);
            println!("samples: {:#?}", wav.data);
        }
    }
}