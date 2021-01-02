use blinds::{Window, EventStream, run_gl, Settings, Event, Key};
use golem::{GolemError, Context, Texture, ColorFormat, ShaderProgram, ShaderDescription, Attribute, AttributeType, Uniform, UniformType, VertexBuffer, ElementBuffer, UniformValue, GeometryMode, TextureFilter};
use golem::Dimension::D2;
use std::io::Cursor;
use rom_res_rs::ResourceFile;
use rom_media_rs::video::{SmackerPlayer, PlayerState, RenderingFramesState};
use std::time::Instant;
use rom_media_rs::image_rendering::blittable::BlitBuilder;
use blinds::event::KeyboardEvent;

const VIDEO4_RES: &[u8] = include_bytes!("VIDEO4.RES");
const VIDEO_PATH: &str = "INTRO/04.smk";

async fn app(
    window: Window,
    ctx: golem::glow::Context,
    mut events: EventStream,
) -> Result<(), GolemError> {
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

    let mut last_instant = Instant::now();

    let ctx = &Context::from_glow(ctx)?;

    let mut stage_buffer = Vec::new();
    stage_buffer.resize(320 * 240, 0x0u32);
    let mut texture = Texture::new(&ctx)?;
    texture.set_magnification(TextureFilter::Nearest);
    texture.set_minification(TextureFilter::Nearest);
    texture.set_wrap_h(golem::TextureWrap::ClampToEdge);
    texture.set_wrap_v(golem::TextureWrap::ClampToEdge);
    let casted = bytemuck::cast_slice_mut(&mut stage_buffer);
    texture.set_image(
        Some(&casted),
        320 as u32,
        240 as u32,
        ColorFormat::RGBA
    );

    #[rustfmt::skip]
        let vertices = [
        // Position         UV
        -1.0,  1.0,        0.0, 0.0,
         1.0,  1.0,        1.0, 0.0,
         1.0, -1.0,        1.0, 1.0,
        -1.0, -1.0,        0.0, 1.0,
    ];
    let indices = [0, 1, 2, 2, 3, 0];

    let mut shader = ShaderProgram::new(
        ctx,
        ShaderDescription {
            vertex_input: &[
                Attribute::new("vert_position", AttributeType::Vector(D2)),
                Attribute::new("vert_uv", AttributeType::Vector(D2)),
            ],
            fragment_input: &[Attribute::new("frag_uv", AttributeType::Vector(D2))],
            uniforms: &[Uniform::new("image", UniformType::Sampler2D)],
            vertex_shader: r#" void main() {
                gl_Position = vec4(vert_position, 0, 1);
                frag_uv = vert_uv;
            }"#,
                fragment_shader: r#" void main() {
                gl_FragColor = texture(image, frag_uv);
            }"#,
        },
    )?;

    let mut vb = VertexBuffer::new(ctx)?;
    let mut eb = ElementBuffer::new(ctx)?;
    vb.set_data(&vertices);
    eb.set_data(&indices);
    shader.bind();

    shader.set_uniform("image", UniformValue::Int(1))?;

    let bind_point = std::num::NonZeroU32::new(1).unwrap();
    texture.set_active(bind_point);

    loop {
        let dt = last_instant.elapsed().as_micros() as f32 / 1000.0;
        last_instant = Instant::now();

        match player.frame(dt).unwrap() {
            PlayerState::FinishedPlaying => {
                break Ok(())
            },
            PlayerState::FadeIn(_)
            |
            PlayerState::IsRendering {
                state: RenderingFramesState::RenderedNewFrame,
                ..
            }
            |
            PlayerState::FadeOut(_) => {
                BlitBuilder::create(&mut stage_buffer, 320, &player)
                    .with_dest_pos(0, 30)
                    .blit();

                let casted = bytemuck::cast_slice_mut(&mut stage_buffer);
                texture.set_image(
                    Some(&casted),
                    320 as u32,
                    240 as u32,
                    ColorFormat::RGBA
                );
            },
            _ => ()
        }

        ctx.clear();
        unsafe {
            shader.draw(&vb, &eb, 0..indices.len(), GeometryMode::Triangles)?;
        }
        window.present();

        match events.next_event().await {
            None => (),
            Some(event) => {
                match event {
                    Event::KeyboardInput(keyEvent) => {
                        if keyEvent.key() == Key::Escape {
                            break Ok(())
                        }
                    },
                    _ => ()
                }
            }
        }
    }
}

fn main() {
    run_gl(Settings{
        size: [640.0, 480.0].into(),
        resizable: false,
        fullscreen: true,
        cursor_icon: None,
        ..Default::default()
    },
        |window, gfx, events| async move {
             app(window, gfx, events).await.unwrap()
        }
    );
}