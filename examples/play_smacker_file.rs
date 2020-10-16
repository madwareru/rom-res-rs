use rom_res_rs::*;
use std::io::{Cursor, Write};
use minifb::{Window, WindowOptions, Key};
use rom_media_rs::video::{SmackerPlayer, PlayerState, FadeInState, RenderingFramesState};
use std::time::Instant;

const VIDEO4_RES: &[u8] = include_bytes!("VIDEO4.RES");
const VIDEO_PATH: &str = "M10/01.smk";

fn main() {
    let cursor = Cursor::new(VIDEO4_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(smk_file) = resource_file.get_resource_bytes(VIDEO_PATH){
            let mut cursor = Cursor::new(smk_file);
            if let Ok(player) = SmackerPlayer::load_from_stream(&mut cursor) {
                let mut player = player;
                player.set_fade_in_ms(800);
                player.set_fade_out_ms(800);
                let(w, h) = (player.frame_width, player.frame_height);
                println!("width: {}, height: {}", w, h);

                let mut buffer = vec![0xFF000000u32; w * h];
                let (win_w, win_h) = if w < 400 {
                    (w * 4, h * 4)
                } else if w < 700 {
                    (w * 2, h * 2)
                } else if w <= 800 {
                    ((w * 3) / 2, (h * 3) / 2)
                } else {
                    (w, h)
                };
                let mut window = Window::new(VIDEO_PATH, win_w, win_h, WindowOptions::default())
                    .unwrap_or_else(|e| { panic!("{}", e); });

                window.limit_update_rate(Some(std::time::Duration::from_micros(16660)));

                let mut instant = Instant::now();
                let mut first_frame = true;
                while window.is_open() && !window.is_key_down(Key::Escape) {
                    let dt = instant.elapsed().as_micros() as f32 / 1000.0;
                    instant = Instant::now();
                    if first_frame {
                        first_frame = false;
                        window.update_with_buffer(&buffer, w, h).unwrap();
                        continue;
                    }
                    match player.frame(dt).unwrap() {
                        PlayerState::FinishedPlaying => {
                            break;
                        },
                        PlayerState::FadeIn(_) => {
                            player.blit_picture(&mut buffer, 0, 0, w);
                            window.update_with_buffer(&buffer, w, h).unwrap();
                        }
                        PlayerState::IsRendering {
                            state: RenderingFramesState::RenderedNewFrame,
                            ..
                        } => {
                            player.blit_picture(&mut buffer, 0, 0, w);
                            window.update_with_buffer(&buffer, w, h).unwrap();
                        }
                        PlayerState::FadeOut(_) => {
                            player.blit_picture(&mut buffer, 0, 0, w);
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