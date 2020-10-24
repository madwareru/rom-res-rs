use macroquad::{clear_background, draw_window, megaui::{widgets, Vector2}, Vec2, WHITE, WindowParams};

use std::io::Cursor;
use rom_res_rs::ResourceFile;
use rom_media_rs::audio::{SoundMixer, Sound, PlaybackBuilder};
use rom_media_rs::audio::mixer::PlaybackStyle;

const MUSIC_RES: &[u8] = include_bytes!("MUSIC.RES");
const SFX_RES: &[u8] = include_bytes!("SFX.RES");
const SPEECH_RES: &[u8] = include_bytes!("SPEECH.RES");

#[macroquad::main("Play ROM sounds")]
async fn main() {
    if let (
        Ok(music_resource_file),
        Ok(sfx_resource_file),
        Ok(speech_resource_file)
    ) = (
        ResourceFile::new(Cursor::new(MUSIC_RES)),
        ResourceFile::new(Cursor::new(SFX_RES)),
        ResourceFile::new(Cursor::new(SPEECH_RES))
    ) {
        let mut music_resource_file = music_resource_file;
        let mut sfx_resource_file = sfx_resource_file;
        let mut speech_resource_file = speech_resource_file;
        let music_resources = music_resource_file.get_resource_list();
        let mut sfx_resources = sfx_resource_file.get_resource_list();
        sfx_resources = sfx_resources
            .iter()
            .filter(|s| &s[s.len() - 4..] == ".wav")
            .map(|s| s.clone())
            .collect();
        let mut speech_resources = speech_resource_file.get_resource_list();
        speech_resources = speech_resources
            .iter()
            .filter(|s| &s[s.len() - 4..] == ".wav")
            .map(|s| s.clone())
            .collect();

        let mut music_mixer = SoundMixer::new();
        let mut sfx_mixer = SoundMixer::new();
        let mut music_sound_id = None;
        loop {
            clear_background(WHITE);
            draw_window(
                0, Vec2::new(20., 20.), Vec2::new(210., 512.),
                Some(WindowParams {
                    label: "MUSIC".to_string(),
                    movable: false,
                    close_button: false,
                    titlebar: true,
                }),
                |ui| {
                    let mut y_pos = 20.0;
                    for i in 0..music_resources.len() {
                        if widgets::Button::new(&music_resources[i])
                            .position(Vector2::new(5., y_pos))
                            .size(Vector2::new(200., 17.))
                            .ui(ui)
                        {
                            if let Ok(bytes) = music_resource_file.get_resource_bytes(&music_resources[i]) {
                                let decoded_wav = Sound::from_bytes_ext(
                                    bytes,
                                    PlaybackStyle::Looped
                                ) .unwrap();

                                if let Some(id) = music_sound_id {
                                    music_mixer.stop(id);
                                }

                                music_sound_id =
                                    music_mixer.play(
                                        PlaybackBuilder::new()
                                        .with_sound(decoded_wav)
                                    );
                            }
                        }
                        y_pos += 30.;
                    }
                });

            draw_window(
                1, Vec2::new(240., 20.), Vec2::new(210., 512.),
                Some(WindowParams {
                    label: "SFX".to_string(),
                    movable: false,
                    close_button: false,
                    titlebar: true,
                }),
                |ui| {
                    let mut y_pos = 20.0;
                    for i in 0..sfx_resources.len() {
                        if widgets::Button::new(&sfx_resources[i])
                            .position(Vector2::new(5., y_pos))
                            .size(Vector2::new(200., 17.))
                            .ui(ui)
                        {
                            if let Ok(bytes) = sfx_resource_file.get_resource_bytes(&sfx_resources[i]) {
                                let decoded_wav = Sound::from_bytes(
                                    bytes
                                ).unwrap();

                                sfx_mixer.play(
                                    PlaybackBuilder::new()
                                        .with_sound(decoded_wav)
                                ).unwrap();
                            }
                        }
                        y_pos += 30.;
                    }
                });

            draw_window(
                2, Vec2::new(460., 20.), Vec2::new(210., 512.),
                Some(
                    WindowParams {
                        label: "SPEECH".to_string(),
                        movable: false,
                        close_button: false,
                        titlebar: true,
                    }),
                |ui| {
                    let mut y_pos = 20.0;
                    for i in 0..speech_resources.len() {
                        if widgets::Button::new(&speech_resources[i])
                            .position(Vector2::new(5., y_pos))
                            .size(Vector2::new(200., 17.))
                            .ui(ui)
                        {
                            if let Ok(bytes) = speech_resource_file.get_resource_bytes(&speech_resources[i]) {
                                let decoded_wav = Sound::from_bytes(
                                    bytes
                                ).unwrap();

                                sfx_mixer.play(
                                    PlaybackBuilder::new()
                                        .with_sound(decoded_wav)
                                ).unwrap();
                            }
                        }
                        y_pos += 30.;
                    }
                });

            music_mixer.frame();
            sfx_mixer.frame();

            macroquad::next_frame().await;
        }
    }
}
