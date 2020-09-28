use rom_res_rs::*;
use std::io::Cursor;
use quad_snd::*;
use quad_snd::mixer::SoundMixer;
use wav::BitDepth;

const MUSIC_RES: &[u8] = include_bytes!("MUSIC.RES");

fn main() {
    let cursor = Cursor::new(MUSIC_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(bytes) = resource_file.get_resource_bytes("b10.wav") {
            let (header, old_track) = wav::read(&mut(&bytes[..])).unwrap();

            let track = match old_track {
                BitDepth::Eight(d) => {
                    let mut new_vec = Vec::new();
                    for u_8 in d {
                        new_vec.push(u_8);
                        new_vec.push(u_8);
                    }
                    BitDepth::Eight(new_vec)
                },
                BitDepth::Sixteen(d) => {
                    let mut new_vec = Vec::new();
                    for i_16 in d {
                        new_vec.push(i_16);
                        new_vec.push(i_16);
                    }
                    BitDepth::Sixteen(new_vec)
                },
                BitDepth::TwentyFour(d) => {
                    let mut new_vec = Vec::new();
                    for i_32 in d {
                        new_vec.push(i_32);
                        new_vec.push(i_32);
                    }
                    BitDepth::TwentyFour(new_vec)
                },
                BitDepth::Empty => BitDepth::Empty
            };
            let mut header = header;
            header.sampling_rate = header.sampling_rate * 2;
            header.bytes_per_second = header.bytes_per_second * 2;

            let mut reprocessed_audio: Vec<u8> = Vec::new();

            wav::write(header, track, &mut reprocessed_audio).unwrap();
            let decoded_wav = decoder::read_wav(&reprocessed_audio[..]).unwrap();
            let mut mixer = SoundMixer::new();
            mixer.play(decoded_wav);
            loop {
                mixer.frame();
            }
        }
    }
}