//! Plays an audio file through MVerb with default parameters

use rodio::{source::SamplesConverter, source::Source, Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

use mverb_rs::MVerb;

struct MVerbSource {
    reverb: MVerb,
    samples_converter: SamplesConverter<Decoder<BufReader<File>>, f32>,
}

impl MVerbSource {
    fn new(samples_converter: SamplesConverter<Decoder<BufReader<File>>, f32>) -> Self {
        let mut reverb = MVerb::default();

        reverb.set_sample_rate(samples_converter.sample_rate() as f32);

        Self {
            reverb,
            samples_converter,
        }
    }
}

impl Iterator for MVerbSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sample) = self.samples_converter.next() {
            Some(self.reverb.process((sample, sample)).0 + sample)
        } else {
            None
        }
    }
}

impl Source for MVerbSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn sample_rate(&self) -> u32 {
        self.samples_converter.sample_rate()
    }

    fn channels(&self) -> u16 {
        self.samples_converter.channels()
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn main() {
    let file_path = "examples/arpeggio.mp3";
    // let file_path = "examples/lfo_plucks.mp3";
    let file = BufReader::new(File::open(file_path).unwrap());
    // Decode that sound file into a source
    let dry_audio = Decoder::new(file).unwrap();

    let reverb_audio_source = MVerbSource::new(dry_audio.convert_samples());

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(reverb_audio_source);

    sink.sleep_until_end();
}
