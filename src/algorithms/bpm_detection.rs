use rustfft::FftPlanner;
use rustfft::num_complex::Complex;

pub fn compute_autocorrelation(odf: &Vec<f32>) -> Vec<f32> {
    let n = odf.len();
    let padded_len = 2 * n; // zero-pad to avoid wrap-around issues
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(padded_len);
    let ifft = planner.plan_fft_inverse(padded_len);

    // Create a buffer with zero-padding
    let mut buffer: Vec<Complex<f32>> = odf.iter().map(|&x| Complex { re: x, im: 0.0 }).collect();
    buffer.resize(padded_len, Complex { re: 0.0, im: 0.0 });

    // Compute FFT
    fft.process(&mut buffer);

    // Multiply by complex conjugate (compute power spectrum)
    for x in buffer.iter_mut() {
        // Here, x.norm_sqr() gives the power.
        *x = Complex {
            re: x.norm_sqr(),
            im: 0.0,
        };
    }

    // Inverse FFT to get autocorrelation
    ifft.process(&mut buffer);

    buffer
        .iter()
        .take(n)
        .map(|x| x.re / padded_len as f32)
        .collect()
}

pub fn find_dominant_period(autocorr: &Vec<f32>, sample_rate: f32) -> f32 {
    let mut max_lag = 0;
    let mut max_value = 0.0;

    for lag in 10..autocorr.len() {
        if autocorr[lag] > max_value {
            max_value = autocorr[lag];
            max_lag = lag;
        }
    }

    max_lag as f32 / sample_rate
}

pub fn bpm(odf: &Vec<f32>, sample_rate: f32) -> f32 {
    let autocorr = compute_autocorrelation(odf);
    let dominant_period = find_dominant_period(&autocorr, sample_rate);

    // Convert period to BPM
    60.0 / dominant_period
}
