use macroquad::{clear_background, draw_window, megaui::{widgets, Vector2}, Vec2, WHITE, WindowParams};

use quad_snd::{mixer::{SoundMixer, PlaybackStyle}, decoder};
use std::io::Cursor;
use rom_res_rs::ResourceFile;
use rom_loaders_rs::multimedia::WavContent;

const MUSIC_RES: &[u8] = include_bytes!("MUSIC.RES");
const SFX_RES: &[u8] = include_bytes!("SFX.RES");
const SPEECH_RES: &[u8] = include_bytes!("SPEECH.RES");

fn reprocess_wav(bytes: &[u8]) -> Vec<u8> {
    let mut wav_file = WavContent::read(&mut Cursor::new(bytes)).unwrap();
    let correction_needed = wav_file.fmt.sampling_rate == 22050;
    let mut reprocessed_audio: Vec<u8> = Vec::new();

    if correction_needed {
        let mut new_vec = Vec::with_capacity(wav_file.data.len() * 2);
        let mut offset = 0;
        while offset < wav_file.data.len() {
            if wav_file.fmt.channels == 1 {
                let sample = wav_file.data[offset];
                offset += 1;
                new_vec.push(sample);
                new_vec.push(sample);
            } else {
                let sample0 = wav_file.data[offset];
                let sample1 = wav_file.data[offset + 1];
                offset += 2;
                new_vec.push(sample0);
                new_vec.push(sample1);
                new_vec.push(sample0);
                new_vec.push(sample1);
            }
        }
        wav_file.fmt.sampling_rate *= 2;
        wav_file.fmt.data_rate *= 2;
        wav_file.data = new_vec;
    }

    wav_file.write(&mut reprocessed_audio).unwrap();
    reprocessed_audio
}

#[macroquad::main("Play ROM sounds")]
async fn main() {
    let (music_cursor, sfx_cursor, speech_cursor) = (
        Cursor::new(MUSIC_RES),
        Cursor::new(SFX_RES),
        Cursor::new(SPEECH_RES)
    );
    if let (
        Ok(music_resource_file),
        Ok(sfx_resource_file),
        Ok(speech_resource_file)
    ) = (
        ResourceFile::new(music_cursor),
        ResourceFile::new(sfx_cursor),
        ResourceFile::new(speech_cursor)
    ) {
        let mut music_resource_file = music_resource_file;
        let mut sfx_resource_file = sfx_resource_file;
        let mut speech_resource_file = speech_resource_file;
        let mut music_resources = music_resource_file.get_resource_list();
        music_resources.sort();
        let mut sfx_resources = sfx_resource_file.get_resource_list();
        sfx_resources = sfx_resources
            .iter()
            .filter(|s| &s[s.len() - 4..] == ".wav")
            .map(|s| s.clone())
            .collect();
        sfx_resources.sort();
        let mut speech_resources = speech_resource_file.get_resource_list();
        speech_resources = speech_resources
            .iter()
            .filter(|s| &s[s.len() - 4..] == ".wav")
            .map(|s| s.clone())
            .collect();
        speech_resources.sort();

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
                                let reprocessed_audio = reprocess_wav(bytes);
                                let decoded_wav = decoder::read_wav_ext(
                                    &reprocessed_audio[..],
                                    PlaybackStyle::Looped,
                                ).unwrap();

                                if let Some(id) = music_sound_id {
                                    music_mixer.stop(id);
                                }

                                music_sound_id = Some(music_mixer.play(decoded_wav));
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
                                let reprocessed_audio = reprocess_wav(bytes);

                                let decoded_wav = decoder::read_wav(
                                    &reprocessed_audio[..]
                                ).unwrap();

                                sfx_mixer.play(decoded_wav);
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
                                let reprocessed_audio = reprocess_wav(bytes);

                                let decoded_wav = decoder::read_wav(
                                    &reprocessed_audio[..]
                                ).unwrap();

                                sfx_mixer.play(decoded_wav);
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