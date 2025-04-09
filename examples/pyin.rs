use std::{ops::Deref, path::Path};

use pyin::{Framing, PYINExecutor, PadMode};
use transcriber::{
    algorithms::shared::frame_to_seconds, charts::print_frequencies, notes::Note,
    samples::file_to_samples,
};

pub fn main() {
    let fmin = 40f64; // minimum frequency in Hz
    let fmax = 600f64; // maximum frequency in Hz
    let sr = 44100; // sampling rate of audio data in Hz
    let frame_length = 2048; // frame length in samples
    let (win_length, hop_length, resolution) = (Some(1024), Some(512), Some(0.9)); // None to use default values
    let mut pyin_exec = PYINExecutor::new(
        fmin,
        fmax,
        sr,
        frame_length,
        win_length,
        hop_length,
        resolution,
    );

    println!("PYIN Executor initialized.");

    let samples = file_to_samples(&Path::new("audio/test4.wav"));

    let wav: Vec<f64> = file_to_samples(&Path::new("audio/test4.wav"))
        .deref()
        .iter()
        .map(|f| *f as f64)
        .collect();

    let fill_unvoiced = f64::NAN;
    let framing = Framing::Center(PadMode::Constant(0.)); // Zero-padding is applied on both sides of the signal. (only if cetner is true)

    let (timestamp, f0, _voiced_flag, voiced_prob) = pyin_exec.pyin(&wav, fill_unvoiced, framing);

    for (i, freq) in f0.iter().enumerate() {
        if !freq.is_nan() {
            println!(
                "Note: {:?} freq: {} secs: {:?} secs {:?} Accuracy: {:?}",
                Note::from(*freq as f32),
                f0[i],
                timestamp[i],
                frame_to_seconds(samples.spec.duration_milis / 1000.0, f0.len(), i),
                voiced_prob[i]
            );
        }
    }

    print_frequencies(
        &f0.iter()
            .map(|n| match n.is_nan() {
                true => (0.0, false),
                false => (*n as f32, true),
            })
            .collect::<Vec<(f32, bool)>>(),
    )
    .unwrap();

    println!("Total frames: {:?}", f0.len());
}
