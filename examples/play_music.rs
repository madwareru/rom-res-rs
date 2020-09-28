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
        if let Ok(bytes) = resource_file.get_resource_bytes("inn.wav") {
            let (header, data) = wav::read(&mut(&bytes[..])).unwrap();
            let mut new_vec = Vec::new();
            match data {
                BitDepth::Eight(d) => {assert!(false);}
                BitDepth::Sixteen(d) => {
                    for i_16 in d {
                        new_vec.push(i_16 / 2);
                        new_vec.push(i_16 / 2);
                    }
                }
                BitDepth::TwentyFour(d) => {assert!(false);}
                BitDepth::Empty => {assert!(false);}
            }
            let mut header = header;
            header.sampling_rate = 22050 * 2;
            header.bytes_per_sample = 4;
            header.bytes_per_second = 88200 * 2;
            let track = BitDepth::Sixteen(new_vec);

            let mut reprocessed_audio: Vec<u8> = Vec::new();

            wav::write(header, track, &mut reprocessed_audio);
            let decoded_wav = decoder::read_wav(&reprocessed_audio[..]).unwrap();
            let mut mixer = SoundMixer::new();
            mixer.play(decoded_wav);
            loop {
                mixer.frame();
            }
        }
    }
}