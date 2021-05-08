use miniquad::*;
use rom_media_rs::image_rendering::bmp_sprite_decorators::TrueColorSurfaceSprite;
use rom_media_rs::image_rendering::blittable::{Blittable, BlitBuilder};
use std::io::Cursor;
use rom_res_rs::ResourceFile;
use rom_media_rs::video::{SmackerPlayer, PlayerState, RenderingFramesState};
use std::time::Instant;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    stage_surface: TrueColorSurfaceSprite,
    player: SmackerPlayer,
    last_instant: Instant,
}

const VIDEO4_RES: &[u8] = include_bytes!("VIDEO4.RES");
const VIDEO_PATH: &str = "INTRO/04.smk";

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
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

        let last_instant = Instant::now();

        #[rustfmt::skip]
            let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -1., y: -1. }, uv: Vec2 { x: 0., y: 1. } },
            Vertex { pos : Vec2 { x:  1., y: -1. }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x:  1., y:  1. }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x: -1., y:  1. }, uv: Vec2 { x: 0., y: 0. } },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let stage_surface = TrueColorSurfaceSprite::new(320, 240);
        let casted = bytemuck::cast_slice(stage_surface.color_data());
        let texture = Texture::from_data_and_format(
            ctx,
            &casted,
            TextureParams {
                format: TextureFormat::RGBA8,
                wrap: TextureWrap::Clamp,
                filter: FilterMode::Nearest,
                width: stage_surface.get_width() as u32,
                height: stage_surface.get_height() as u32
            }
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        Stage {
            pipeline,
            bindings,
            stage_surface,
            player,
            last_instant
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {
        let dt = self.last_instant.elapsed().as_micros() as f32 / 1000.0;
        self.last_instant = Instant::now();
        match self.player.frame(dt).unwrap() {
            PlayerState::FinishedPlaying => {
                _ctx.quit()
            },
            PlayerState::FadeIn(_)
            |
            PlayerState::FadeOut(_)
            |
            PlayerState::IsRendering {
                state: RenderingFramesState::RenderedNewFrame,
                ..
            }=> {
                BlitBuilder::try_create(&mut self.stage_surface, &self.player)
                    .expect("failed to create blit builder")
                    .with_dest_pos(0, 30)
                    .blit();

                let casted = bytemuck::cast_slice(self.stage_surface.color_data());
                self.bindings.images[0].update(_ctx, casted);
            },
            _ => ()
        }

    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        ctx.apply_uniforms(&shader::Uniforms {
            offset: (0.0, 0.0),
        });

        ctx.draw(0, 6, 1);

        ctx.end_render_pass();

        ctx.commit_frame();
    }
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;

    uniform vec2 offset;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(pos + offset, 0, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
    }
}

fn main() {
    miniquad::start(conf::Conf {
        window_width: 640,
        window_height: 480,
        window_title: "play_smacker_miniquad".to_string(),
        ..Default::default()
    }, |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}