pub fn standardize(data: &mut [f32]) {
    let n = data.len() as f32;

    let mean = data.iter().sum::<f32>() / n;

    let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / n;
    let std_dev = variance.sqrt();

    for x in data.iter_mut() {
        *x = (*x - mean) / std_dev;
    }
}

pub fn frame_to_seconds(duration: f32, frames: usize, frame: usize) -> f32 {
    (duration / frames as f32) * frame as f32
}

pub fn frame_to_frames(frames_a: usize, frames_b: usize, frame: usize) -> usize {
    frames_b * frame / frames_a
}
