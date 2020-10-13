use rom_res_rs::*;
use std::io::{Cursor};
use minifb::{Window, WindowOptions, Key};
use rom_media_rs::video::{SmackerPlayer, PlayerState};
use std::time::Instant;

const VIDEO8_RES: &[u8] = include_bytes!("VIDEO8.RES");
const VIDEO_PATH: &str = "INTRO/04.smk";

fn main() {
    let cursor = Cursor::new(VIDEO8_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(reg_file) = resource_file.get_resource_bytes(VIDEO_PATH){
            let mut cursor = Cursor::new(reg_file);
            if let Ok(player) = SmackerPlayer::load_from_stream(&mut cursor) {
                let mut player = player;
                let(w, h) = (player.frame_width + 10, player.frame_height + 10);
                println!("width: {}, height: {}", w, h);

                let mut buffer = vec![0u32; w * h];
                let mut window = Window::new(VIDEO_PATH, w * 4, h * 4, WindowOptions::default())
                    .unwrap_or_else(|e| { panic!("{}", e); });

                window.limit_update_rate(Some(std::time::Duration::from_micros(33330)));

                let mut instant = Instant::now();
                while window.is_open() && !window.is_key_down(Key::Escape) {
                    let dt = instant.elapsed().as_micros() as f32 / 1000.0;
                    instant = Instant::now();
                    match player.frame(dt, false, true).unwrap() {
                        PlayerState::FinishedPlaying => {
                            break;
                        },
                        PlayerState::RenderedNewFrame => {
                            player.blit_picture(&mut buffer, 5, 5, w);
                            window.update_with_buffer(&buffer, w, h).unwrap();
                        },
                        _ => {
                            window.update();
                        }
                    }
                }
            }
        }
    }
}