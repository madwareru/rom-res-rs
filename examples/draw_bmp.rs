use rom_res_rs::*;
use std::io::Cursor;
use rom_loaders_rs::images::sprite::BmpSprite;
use rom_media_rs::windowing::{PixelWindowHandler, Key, PixelWindowControlFlow, start_opengl_window, WindowParameters};
use std::time::Duration;
use rom_media_rs::image_rendering::blittable::BlitBuilder;

const MAIN_RES: &[u8] = include_bytes!("MAIN.RES");

struct SpriteRenderingWindow {
    sprite: BmpSprite,
    early_exit: bool
}
impl PixelWindowHandler for SpriteRenderingWindow {
    const TITLE: &'static str = "Bmp rendering";
    const FRAME_INTERVAL: Duration = Duration::from_micros(16667);

    fn update(&mut self) -> PixelWindowControlFlow {
        if self.early_exit {
            PixelWindowControlFlow::Exit
        } else {
            PixelWindowControlFlow::Continue
        }
    }

    fn render(&mut self, buffer: &mut [u32], w: u16, _h: u16) {
        BlitBuilder::new(buffer, w as usize, &self.sprite)
            .blit()
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
    let cursor = Cursor::new(MAIN_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(bmp_file) = resource_file.get_resource_bytes("graphics/mainmenu/menu_.bmp"){
            let bmp_sprite = BmpSprite::read_from(&mut Cursor::new(bmp_file));
            if let Ok(sprite) = bmp_sprite {
                let window = SpriteRenderingWindow { sprite, early_exit: false };
                start_opengl_window(window, WindowParameters {
                    window_width: 640,
                    window_height: 480,
                    scale_up: 1,
                    fullscreen: false
                })
            }
        }
    }
}