use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::io::Cursor;

use crate::Sound;

pub fn play_sound(sound: Sound) -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    let bytes: Box<&[u8]> = match sound {
        Sound::JobsDone => Box::new(include_bytes!("../sounds/jobs-done.wav")),
        Sound::WorkComplete => Box::new(include_bytes!("../sounds/work-complete.wav")),
        Sound::ZugZug => Box::new(include_bytes!("../sounds/zug-zug.wav")),
    };

    let source = Decoder::new_wav(Cursor::new(*bytes)).expect("Unable to decode WAV file");

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
