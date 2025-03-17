pub fn standardize(data: &mut [f32]) {
    let n = data.len() as f32;

    let mean = data.iter().sum::<f32>() / n;

    let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / n;
    let std_dev = variance.sqrt();

    for x in data.iter_mut() {
        *x = (*x - mean) / std_dev;
    }
}
