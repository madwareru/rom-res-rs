use rom_res_rs::*;
use std::io::{Cursor};
use rom_media_rs::video::{SmackerPlayer, PlayerState, RenderingFramesState};
use std::time::{Instant, Duration};
use rom_media_rs::windowing::{PixelWindowHandler, PixelWindowControlFlow, start_pixel_window, WindowParameters, Key};
use rom_media_rs::image_rendering::blittable::BlitBuilder;
use rom_media_rs::graphics::PixelSurface;

const VIDEO4_RES: &[u8] = include_bytes!("VIDEO4.RES");
const VIDEO_PATH: &str = "INTRO/04.smk";

struct SmackerPlayerWindow {
    player: SmackerPlayer,
    last_instant: Instant,
    frame_dirty: bool,
    early_exit: bool,
    surface: PixelSurface
}

impl PixelWindowHandler for SmackerPlayerWindow {
    const FRAME_INTERVAL: Duration = Duration::from_micros(16660);

    fn create(window_params: &WindowParameters) -> Self {
        let cursor = Cursor::new(VIDEO4_RES);
        let mut resource_file = ResourceFile::new(cursor)
            .expect(&format!("failed to open VIDEO4.RES"));

        let smk_file = resource_file.get_resource_bytes(VIDEO_PATH)
            .expect(&format!("failed to load resource {}", VIDEO_PATH));

        let mut cursor = Cursor::new(smk_file);

        let mut player = SmackerPlayer::load_from_stream(&mut cursor)
            .expect("failed to load smacker file");

        player.set_fade_in_ms(800);
        player.set_fade_out_ms(800);

        let surface = PixelSurface::create(
            window_params.window_width,
            window_params.window_height
        );

        Self {
            player,
            last_instant: Instant::now(),
            frame_dirty: false,
            early_exit: false,
            surface
        }
    }

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

    fn prerender(&mut self) {
        if self.frame_dirty {
            let buffer = &mut self.surface.get_buffer_mut();
            let w = buffer.width();
            BlitBuilder::create(buffer, w, &self.player)
                .with_dest_pos(0, 30)
                .blit();
            self.frame_dirty = false;
        }
    }

    fn render(&mut self) {
        self.surface.draw(0.0, 0.0, 1.0, 1.0);
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

    fn on_mouse_button_pressed(&mut self, _button_id: u16) {

    }

    fn on_mouse_button_released(&mut self, _button_id: u16) {

    }

    fn on_window_closed(&mut self) {

    }
}

fn main() {
    start_pixel_window::<SmackerPlayerWindow>(WindowParameters{
        title: "Smacker playback",
        window_width: 320,
        window_height: 240,
        fullscreen: true,
        scale_up: 2,
        cursor_visible: false
    })
}