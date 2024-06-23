use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, Error};
use rodio::source::SamplesConverter;
use rodio::{Decoder, OutputStream, source::Source, decoder::DecoderError};

use crate::constants::GONG_PATH;

// SoundPlayer Interface
pub trait Player {
    // Open a path and return the stream
    fn open_file() -> File;

    // Store a stream to instance
    fn set_sounds(audio_file: File) -> SoundsLibrary;

    // Play a stream
    fn play_sound(self);
}


pub struct SoundsLibrary {
    work_sound: SamplesConverter<Decoder<BufReader<File>>, f32>,
    // break_sound: Decoder<BufReader<File>>,
    // session_end_sound: Decoder<BufReader<File>>,
    // warning_sound: Decoder<BufReader<File>>,
}


impl Player for SoundsLibrary {
    fn open_file() -> File {
        // Define the path
        let path: PathBuf = PathBuf::from(GONG_PATH);

        // Open File
        let file: Result<File, Error> = File::open(path);

        // Return file if it was readable
        match file {
            Ok(output) => {
                return output;
            }
            Err(e) => {
                panic!("Could not read file: {}", e);
            }
        }
    }

    fn set_sounds(audio_file: File) -> SoundsLibrary {
        let source = BufReader::new(audio_file);
        let sound: Result<Decoder<BufReader<File>>, DecoderError> = Decoder::new(source);

        match sound {
            Ok(sound_stream) => {
                let library = SoundsLibrary {
                    work_sound: sound_stream.convert_samples()
                };
                // self.break_sound = sound_stream;
                // self.session_end_sound = sound_stream;
                // self.warning_sound = sound_stream;
                return library;
            }
            Err(e) => {
                panic!("Failed to decode! {}", e)
            }
        }
    }

    fn play_sound(self) {
        let stream_utils: Result<(OutputStream, rodio::OutputStreamHandle), rodio::StreamError> = OutputStream::try_default();

        match stream_utils {
            Ok(utils) => {
                let (_stream, stream_handle) = utils;
                let _ = stream_handle.play_raw(self.work_sound);
            }
            Err(e) => {
                panic!("Failed to initialize audio stream: {}", e)
            }
        }
    }
}