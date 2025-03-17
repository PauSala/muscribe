use std::ops::Deref;

use cqt_rs::{CQTParams, Cqt};
use ndarray::Array2;

use crate::samples::Samples;

static MIN_FREQ: f32 = 30.0;
static MAX_FREQ: f32 = 8000.0;
static BINS_PER_OCTAVE: usize = 96;

pub struct CqtFeatures(Array2<f32>);

impl From<Samples> for CqtFeatures {
    fn from(samples: Samples) -> Self {
        let min_freq = MIN_FREQ;
        let max_freq = MAX_FREQ;
        let bins_per_octave = BINS_PER_OCTAVE;
        let sample_rate = 44100;
        let window_length = 4096;

        let cqt_params = CQTParams::new(
            min_freq,        // Minimum frequency (Hz)
            max_freq,        // Maximum frequency (Hz)
            bins_per_octave, // Bins per octave
            sample_rate,     // Sampling rate
            window_length,   // Window length (FFT size)
        )
        .expect("Error creating CQTParams");

        let cqt = Cqt::new(cqt_params);
        let hop_size = 512;

        CqtFeatures(
            cqt.process(&samples, hop_size)
                .expect("Error computing CQT features"),
        )
    }
}

fn compute_log_weights(min_freq: f32, num_bins: usize) -> Vec<f32> {
    (0..num_bins)
        .map(|k| {
            let f_k = min_freq * (2.0f32.powf(k as f32 / num_bins as f32));
            (f_k / min_freq).log2()
        })
        .collect()
}

impl CqtFeatures {
    fn spectral_flux(&self) -> Vec<f32> {
        let n_samples = self.shape()[0];
        let n_bins = self.shape()[1];
        let mut flux = Vec::with_capacity(n_samples);

        // First frame has no previous frame for comparison
        flux.push(0.0);

        let weights = compute_log_weights(bin_to_freq(0), n_bins);

        let mut min_flux = f32::MAX;
        let mut max_flux = f32::MIN;

        for n in 1..n_samples {
            let current_spectrum = self.slice(ndarray::s![n, ..]);
            let previous_spectrum = self.slice(ndarray::s![n - 1, ..]);

            let mut frame_flux = 0.0;
            for (k, (current, previous)) in current_spectrum
                .iter()
                .zip(previous_spectrum.iter())
                .enumerate()
            {
                let diff = current - previous;
                // Half-wave rectification and log-based weighting
                if diff > 0.0 {
                    frame_flux += weights[k] * (diff * diff);
                }
            }

            min_flux = min_flux.min(frame_flux);
            max_flux = max_flux.max(frame_flux);

            flux.push(frame_flux);
        }

        // Normalize
        // let range = max_flux - min_flux;
        // if range > 0.0 {
        //     flux.iter_mut().for_each(|f| {
        //         *f = (*f - min_flux) / range;
        //     });
        // } else {
        //     flux.iter_mut().for_each(|f| *f = 0.0);
        // }

        flux
    }

    pub fn onsets(&self) -> Vec<bool> {
        let window_size = self.shape()[0];
        let threshold_factor = 1.0;

        let spectral_flux = self.spectral_flux();
        let mut onsets = vec![false; spectral_flux.len()];

        let half_window = window_size / 2;

        for i in 0..spectral_flux.len() {
            let start = if i < half_window { 0 } else { i - half_window };
            let end = if i + half_window >= spectral_flux.len() {
                spectral_flux.len()
            } else {
                i + half_window + 1
            };

            let window: Vec<&f32> = spectral_flux[start..end].iter().collect();

            // Moving average
            let window_len = window.len() as f32;
            let window_sum: f32 = window.into_iter().sum();
            let moving_average = window_sum / window_len;

            // Threshold
            let threshold = moving_average * threshold_factor;
            if spectral_flux[i] > threshold {
                onsets[i] = true;
            }
        }

        onsets
    }

    pub fn start(&self, onsets: &[bool], duration_millis: f32) -> f32 {
        let first_index = onsets
            .iter()
            .position(|&o| o)
            .unwrap_or_else(|| onsets.len());
        frame_to_millis(
            first_index,
            (onsets.len() as f32 / duration_millis) * 1000.0,
        )
    }

    pub fn frequencies(&self) -> Vec<(f32, bool)> {
        let mut frequencies = Vec::with_capacity(self.shape()[0]);
        let (num_frames, _) = self.dim();
        let onsets = self.onsets();

        for frame_idx in 0..num_frames {
            // Extract the current frame (row) from the ndarray
            let frame = self.row(frame_idx);

            // Find the bin with the maximum amplitude
            if let Some((max_bin, _)) = frame
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            {
                let freq = bin_to_freq(max_bin);
                // println!(
                //     "Frame {}: Max bin = {}, Frequency = {:.2} Hz, Amplitude = {:.4}",
                //     frame_idx, max_bin, freq, max_value
                // );
                frequencies.push((freq, onsets[frame_idx]));
            }
        }
        frequencies
    }
}

pub fn frame_to_millis(frame: usize, sample_rate: f32) -> f32 {
    frame as f32 / sample_rate * 1000.0
}

impl Deref for CqtFeatures {
    type Target = Array2<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[inline(always)]
pub fn bin_to_freq(bin: usize) -> f32 {
    MIN_FREQ * (2.0_f32).powf(bin as f32 / BINS_PER_OCTAVE as f32)
}
