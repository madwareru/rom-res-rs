use rom_res_rs::*;
use std::io::Cursor;
use rom_loaders_rs::images::sprite::BmpSprite;
use rom_media_rs::windowing::{PixelWindowHandler, Key, PixelWindowControlFlow, start_pixel_window, WindowParameters};
use std::time::Duration;
use rom_media_rs::image_rendering::blittable::{BlitBuilder, Blittable, Rect};
use rom_media_rs::graphics::PixelSurface;
use rom_loaders_rs::images::ingame_sprite::{read_image, ImageType, read_palette};
use rom_media_rs::image_rendering::ingame_sprite_decorators::{SpriteRenderingScope, PalettedSpriteRenderingScope};
use rom_media_rs::image_rendering::bmp_sprite_decorators::TrueColorSurfaceSprite;

const MAIN_RES: &[u8] = include_bytes!("MAIN.RES");
const GRAPHICS_RES: &[u8] = include_bytes!("GRAPHICS.RES");
const BUFFER_SIZE: usize = 512;
const STAGE_ATLAS_SIZE: usize = 1024;
const ATLAS_SIZE: usize = 4096;

struct AtlasSubRect {
    pub atlas_id: usize,
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
    pub padding_l: u16,
    pub padding_r: u16,
    pub padding_t: u16,
    pub padding_b: u16,
    pub flag: bool
}

#[derive(Clone, Copy)]
struct ShelfInfo {
    pub y_position: i32,
    pub right: i32,
    pub height: i32,
}

struct SpriteRenderingWindow {
    sprite: BmpSprite,
    tiles_atlas: BmpSprite,
    early_exit: bool,
    surface: PixelSurface
}

struct Highlighter{
    pub foreground_rgb: u32,
    pub background_rgba: u32
}

impl Blittable<u32> for Highlighter {
    fn blit_impl(&self, buffer: &mut [u32], buffer_width: usize, self_rect: Rect, dst_rect: Rect) {
        let foreground_comp0 = self.foreground_rgb & 0xFF;
        let foreground_comp1 = (self.foreground_rgb & 0xFF00) / 0x100;
        let foreground_comp2 = (self.foreground_rgb & 0xFF0000) / 0x10000;
        let mut stride = dst_rect.y_range.start * buffer_width;

        for _ in dst_rect.y_range.start..dst_rect.y_range.start+self_rect.y_range.end {
            for clr in &mut buffer[stride + dst_rect.x_range.start..stride+dst_rect.x_range.start+self_rect.x_range.end].iter_mut() {
                if *clr == 0 {
                    *clr = self.background_rgba;
                } else {
                    let c = *clr;
                    let comp0 = ((c & 0xFF) + foreground_comp0) / 2; let c = c / 0x100;
                    let comp1 = ((c & 0xFF) + foreground_comp1) / 2; let c = c / 0x100;
                    let comp2 = ((c & 0xFF) + foreground_comp2) / 2;
                    *clr = 0xFF000000
                        | (comp2 * 0x10000)
                        | (comp1 * 0x100)
                        | comp0
                }
            }
            stride += buffer_width;
        }
    }

    fn get_width(&self) -> usize {
        32768
    }

    fn get_height(&self) -> usize {
        32768
    }
}

