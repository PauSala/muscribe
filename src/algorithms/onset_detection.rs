use rustfft::{FftPlanner, num_complex::Complex};

pub struct StftBasedOnset<'a> {
    samples: &'a [f32],
    frame_size: usize,
    hop_size: usize,
}

impl<'a> StftBasedOnset<'a> {
    pub fn new(samples: &'a [f32], frame_size: usize, hop_size: usize) -> Self {
        StftBasedOnset {
            samples,
            frame_size,
            hop_size,
        }
    }

    /// Complex domain
    pub fn cd(&self) -> Vec<f32> {
        self.cd_inner(false)
    }

    /// Rectified Complex Domain
    pub fn rcd(&self) -> Vec<f32> {
        self.cd_inner(true)
    }

    fn cd_inner(&self, rcd: bool) -> Vec<f32> {
        let stft = self.stft();

        let num_frames = stft.len();
        let num_bins = stft[0].len();
        let mut cd = Vec::with_capacity(num_frames - 2);

        if num_frames < 3 {
            return cd;
        }

        for n in 2..num_frames {
            let mut sum = 0.0;
            for k in 0..num_bins {
                let x_n = stft[n][k]; // X(n, k)
                let x_n1 = stft[n - 1][k]; // X(n-1, k)
                let x_n2 = stft[n - 2][k]; // X(n-2, k)

                // Amplitude and phase of X(n-1, k)
                let amp_n1 = x_n1.norm();
                let phase_n1 = x_n1.arg();

                // Phase difference with normalization to [-π, π]
                let mut phase_diff = x_n1.arg() - x_n2.arg();
                phase_diff = (phase_diff + std::f32::consts::PI) % (2.0 * std::f32::consts::PI)
                    - std::f32::consts::PI;

                // Predicted target value X_T(n, k)
                let x_target = Complex::from_polar(amp_n1, phase_n1 + phase_diff);

                if x_n.norm() >= amp_n1 || !rcd {
                    sum += (x_n - x_target).norm();
                }
            }
            cd.push(sum);
        }
        cd
    }

    /// Short Term Fourier Transform
    fn stft(&self) -> Vec<Vec<Complex<f32>>> {
        let mut stft_result = Vec::new();

        let frame_size = self.frame_size;
        let hop_size = self.hop_size;

        let hw = self.hamming_window(frame_size);

        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(frame_size);

        for frame in self.samples.windows(frame_size).step_by(hop_size) {
            let mut f: Vec<Complex<f32>> = frame
                .iter()
                .enumerate()
                .map(|(j, &sample)| Complex::new(sample * hw[j], 0.0))
                .collect();
            fft.process(&mut f);
            stft_result.push(f);
        }

        stft_result
    }

    fn hamming_window(&self, size: usize) -> Vec<f32> {
        (0..size)
            .map(|i| {
                0.54 - 0.46 * (2.0 * std::f32::consts::PI * i as f32 / (size as f32 - 1.0)).cos()
            })
            .collect()
    }
}
