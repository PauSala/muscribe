use rustfft::{FftDirection, FftPlanner, num_complex::Complex};

pub fn stft(input_signal: Vec<f32>, frame_size: usize, hop_size: usize) -> Vec<Vec<Complex<f32>>> {
    let mut stft_result = Vec::new();

    let hw = hamming_window(frame_size);

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft(frame_size, FftDirection::Forward);

    let num_frames = (input_signal.len() as f32 / hop_size as f32).ceil() as usize;

    for i in 0..num_frames {
        let start = i * hop_size;
        let end = (start + frame_size).min(input_signal.len());

        if end - start < frame_size {
            break;
        }

        let mut frame: Vec<Complex<f32>> = input_signal[start..end]
            .iter()
            .enumerate()
            .map(|(j, &sample)| Complex::new(sample * hw[j], 0.0))
            .collect();

        fft.process(&mut frame);
        stft_result.push(frame);
    }

    stft_result
}

pub fn compute_cd(stft: &Vec<Vec<Complex<f32>>>) -> Vec<f32> {
    let num_frames = stft.len();
    let num_bins = stft[0].len();
    let mut cd = Vec::with_capacity(num_frames);

    // Skip the first two frames
    for n in 2..num_frames {
        let mut sum = 0.0;
        for k in 0..num_bins {
            let x_n1 = stft[n - 1][k]; // X(n-1, k)
            let x_n2 = stft[n - 2][k]; // X(n-2, k)

            // Amplitude and phase of X(n-1, k)
            let amp_n1 = x_n1.norm();
            let phase_n1 = x_n1.arg();

            // Phase difference
            let phase_diff = x_n1.arg() - x_n2.arg();

            // Target value X_T(n, k)
            let x_target = Complex::from_polar(amp_n1, phase_n1 + phase_diff);

            // Absolute difference
            sum += (stft[n][k] - x_target).norm();
        }
        cd.push(sum);
    }
    cd
}

// Example implementation of a Hamming window
fn hamming_window(size: usize) -> Vec<f32> {
    (0..size)
        .map(|i| 0.54 - 0.46 * (2.0 * std::f32::consts::PI * i as f32 / (size as f32 - 1.0)).cos())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_window() {
        let window = hamming_window(5);
        assert_eq!(window, vec![0.08, 0.54, 1.0, 0.54, 0.08]);
    }

    #[test]
    fn test_stft() {
        let input_signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let frame_size = 4;
        let hop_size = 2;

        let stft_result = stft(input_signal, frame_size, hop_size);

        assert_eq!(stft_result.len(), 3);
        assert_eq!(stft_result[0].len(), 4);
        println!("{:?}", stft_result[0]);
    }

    #[test]
    fn test_cd() {
        let input_signal = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
        ];
        let frame_size = 4;
        let hop_size = 2;

        let stft_result = stft(input_signal, frame_size, hop_size);
        println!("{:?}", stft_result);

        let cd = compute_cd(&stft_result);
        println!("{:?}", cd);
    }
}