mod units_objects_monsters {
    pub const PATHS: &[&str] = &[
        "structures/bridge1v/house.256",
        "structures/bridge2/house.256",
        "structures/bridge3/house.256",
        "structures/bridge4/house.256",
        "structures/campfire/house.256",
        "structures/castle/house.256",
        "structures/cave/house.256",
        "structures/church/house.256",
        "structures/grave1/house.256",
        "structures/grave2/house.256",
        "structures/grave3/house.256",
        "structures/grave4/house.256",
        "structures/hangman/house.256",
        "units/monsters/ogre/sprites.256",
        "units/monsters/squirrel/sprites.256",
        "units/monsters/star/sprites.256",
        "objects/bush3/dead/sprites.256",
        "objects/bush3/sprites.256",
        "objects/elka1/dead/sprites.256",
        "objects/elka1/sprites.256",
        "objects/elka2/dead/sprites.256",
        "objects/elka2/sprites.256",
        "objects/elka3/dead/sprites.256",
        "objects/elka3/sprites.256",
        "objects/fence/sprites.256",
        "objects/iva1/dead/sprites.256",
        "objects/iva1/sprites.256",
        "objects/iva2/dead/sprites.256",
        "objects/iva2/sprites.256",
        "objects/iva3/dead/sprites.256",
        "objects/iva3/sprites.256",
        "objects/maple1/dead/sprites.256",
        "objects/maple1/sprites.256",
        "objects/maple2/dead/sprites.256",
        "objects/maple2/sprites.256",
        "objects/maple3/dead/sprites.256",
        "objects/maple3/sprites.256",
        "objects/oak1/dead/sprites.256",
        "objects/oak1/sprites.256",
        "objects/oak2/dead/sprites.256",
        "objects/oak2/sprites.256",
        "objects/oak3/dead/sprites.256",
        "objects/oak3/sprites.256",
        "objects/palka/sprites.256",
        "objects/pine1/dead/sprites.256",
        "objects/pine1/sprites.256",
        "objects/pine2/dead/sprites.256",
        "objects/pine2/sprites.256",
        "objects/pine3/dead/sprites.256",
        "objects/pine3/sprites.256",
        "objects/pointer/sprites.256",
        "objects/statue/sprites.256",
        "objects/stones/sprites.256",
        "objects/totem/sprites.256",
        "objects/vallen1/dead/sprites.256",
        "objects/vallen1/sprites.256",
        "objects/vallen2/dead/sprites.256",
        "objects/vallen2/sprites.256",
        "objects/vallen3/dead/sprites.256",
        "objects/vallen3/sprites.256",
        "units/monsters/troll/sprites.256",
        "units/monsters/turtle/sprites.256",
        "objects/bush2/dead/sprites.256",
        "objects/bush2/sprites.256",
        "objects/bones/sprites.256",
        "objects/bush1/dead/sprites.256",
        "objects/bush1/sprites.256",
        "units/monsters/legg/sprites.256",
        "units/monsters/ghost/sprites.256",
        "units/monsters/bat/sprites.256",
        "units/monsters/bee/sprites.256",
        "units/monsters/dragon/sprites.256",
        "units/heroes/archer/sprites.256",
        "units/heroes/axeman/sprites.256",
        "units/heroes/axeman2h/sprites.256",
        "units/heroes/axeman_/sprites.256",
        "units/heroes/clubman/sprites.256",
        "units/heroes/clubman_/sprites.256",
        "units/heroes/mage/sprites.256",
        "units/heroes/mage_st/sprites.256",
        "units/heroes/pikeman/sprites.256",
        "units/heroes/pikeman_/sprites.256",
        "units/heroes/swordsman/sprites.256",
        "units/heroes/swordsman2h/sprites.256",
        "units/heroes/swordsman_/sprites.256",
        "units/heroes/unarmed/sprites.256",
        "units/heroes/unarmed_/sprites.256",
        "units/heroes/xbowman/sprites.256",
        "units/heroes_l/archer/sprites.256",
        "units/heroes_l/axeman/sprites.256",
        "units/heroes_l/axeman2h/sprites.256",
        "units/heroes_l/axeman_/sprites.256",
        "units/heroes_l/clubman/sprites.256",
        "units/heroes_l/clubman_/sprites.256",
        "units/heroes_l/pikeman/sprites.256",
        "units/heroes_l/pikeman_/sprites.256",
        "units/heroes_l/swordsman/sprites.256",
        "units/heroes_l/swordsman2h/sprites.256",
        "units/heroes_l/swordsman_/sprites.256",
        "units/heroes_l/unarmed/sprites.256",
        "units/heroes_l/unarmed_/sprites.256",
        "units/heroes_l/xbowman/sprites.256",
        "units/humans/archer/archer.256",
        "units/humans/axeman/axeman.256",
        "units/humans/axeman_2hd/axeman_2hd.256",
        "units/humans/catapult1/sprites.256",
        "units/humans/catapult2/sprites.256",
        "units/humans/cavalrypike/sprites.256",
        "units/humans/cavalrysword/sprites.256",
        "units/humans/clubman/clubman.256",
        "units/humans/clubman_sh/clubman_sh.256",
        "units/humans/mage/sprites.256",
        "units/humans/mage_st/mage_st.256",
        "units/humans/pikeman_/sprites.256",
        "units/humans/swordsman/swordsman.256",
        "units/humans/swordsman2/swordsman2.256",
        "units/humans/swordsman_/swordsman_.256",
        "units/humans/unarmed/sprites.256",
        "units/humans/xbowman/xbowman.256",
        "units/monsters/orc_s/sprites.256",
        "units/monsters/orc/sprites.256",
        "units/monsters/goblin/sprites.256",
        "units/monsters/goblin_s/sprites.256",
        "structures/hut1/house.256",
        "structures/hut2/house.256",
        "structures/hut3/house.256",
        "structures/hut4/house.256",
        "structures/hut5/house.256",
        "structures/hut6/house.256",
        "structures/hut7(h3)/house.256",
        "structures/hut8(h3)/house.256",
        "structures/hut9(h2)/house.256",
        "structures/huta(h2)/house.256",
        "structures/hutb(o0)/house.256",
        "structures/hutc(o1)/house.256",
        "structures/hutd(o2)/house.256",
        "structures/hute(b0)/house.256",
        "structures/hutf(b1)/house.256",
        "structures/inn1/house.256",
        "structures/inn2/house.256",
        "structures/inn3/house.256",
        "structures/leg's/house.256",
        "structures/magic/house.256",
        "structures/mill1/house.256",
        "structures/mill2/house.256",
        "structures/mill3/house.256",
        "structures/ruins1/house.256",
        "structures/ruins2/house.256",
        "structures/ruins3/house.256",
        "structures/shop1/house.256",
        "structures/shop2/house.256",
        "structures/sphinx1/house.256",
        "structures/sphinx2/house.256",
        "structures/sphinx3/house.256",
        "structures/sphinx4/house.256",
        "structures/sphinx5/house.256",
        "structures/sphinx6/house.256",
        "structures/sphinx7/house.256",
        "structures/sphinx8/house.256",
        "structures/switch1/house.256",
        "structures/switch2/house.256",
        "structures/teleport/house.256",
        "structures/tower1/house.256",
        "structures/tower2/house.256",
        "structures/tower_1/house.256",
        "structures/tower_2/house.256",
        "structures/tower_m/house.256",
        "structures/tower_s1/house.256",
        "structures/tower_s2/house.256",
        "structures/train1/house.256",
        "structures/train2/house.256",
        "structures/train3/house.256",
        "structures/well1/house.256",
        "structures/well2/house.256",
        "structures/well3/house.256",
    ];
}

