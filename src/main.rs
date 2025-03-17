use transcriber::{
    algorithms::{onset_detection::StftBasedOnset, peak_picking::standardize},
    charts::plot,
    samples::file_to_samples,
};

fn main() {
    let bpm = 60;
    println!("BPM: {}", bpm);

    let samples = file_to_samples();

    let onset = StftBasedOnset::new(&samples, 2048, 441);
    let mut cd = onset.rcd();
    standardize(&mut cd);

    plot(&cd, "complex_domain").unwrap();

    // let notes = Yin::from(samples.clone());
    // println!("Yan samples {:?}", notes.len());

    // for note in notes.iter() {
    //     match note {
    //         Some(n) => println!("{:?} Hz", n.name),
    //         None => (),
    //     }
    // }

    // print_frequencies(
    //     &notes
    //         .iter()
    //         .map(|n| match n {
    //             Some(n) => (n.freq, true),
    //             None => (0.0, false),
    //         })
    //         .collect::<Vec<(f32, bool)>>(),
    // )
    // .unwrap();

    // let cqt_features: CqtFeatures = samples.clone().into();

    // let (num_frames, num_bins) = cqt_features.dim();
    // println!(
    //     "Shape: {} frames x {} bins Yan scale {}",
    //     num_frames,
    //     num_bins,
    //     num_frames as f32 / notes.len() as f32
    // );

    // let start = cqt_features.start(&cqt_features.onsets(), samples.spec.duration_milis);
    // println!("Start: {:.2} s", start * 0.001);

    // print_chart(
    //     &cqt_features
    //         .onsets()
    //         .iter()
    //         .map(|o| if *o { 1.0 } else { 0.0 })
    //         .collect::<Vec<f32>>(),
    // )
    // .unwrap();
}
