use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, IntoDrawingArea, Rectangle},
    style::{BLUE, Color, WHITE},
};
use transcriber::{charts::plot, cqt::CqtFeatures, samples::file_to_samples, stft, yin::Yin};

fn main() {
    let bpm = 60;
    println!("BPM: {}", bpm);

    let samples = file_to_samples();

    let cd = stft::compute_cd(&stft::stft(samples.to_vec(), 2048, 441));

    plot(&cd, "cd.png").unwrap();

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

fn print_chart(data: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("chart.png", (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_index = data.len() as f32;

    let mut chart = ChartBuilder::on(&root)
        .caption("Normalized Data Chart", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..max_index, 0f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(data.iter().enumerate().map(|(i, &value)| {
        let x = i as f32;
        let bar = Rectangle::new([(x, 0.0), (x + 0.8, value)], BLUE.filled());
        bar
    }))?;
    Ok(())
}

fn print_frequencies(data: &[(f32, bool)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("frequencies.png", (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_index = data.len() as f32;
    let max = data
        .iter()
        .filter(|(_, b)| b == &true)
        .map(|(f, _)| f)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Frequencies", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..max_index, 0f32..*max)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(data.iter().enumerate().map(|(i, &value)| {
        let x = i as f32;
        let y = if value.1 { value.0 } else { 0.0 };
        let bar = Rectangle::new([(x, 0.0), (x + 0.8, y)], BLUE.filled());
        bar
    }))?;
    Ok(())
}

pub fn frame_to_millis(frame: usize, sample_rate: usize) -> f32 {
    frame as f32 / sample_rate as f32 * 1000.0
}
