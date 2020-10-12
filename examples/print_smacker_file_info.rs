use rom_res_rs::*;
use std::io::Cursor;

mod hex_print_utils;
use hex_print_utils::*;
use rom_loaders_rs::multimedia::SmackerFile;
use minifb::{Window, WindowOptions, Key};
use std::ptr;

const VIDEO8_RES: &[u8] = include_bytes!("VIDEO8.RES");

fn scale_2x_unsafy(pixels: &[u32], output: &mut Vec<u32>, width: u32, height: u32) {
    let whole_size = (width * height * 4) as usize;
    let dbl_width = (width * 2) as usize;

    let mut pp= pixels.as_ptr();
    let pp_out = &mut output[0..whole_size];
    let mut pp_out = pp_out.as_mut_ptr();

    unsafe {
        for _ in 0..height {
            for _ in 0..width {
                *pp_out = *pp; pp_out = pp_out.add(1);
                *pp_out = *pp; pp_out = pp_out.add(1);
                pp = pp.add(1);
            }
            ptr::copy_nonoverlapping(pp_out.sub(dbl_width), pp_out, dbl_width);
            pp_out = pp_out.add(dbl_width);
        }
    }
}

fn main() {
    let cursor = Cursor::new(VIDEO8_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(reg_file) = resource_file.get_resource_bytes("INTRO/04.smk"){
            let mut cursor = Cursor::new(reg_file);
            if let Ok(smk) = SmackerFile::load(&mut cursor) {
                let mut smk = smk;
                let(w, h) = (smk.file_info.width as usize, smk.file_info.height as usize);
                println!("width: {}, height: {}", w, h);
                println!("frame count: {}, frame interval: {}", smk.file_info.frames.len(), smk.file_info.frame_interval);
                println!("sound sample rate: {}", smk.file_info.audio_rate[0]);

                let mut small_buffer = vec![0u32; w * h];
                let mut medium_buffer = vec![0u32; w * h * 4];
                let mut buffer = vec![0u32; w * h * 16];
                let mut window = Window::new("Test - ESC to exit", w * 4, h * 4,
                    WindowOptions::default(),
                )
                .unwrap_or_else(|e| {
                    panic!("{}", e);
                });

                window.limit_update_rate(Some(std::time::Duration::from_micros(
                    (smk.file_info.frame_interval * 1000.0) as u64
                )));
                let mut frame = 0;

                while window.is_open() && !window.is_key_down(Key::Escape) {
                    if frame < smk.file_info.frames.len() {
                        smk.unpack(frame, false, true).unwrap();
                        frame += 1;
                    }
                    let mut offset = 0;
                    for i in small_buffer.iter_mut() {
                        *i = smk.file_info.smacker_decode_context.palette[smk.file_info.smacker_decode_context.image[offset] as usize];
                        offset += 1;
                    }
                    scale_2x_unsafy(&small_buffer, &mut medium_buffer, w as u32, h as u32);
                    scale_2x_unsafy(&medium_buffer, &mut buffer, (w * 2) as u32, (h * 2) as u32);
                    window
                        .update_with_buffer(&buffer, w * 4, h * 4)
                        .unwrap();
                }

            } else {
                print_hex(reg_file);
            }
        }
    }
}