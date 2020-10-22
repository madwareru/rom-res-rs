use rom_res_rs::*;
use std::io::{Cursor};
use rom_media_rs::video::{SmackerPlayer, PlayerState, RenderingFramesState};
use std::time::{Instant, Duration};
use rom_media_rs::windowing::{PixelWindowHandler, PixelWindowControlFlow, start_opengl_window, WindowParameters, Key};

const VIDEO4_RES: &[u8] = include_bytes!("VIDEO4.RES");
const VIDEO_PATH: &str = "INTRO/04.smk";

struct SmackerPlayerWindow {
    player: SmackerPlayer,
    last_instant: Instant,
    frame_dirty: bool,
    early_exit: bool
}
impl SmackerPlayerWindow {
    fn new(player: SmackerPlayer) -> Self {
        Self {
            player,
            last_instant: Instant::now(),
            frame_dirty: false,
            early_exit: false
        }
    }
}

impl PixelWindowHandler for SmackerPlayerWindow {
    const TITLE: &'static str = "Smacker playback";
    const FRAME_INTERVAL: Duration = Duration::from_micros(16660);

    fn update(&mut self) -> PixelWindowControlFlow {
        if self.early_exit {
            return PixelWindowControlFlow::Exit;
        }
        let dt = self.last_instant.elapsed().as_micros() as f32 / 1000.0;
        self.last_instant = Instant::now();
        match self.player.frame(dt).unwrap() {
            PlayerState::FinishedPlaying => {
                PixelWindowControlFlow::Exit
            },
            PlayerState::FadeIn(_) => {
                self.frame_dirty = true;
                PixelWindowControlFlow::Continue
            }
            PlayerState::IsRendering {
                state: RenderingFramesState::RenderedNewFrame,
                ..
            } => {
                self.frame_dirty = true;
                PixelWindowControlFlow::Continue
            }
            PlayerState::FadeOut(_) => {
                self.frame_dirty = true;
                PixelWindowControlFlow::Continue
            },
            _ => PixelWindowControlFlow::Continue
        }
    }

    fn render(&mut self, buffer: &mut [u32], w: u16, _h: u16) {
        if self.frame_dirty {
            self.player.blit_picture(buffer, 0, 30, w as usize);
            self.frame_dirty = false;
        }
    }

    fn on_key_pressed(&mut self, key: Key) {
        if key == Key::Escape {
            self.early_exit = true;
        }
    }

    fn on_key_released(&mut self, _key: Key) {

    }

    fn on_mouse_moved(&mut self, _x: f64, _y: f64) {

    }

    fn on_mouse_button_pressed(&mut self, _button_id: u8) {

    }

    fn on_mouse_button_released(&mut self, _button_id: u8) {

    }
}

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
                let(w, h) = (player.frame_width as u16, player.frame_height as u16);
                println!("width: {}, height: {}", w, h);

                let smacker_window = SmackerPlayerWindow::new(player);
                start_opengl_window(smacker_window, WindowParameters{
                    window_width: 320,
                    window_height: 240,
                    fullscreen: true,
                    scale_up: 2
                })
            }
        }
    }
}