mod structures {
    pub const PATHS: &[&str] = &[


    ];
}

impl PixelWindowHandler for SpriteRenderingWindow {
    const FRAME_INTERVAL: Duration = Duration::from_micros(16667);

    fn create(window_params: &WindowParameters) -> Self {
        let cursor = Cursor::new(MAIN_RES);
        let mut resource_file = ResourceFile::new(cursor)
            .expect(&format!("failed to open MAIN.RES"));

        let menu_resource = resource_file.get_resource_bytes("graphics/mainmenu/menu_.bmp")
            .expect(&format!("failed to load resource {}", "graphics/mainmenu/menu_.bmp"));

        let sprite = BmpSprite::read_from(&mut Cursor::new(menu_resource))
            .expect(&format!("failed to load resource bmp content"));

        let cursor = Cursor::new(GRAPHICS_RES);
        let mut resource_file = ResourceFile::new(cursor)
            .expect(&format!("failed to open MAIN.RES"));

        let mut sub_rects = Vec::new();
        let green_highlighter = Highlighter {
            foreground_rgb: 0x00DDDD,
            background_rgba: 0x2000DDDD
        };
        let red_highlighter = Highlighter {
            foreground_rgb: 0x00DDDD,
            background_rgba: 0x20FF0000
        };

        let stage_atlases = {
            let mut stage_atlases = Vec::new();
            let mut atlas_id = 0;

            let mut stage_atlas = TrueColorSurfaceSprite::new(STAGE_ATLAS_SIZE, STAGE_ATLAS_SIZE);

            //let mut colors = vec![0; 4096*4096];
            // let mut draw_closure = |path, x, y| {
            //     let tiles_column_resource = resource_file.get_resource_bytes(path)
            //         .expect(&format!("failed to load resource {}", path));
            //
            //     let tiles_column_sprite = BmpSprite::read_from(&mut Cursor::new(tiles_column_resource))
            //         .expect(&format!("failed to load resource bmp content"));
            //
            //     BlitBuilder::create(&mut colors, 4096, &tiles_column_sprite)
            //         .with_dest_pos(x, y)
            //         .blit();
            // };
            // macro_rules! draw_ext {
            //     ($row:expr, $col:expr, $offs:expr) => {
            //         let s = format!("terrain/tile{}-{:02}.bmp", $row, $col);
            //         draw_closure(&s, ((($row-1) % 2) * 16 + $col) * 32, $offs);
            //     }
            // };
            // draw_ext!(1, 0x0, 0);
            // draw_ext!(1, 0x1, 0);
            // draw_ext!(1, 0x2, 0);
            // draw_ext!(1, 0x3, 0);
            // draw_ext!(1, 0x4, 0);
            // draw_ext!(1, 0x5, 0);
            // draw_ext!(1, 0x6, 0);
            // draw_ext!(1, 0x7, 0);
            // draw_ext!(1, 0x8, 0);
            // draw_ext!(1, 0x9, 0);
            // draw_ext!(1, 0xA, 0);
            // draw_ext!(1, 0xB, 0);
            // draw_ext!(1, 0xC, 0);
            // draw_ext!(1, 0xD, 0);
            // draw_ext!(1, 0xE, 0);
            // draw_ext!(1, 0xF, 0);
            //
            // draw_ext!(2, 0x0, 0);
            // draw_ext!(2, 0x1, 0);
            // draw_ext!(2, 0x2, 0);
            // draw_ext!(2, 0x3, 0);
            // draw_ext!(2, 0x4, 0);
            // draw_ext!(2, 0x5, 0);
            // draw_ext!(2, 0x6, 0);
            // draw_ext!(2, 0x7, 0);
            // draw_ext!(2, 0x8, 0);
            // draw_ext!(2, 0x9, 0);
            // draw_ext!(2, 0xA, 0);
            // draw_ext!(2, 0xB, 0);
            // draw_ext!(2, 0xC, 0);
            // draw_ext!(2, 0xD, 0);
            // draw_ext!(2, 0xE, 0);
            // draw_ext!(2, 0xF, 0);
            //
            // draw_ext!(3, 0x0, 14*32);
            // draw_ext!(3, 0x1, 14*32);
            // draw_ext!(3, 0x2, 14*32);
            // draw_ext!(3, 0x3, 14*32);
            // draw_ext!(3, 0x4, 14*32);
            // draw_ext!(3, 0x5, 14*32);
            // draw_ext!(3, 0x6, 14*32);
            // draw_ext!(3, 0x7, 14*32);
            // draw_ext!(3, 0x8, 14*32);
            // draw_ext!(3, 0x9, 14*32);
            // draw_ext!(3, 0xA, 14*32);
            // draw_ext!(3, 0xB, 14*32);
            // draw_ext!(3, 0xC, 14*32);
            // draw_ext!(3, 0xD, 14*32);
            // draw_ext!(3, 0xE, 14*32);
            // draw_ext!(3, 0xF, 14*32);
            //
            // draw_ext!(4, 0x0, 14*32);
            // draw_ext!(4, 0x1, 14*32);
            // draw_ext!(4, 0x2, 14*32);
            // draw_ext!(4, 0x3, 14*32);

            let mut x_pos = 0;
            let mut y_pos = 0;
            let mut max_h = 0;

            let mut sp = TrueColorSurfaceSprite::new(
                BUFFER_SIZE,
                BUFFER_SIZE
            );
            let blk = TrueColorSurfaceSprite::new(
                BUFFER_SIZE,
                BUFFER_SIZE
            );

            for unit_path in units_objects_monsters::PATHS {
                let unit_resource = resource_file
                    .get_resource_bytes(unit_path)
                    .expect(&format!("failed to load resource {}", unit_path));

                let unit_sprite =
                    read_image(
                        &mut Cursor::new(unit_resource),
                        ImageType::Dot256
                    ).expect(&format!("failed to load resource bmp content"));
                let palette =
                    read_palette(
                        &mut Cursor::new(unit_resource),
                        ImageType::Dot256
                    ).unwrap().unwrap();

                for i in 0..unit_sprite.frames.len() {
                    let frame = &(unit_sprite.frames[i]);
                    BlitBuilder::try_create(&mut sp, &blk).unwrap()
                        .with_source_subrect(0, 0, frame.width as usize, frame.height as usize)
                        .blit(); // clear background

                    let scope = &PalettedSpriteRenderingScope{
                        image_data: &unit_sprite,
                        palette: &palette,
                        img_id: i
                    };

                    BlitBuilder::try_create(&mut sp, scope).unwrap().blit();
                    let (mut min_i, mut min_j) = (frame.width as usize - 1, frame.height as usize - 1);
                    let (mut max_i, mut max_j) = (0, 0);
                    for jj in 0..frame.height as usize {
                        for ii in 0..frame.width as usize {
                            let offset = sp.get_width() * jj + ii;
                            if sp.color_data()[offset] == 0 { continue; }
                            min_i = min_i.min(ii);
                            min_j = min_j.min(jj);
                            max_i = max_i.max(ii);
                            max_j = max_j.max(jj);
                        }
                    }
                    if min_i >= max_i || min_j >= max_j {continue;}
                    let true_h = max_j - min_j + 1;
                    let true_w = max_i - min_i + 1;

                    if (x_pos + true_w as i32) as usize >= STAGE_ATLAS_SIZE {
                        x_pos = 0;
                        y_pos += max_h;
                        max_h = 0;
                    }

                    if (y_pos + true_h as i32) as usize>= STAGE_ATLAS_SIZE {
                        atlas_id += 1;
                        x_pos = 0;
                        y_pos = 0;
                        max_h = 0;
                        let staged = stage_atlas;
                        stage_atlas = TrueColorSurfaceSprite::new(STAGE_ATLAS_SIZE, STAGE_ATLAS_SIZE);
                        stage_atlases.push(staged)
                    }

                    BlitBuilder::try_create(&mut stage_atlas, &sp)
                        .unwrap()
                        .with_dest_pos(x_pos, y_pos)
                        .with_source_subrect(min_i, min_j, true_w, true_h)
                        .blit();

                    let sub_rect = AtlasSubRect {
                        atlas_id,
                        x: x_pos as usize,
                        y: y_pos as usize,
                        w: true_w as usize,
                        h: true_h as usize,
                        padding_l: min_i as u16,
                        padding_r: (frame.width as usize - true_w - min_i) as u16,
                        padding_t: min_j as u16,
                        padding_b: (frame.height as usize - true_h - min_j) as u16,
                        flag: false
                    };

                    sub_rects.push(sub_rect);

                    x_pos += true_w as i32;
                    max_h = max_h.max(true_h as i32);
                }
            }
            stage_atlases.push(stage_atlas);
            stage_atlases
        };

        sub_rects.sort_by(|l, r|{
            if l.h == r.h {
                r.w.cmp(&(l.w))
            }
            else {
                r.h.cmp(&(l.h))
            }
        });

        let (mut colors, sub_rects) = {
            let mut shelfs: Vec<ShelfInfo> = Vec::new();
            let mut current_shelf = ShelfInfo {y_position: 0, right: 0, height: 0 };

            let mut new_colors = TrueColorSurfaceSprite::new(ATLAS_SIZE, ATLAS_SIZE);
            let mut new_sub_rects = Vec::new();

            for sub_rect in sub_rects.iter() {

                let true_h = sub_rect.h as usize;
                let true_w = sub_rect.w as usize;

                if let Some(matched_shelf) = shelfs
                    .iter_mut()
                    .find(|s| {
                        (s.height >= true_h as i32) &&
                        ((s.right + true_w as i32) as usize) < ATLAS_SIZE
                    })
                {
                    let y_pos = matched_shelf.y_position;
                    let x_pos = matched_shelf.right;

                    BlitBuilder::try_create(&mut new_colors, &stage_atlases[sub_rect.atlas_id])
                        .unwrap()
                        .with_dest_pos(x_pos, y_pos)
                        .with_source_subrect(
                            sub_rect.x,
                            sub_rect.y,
                            true_w,
                            true_h
                        ).blit();
                    matched_shelf.right += true_w as i32;
                    let new_sub_rect = AtlasSubRect {
                        atlas_id: 0,
                        x: x_pos as usize,
                        y: y_pos as usize,
                        w: true_w as usize,
                        h: true_h as usize,
                        padding_l: sub_rect.padding_l,
                        padding_r: sub_rect.padding_r,
                        padding_t: sub_rect.padding_t,
                        padding_b: sub_rect.padding_b,
                        flag: true
                    };
                    new_sub_rects.push(new_sub_rect);
                } else {
                    if (current_shelf.right + true_w as i32) as usize >= ATLAS_SIZE {
                        shelfs.push(current_shelf);
                        current_shelf.right = 0;
                        current_shelf.y_position += current_shelf.height;
                        current_shelf.height = 0;
                    }
                    let y_pos = current_shelf.y_position;
                    let x_pos = current_shelf.right;
                    BlitBuilder::try_create(&mut new_colors, &stage_atlases[sub_rect.atlas_id])
                        .unwrap()
                        .with_dest_pos(x_pos, y_pos)
                        .with_source_subrect(
                            sub_rect.x,
                            sub_rect.y,
                            true_w,
                            true_h
                        ).blit();
                    current_shelf.right += true_w as i32;
                    current_shelf.height = current_shelf.height.max(true_h as i32);

                    let new_sub_rect = AtlasSubRect {
                        atlas_id: 0,
                        x: x_pos as usize,
                        y: y_pos as usize,
                        w: true_w as usize,
                        h: true_h as usize,
                        padding_l: sub_rect.padding_l,
                        padding_r: sub_rect.padding_r,
                        padding_t: sub_rect.padding_t,
                        padding_b: sub_rect.padding_b,
                        flag: false
                    };
                    new_sub_rects.push(new_sub_rect);
                }
            }
            (new_colors, new_sub_rects)
        };

        // for sub_rect in sub_rects.iter() {
        //     if sub_rect.flag {
        //         BlitBuilder::try_create(&mut colors, &red_highlighter)
        //             .unwrap()
        //             .with_dest_pos(sub_rect.x as i32, sub_rect.y as i32)
        //             .with_source_subrect(
        //                 0,
        //                 0,
        //                 sub_rect.w,
        //                 sub_rect.h
        //             ).blit();
        //     } else {
        //         BlitBuilder::try_create(&mut colors, &green_highlighter)
        //             .unwrap()
        //             .with_dest_pos(sub_rect.x as i32, sub_rect.y as i32)
        //             .with_source_subrect(
        //                 0,
        //                 0,
        //                 sub_rect.w,
        //                 sub_rect.h
        //             ).blit();
        //     }
        // }

        let colors = Vec::from(colors.color_data());
        {
            use std::path::Path;
            use std::fs::File;
            use std::io::BufWriter;

            let path = Path::new(r"/users/madware/Documents/units.png");
            let file = File::create(path).unwrap();
            let ref mut w = BufWriter::new(file);

            let mut encoder = png::Encoder::new(w, ATLAS_SIZE as u32, ATLAS_SIZE as u32);
            encoder.set_color(png::ColorType::RGBA);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            let mut data = Vec::with_capacity(ATLAS_SIZE*ATLAS_SIZE*4);
            for c in (&colors).iter() {
                let mut clr = *c;
                let b = clr & 0xFF; clr /= 0x100;
                let g = clr & 0xFF; clr /= 0x100;
                let r = clr & 0xFF; clr /= 0x100;
                data.push(r as u8);
                data.push(g as u8);
                data.push(b as u8);
                data.push(clr as u8);
            }
            writer.write_image_data(&data).unwrap(); // Save
        }

        let tiles_atlas = BmpSprite::TrueColor {
            width: ATLAS_SIZE,
            height: ATLAS_SIZE,
            colors
        };

        let surface = PixelSurface::create(
            window_params.window_width,
            window_params.window_height
        );

        Self {
            sprite,
            tiles_atlas,
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
        let buffer = &mut self.surface.get_buffer_mut();
        let w = buffer.width();
        BlitBuilder::create(buffer, w as usize, &self.sprite).blit();
        BlitBuilder::create(buffer, w as usize, &self.tiles_atlas).blit();
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
        window_width: 1024,
        window_height: 768,
        ..Default::default()
    })
}