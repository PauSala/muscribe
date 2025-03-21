use crate::notes::Note;

use super::shared::{frame_to_frames, frame_to_seconds, standardize};

pub fn peak_picking(f: &mut Vec<f32>, w: usize, m: usize, delta: f32, alpha: f32) -> Vec<bool> {
    standardize(f);
    let n = f.len();
    let mut onsets = vec![false; n];

    let mut g_alpha = vec![0.0; n];
    g_alpha[0] = f[0];

    // Adaptive threshold
    for i in 1..n {
        g_alpha[i] = f[i].max(alpha * g_alpha[i - 1] + (1.0 - alpha) * f[i]);
    }

    for i in w..n.saturating_sub(w) {
        // 1. Local Maximum (Strict Comparison)
        let is_local_max = f[i] > f[i - 1] && f[i] > f[i + 1];

        // 2. Local Mean (Symmetric Range)
        let local_mean_range_start = i.saturating_sub(w * m);
        let local_mean_range_end = (i + w * m).min(n - 1);
        let local_mean: f32 = f[local_mean_range_start..=local_mean_range_end]
            .iter()
            .sum::<f32>()
            / (local_mean_range_end - local_mean_range_start + 1) as f32;

        let is_above_local_mean = f[i] >= local_mean + delta;

        // 3. Adaptive Threshold (Use Current Value)
        let is_above_adaptive_threshold = f[i] >= g_alpha[i];

        if is_local_max && is_above_local_mean && is_above_adaptive_threshold {
            onsets[i] = true;
        }
    }

    onsets
}

pub fn peak_picking_to_seconds(pp: &[bool], duration: f32) -> Vec<f32> {
    let mut frame = 0;
    let mut res = Vec::new();

    for f in pp {
        if *f {
            res.push(frame_to_seconds(duration, pp.len(), frame));
        }
        frame += 1;
    }
    res
}

pub fn peak_picking_to_notes(pp: &[bool], notes: &[Option<Note>]) -> Vec<Option<Note>> {
    let mut res = Vec::new();

    for (i, f) in pp.iter().enumerate() {
        if *f {
            let frame = frame_to_frames(pp.len(), notes.len(), i);
            let guess_window = 3;
            let mut note = None;
            for i in
                (frame.saturating_sub(guess_window - 1)..frame.saturating_add(guess_window)).rev()
            {
                if let Some(n) = notes[i] {
                    note = Some(n);
                    break;
                }
            }
            if let Some(n) = note {
                res.push(Some(n));
            }
        }
    }
    res
}
