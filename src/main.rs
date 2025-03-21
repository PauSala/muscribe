use std::{ops::Deref, path::Path};

use pyin::{Framing, PYINExecutor, PadMode};
use transcriber::{
    algorithms::{
        bpm_detection::bpm,
        onset_detection::StftBasedOnset,
        peak_picking::{peak_picking, peak_picking_to_notes, peak_picking_to_seconds},
        shared::standardize,
    },
    charts::{plot, print_frequencies},
    notes::Note,
    samples::file_to_samples,
};

fn main() {
    let samples = file_to_samples(&Path::new("audio/test5.wav"));
    plot(&samples, "samples").unwrap();
    let hop_size = 441;
    let onset = StftBasedOnset::new(&samples, 2048, hop_size);
    let mut cd = onset.rcd();

    let odf_rate = samples.spec.sample_rate as f32 / hop_size as f32;
    let bpm = bpm(&cd, odf_rate);
    println!("BPM: {}", bpm);

    standardize(&mut cd);
    plot(&cd, "odf").unwrap();

    let onsets = peak_picking(&mut cd, 3, 3, 0.5, 0.4);

    plot(
        &(onsets
            .iter()
            .map(|f| if *f { 0.1 } else { 0.0 })
            .collect::<Vec<f32>>()),
        "onsets",
    )
    .unwrap();

    let onset_seconds =
        peak_picking_to_seconds(&onsets, samples.spec.duration_milis as f32 / 1000.0);

    println!("Onsets: {:?}", onset_seconds);

    let fmin = 40f64; // minimum frequency in Hz
    let fmax = 600f64; // maximum frequency in Hz
    let sr = 44100; // sampling rate of audio data in Hz
    let frame_length = 4096; // frame length in samples
    let (win_length, hop_length, resolution) = (None, None, Some(0.05)); // None to use default values
    let mut pyin_exec = PYINExecutor::new(
        fmin,
        fmax,
        sr,
        frame_length,
        win_length,
        hop_length,
        resolution,
    );

    let wav: Vec<f64> = samples.deref().iter().map(|f| *f as f64).collect();
    let fill_unvoiced = f64::NAN;
    let framing = Framing::Center(PadMode::Constant(0.)); // Zero-padding is applied on both sides of the signal. (only if cetner is true)

    let (_timestamp, f0, _voiced_flag, voiced_prob) = pyin_exec.pyin(&wav, fill_unvoiced, framing);
    let notes = &f0
        .iter()
        .enumerate()
        .map(|(i, f)| {
            if f.is_nan() || voiced_prob[i] < 0.7 {
                None
            } else {
                Some(Note::from(*f as f32))
            }
        })
        .collect::<Vec<Option<Note>>>();

    // for note in notes {
    //     println!("Note: {:?}", note);
    // }

    let all_notes = peak_picking_to_notes(&onsets, &notes);
    for note in all_notes.iter() {
        println!("Note: {:?}", note);
    }

    print_frequencies(
        &notes
            .iter()
            .map(|n| match n {
                Some(n) => (n.freq, true),
                None => (0.0, false),
            })
            .collect::<Vec<(f32, bool)>>(),
    )
    .unwrap();
}
