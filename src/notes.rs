use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    pub freq: f32,
    pub octave: usize,
    pub name: NoteName,
}

impl Note {
    pub fn new(freq: f32, octave: usize, name: NoteName) -> Self {
        Self { freq, octave, name }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteName {
    A,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
}

pub fn all_notes() -> Vec<Note> {
    let mut notes = Vec::new();
    let mut base_freq = 16.35;
    for octave in 0..9 {
        for name in &[
            NoteName::C,
            NoteName::CSharp,
            NoteName::D,
            NoteName::DSharp,
            NoteName::E,
            NoteName::F,
            NoteName::FSharp,
            NoteName::G,
            NoteName::GSharp,
            NoteName::A,
            NoteName::ASharp,
            NoteName::B,
        ] {
            if octave == 0 && *name == NoteName::C {
                notes.push(Note::new(base_freq, octave, *name));
                continue;
            }
            let freq = base_freq * 2.0f32.powf(1.0 as f32 / 12.0);
            notes.push(Note::new(freq, octave, *name));

            base_freq = freq;
        }
    }
    notes
}

pub static ALL_NOTES: LazyLock<Vec<Note>> = LazyLock::new(|| all_notes());

impl From<f32> for Note {
    fn from(value: f32) -> Self {
        let notes = &ALL_NOTES;
        let mut start = 0;
        let mut end = notes.len();
        while end - start > 1 {
            let mid = (start + end) / 2;
            if notes[mid].freq < value {
                start = mid;
            } else {
                end = mid;
            }
        }
        let s = &notes[start];
        let e = &notes[end];
        if (s.freq - value).abs() < (e.freq - value).abs() {
            *s
        } else {
            *e
        }
    }
}
