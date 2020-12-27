use rom_res_rs::*;
use std::io::Cursor;
use rom_loaders_rs::images::sprite::BmpSprite;
use rom_media_rs::windowing::{PixelWindowHandler, Key, PixelWindowControlFlow, start_pixel_window, WindowParameters};
use std::time::Duration;
use rom_media_rs::image_rendering::blittable::BlitBuilder;
use rom_media_rs::graphics::PixelSurface;

const MAIN_RES: &[u8] = include_bytes!("MAIN.RES");

struct SpriteRenderingWindow {
    sprite: BmpSprite,
    early_exit: bool,
    surface: PixelSurface
}
impl PixelWindowHandler for SpriteRenderingWindow {
    const FRAME_INTERVAL: Duration = Duration::from_micros(16667);

    fn create(window_params: &WindowParameters) -> Self {
        let cursor = Cursor::new(MAIN_RES);
        let mut resource_file = ResourceFile::new(cursor)
            .expect(&format!("failed to open VIDEO4.RES"));

        let menu_resource = resource_file.get_resource_bytes("graphics/mainmenu/menu_.bmp")
            .expect(&format!("failed to load resource {}", "graphics/mainmenu/menu_.bmp"));

        let sprite = BmpSprite::read_from(&mut Cursor::new(menu_resource))
            .expect(&format!("failed to load resource bmp content"));

        let surface = PixelSurface::create(
            window_params.window_width,
            window_params.window_height
        );

        Self {
            sprite,
            early_exit: false,
            surface
        }
    }

    fn update(&mut self) -> PixelWindowControlFlow {
        if self.early_exit {
            PixelWindowControlFlow::Exit
        } else {
            PixelWindowControlFlow::Continue
        }
    }

    fn prerender(&mut self) {
        let buffer = &mut self.surface.borrow_buffer();
        let w = buffer.width();
        BlitBuilder::new(buffer, w as usize, &self.sprite).blit();
    }

    fn render(&mut self) {
        self.surface.draw(0.0, 0.0, 1.0, 1.0)
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
    start_pixel_window::<SpriteRenderingWindow>(WindowParameters {
        title: "Bmp rendering",
        window_width: 640,
        window_height: 480,
        ..Default::default()
    })
}