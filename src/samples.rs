use hound;

use std::ops::Deref;

#[derive(Clone)]
pub struct SampleSpec {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub duration_milis: f32,
}

#[derive(Clone)]
pub struct Samples {
    samples: Vec<f32>,
    pub spec: SampleSpec,
}

impl Samples {
    fn new(samples: Vec<f32>, spec: SampleSpec) -> Self {
        Samples { samples, spec }
    }
}

impl Deref for Samples {
    type Target = [f32];

    fn deref(&self) -> &Self::Target {
        &self.samples
    }
}

pub fn file_to_samples() -> Samples {
    // TODO: pass a path to this function
    let path = "audio/test.wav";

    // Open the WAV file
    let mut reader = hound::WavReader::open(path).expect("Failed to open WAV file");
    println!("Sample spec: {:?}", reader.spec());

    // Collect samples into a vector
    let samples: Vec<i16> = reader
        .samples::<i16>()
        .map(|s| s.expect("Failed to read sample"))
        .collect();

    // let clnd: Vec<f32> = samples.clone().iter().map(|f| *f as f32).collect();
    // plot(&clnd, "Samples.png").unwrap();

    println!("Loaded {} samples", samples.len());

    let spec = SampleSpec {
        sample_rate: reader.spec().sample_rate,
        channels: reader.spec().channels,
        bits_per_sample: reader.spec().bits_per_sample,
        duration_milis: samples.len() as f32 / reader.spec().sample_rate as f32 * 1000.0,
    };

    print!("Duration: {} s\n", spec.duration_milis / 1000.0);

    Samples::new(samples.iter().map(|&sample| sample as f32).collect(), spec)
}
