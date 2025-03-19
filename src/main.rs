use std::path::Path;

use transcriber::{
    algorithms::{
        bpm_detection::bpm,
        onset_detection::StftBasedOnset,
        peak_picking::{peak_picking, standardize},
        yin::Yin,
    },
    charts::{plot, print_frequencies},
    samples::file_to_samples,
};

fn main() {
    let samples = file_to_samples(&Path::new("audio/test2.wav"));
    let hop_size = 441;
    let onset = StftBasedOnset::new(&samples, 2048, hop_size);
    let mut cd = onset.spectral_flux();

    let effective_odf_rate = samples.spec.sample_rate as f32 / hop_size as f32;
    println!("BPM: {}", bpm(&cd, effective_odf_rate));

    standardize(&mut cd);

    plot(&cd, "complex_domain").unwrap();
    plot(&peak_picking(&cd, 1, 3, 0.8, 0.8), "onsets").unwrap();

    let notes = Yin::from(samples.clone());

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
