use crate::{notes::Note, samples::Samples};
use pitch_detection::detector::{PitchDetector, yin::YINDetector};
use std::ops::Deref;

#[derive(Debug)]
pub struct Yin(Vec<Option<Note>>);

impl From<Samples> for Yin {
    fn from(samples: Samples) -> Self {
        const SAMPLE_RATE: usize = 44100;
        const SIZE: usize = 1024 * 5;
        const PADDING: usize = SIZE / 2;
        const POWER_THRESHOLD: f64 = 1.0;
        const CLARITY_THRESHOLD: f64 = 0.8;

        let signal: Vec<f64> = samples.iter().map(|s| *s as f64).collect();
        let mut detector = YINDetector::new(SIZE, PADDING);

        let mut notes = Vec::new();

        for i in 0..signal.len() / SIZE {
            let samples = &signal[i * SIZE..(i + 1) * SIZE];
            match detector.get_pitch(&samples, SAMPLE_RATE, POWER_THRESHOLD, CLARITY_THRESHOLD) {
                Some(pitch) => {
                    notes.push(Some(Note::from(pitch.frequency as f32)));
                }
                None => {
                    notes.push(None);
                }
            }
        }
        Yin(notes)
    }
}

impl Deref for Yin {
    type Target = [Option<Note>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
