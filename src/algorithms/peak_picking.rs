pub fn standardize(data: &mut [f32]) {
    let n = data.len() as f32;

    let mean = data.iter().sum::<f32>() / n;

    let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / n;
    let std_dev = variance.sqrt();

    for x in data.iter_mut() {
        *x = (*x - mean) / std_dev;
    }
}

pub fn peak_picking(f: &Vec<f32>, w: usize, m: usize, delta: f32, alpha: f32) -> Vec<f32> {
    let n = f.len();
    let mut onsets = Vec::new();

    let mut g_alpha = vec![0.0; n];
    g_alpha[0] = f[0];

    // Adaptive threshold function
    for i in 1..n {
        g_alpha[i] = f[i].max(alpha * g_alpha[i - 1] + (1.0 - alpha) * f[i]);
    }

    for i in w..(n - w) {
        // Local maximum
        let is_local_max = (i - w..=i + w).all(|k| f[i] >= f[k]);

        // Local mean
        let local_mean_range_start = i.saturating_sub(m * w);
        let local_mean_range_end = (i + w).min(n - 1);
        let local_mean_sum: f32 = (local_mean_range_start..=local_mean_range_end)
            .map(|k| f[k])
            .sum();
        let local_mean =
            local_mean_sum / (local_mean_range_end - local_mean_range_start + 1) as f32;
        let is_above_local_mean = f[i] >= local_mean + delta;

        // Adaptive threshold
        let is_above_adaptive_threshold = f[i] >= g_alpha[i - 1];

        if is_local_max && is_above_local_mean && is_above_adaptive_threshold {
            onsets.push(1.0);
        } else {
            onsets.push(0.0);
        }
    }

    onsets
}
