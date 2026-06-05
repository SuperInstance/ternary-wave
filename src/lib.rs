#![forbid(unsafe_code)]

/// Generate a square wave with values in {-1, 0, 1}.
pub fn square(freq: usize, ticks: usize) -> Vec<i8> {
    if freq == 0 || ticks == 0 {
        return vec![0; ticks];
    }
    let period = ticks / freq.max(1);
    if period == 0 {
        return vec![0; ticks];
    }
    let half = period / 2;
    (0..ticks)
        .map(|i| {
            let phase = i % period;
            if phase < half { 1 } else { -1 }
        })
        .collect()
}

/// Generate a sawtooth wave with ternary values.
pub fn saw(freq: usize, ticks: usize) -> Vec<i8> {
    if freq == 0 || ticks == 0 {
        return vec![0; ticks];
    }
    let period = ticks / freq.max(1);
    if period == 0 {
        return vec![0; ticks];
    }
    (0..ticks)
        .map(|i| {
            let phase = i % period;
            if phase == 0 { 0 } else if phase <= period / 3 { 1 } else if phase <= 2 * period / 3 { 0 } else { -1 }
        })
        .collect()
}

/// Generate a triangle wave with ternary values.
pub fn triangle(freq: usize, ticks: usize) -> Vec<i8> {
    if freq == 0 || ticks == 0 {
        return vec![0; ticks];
    }
    let period = ticks / freq.max(1);
    if period == 0 {
        return vec![0; ticks];
    }
    let quarter = period / 4;
    (0..ticks)
        .map(|i| {
            let phase = i % period;
            if quarter == 0 {
                0
            } else if phase < quarter {
                1
            } else if phase < quarter * 3 {
                -1
            } else {
                1
            }
        })
        .collect()
}

/// Generate a pulse wave with configurable duty cycle.
pub fn pulse(freq: usize, ticks: usize, duty: usize) -> Vec<i8> {
    if freq == 0 || ticks == 0 {
        return vec![0; ticks];
    }
    let period = ticks / freq.max(1);
    if period == 0 {
        return vec![0; ticks];
    }
    let on = (period * duty.min(100)) / 100;
    (0..ticks)
        .map(|i| if i % period < on { 1 } else { -1 })
        .collect()
}

/// Simple pseudo-random ternary noise from seed.
pub fn noise(seed: u64, ticks: usize) -> Vec<i8> {
    let mut state = seed.wrapping_add(0x9E3779B97F4A7C15);
    (0..ticks)
        .map(|_| {
            state = state.wrapping_mul(0x517CC1B727220A95);
            state ^= state >> 33;
            state = state.wrapping_mul(0x6C62272E07BB0142);
            state ^= state >> 29;
            let v = (state & 3) as i8;
            match v {
                0 | 1 => 1,
                2 => -1,
                _ => 0,
            }
        })
        .collect()
}

/// Loop through an arbitrary wavetable for `ticks` samples.
pub fn wavetable(positions: &[i8], ticks: usize) -> Vec<i8> {
    if positions.is_empty() || ticks == 0 {
        return vec![0; ticks];
    }
    (0..ticks)
        .map(|i| positions[i % positions.len()])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_basic() {
        let w = square(2, 10);
        assert_eq!(w.len(), 10);
        assert!(w.iter().all(|&v| v == -1 || v == 0 || v == 1));
    }

    #[test]
    fn square_zero_freq() {
        let w = square(0, 5);
        assert_eq!(w, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn square_zero_ticks() {
        let w = square(4, 0);
        assert!(w.is_empty());
    }

    #[test]
    fn saw_basic() {
        let w = saw(2, 12);
        assert_eq!(w.len(), 12);
        assert!(w.iter().all(|&v| (-1..=1).contains(&v)));
    }

    #[test]
    fn triangle_basic() {
        let w = triangle(2, 16);
        assert_eq!(w.len(), 16);
        assert!(w.iter().all(|&v| (-1..=1).contains(&v)));
    }

    #[test]
    fn pulse_50_duty() {
        let w = pulse(1, 10, 50);
        let ones = w.iter().filter(|&&v| v == 1).count();
        let negs = w.iter().filter(|&&v| v == -1).count();
        assert_eq!(ones, 5);
        assert_eq!(negs, 5);
    }

    #[test]
    fn pulse_25_duty() {
        let w = pulse(1, 8, 25);
        assert_eq!(w, vec![1, 1, -1, -1, -1, -1, -1, -1]);
    }

    #[test]
    fn noise_deterministic() {
        let a = noise(42, 10);
        let b = noise(42, 10);
        assert_eq!(a, b);
    }

    #[test]
    fn noise_different_seeds() {
        let a = noise(42, 100);
        let b = noise(99, 100);
        assert_ne!(a, b);
    }

    #[test]
    fn noise_ternary_values() {
        let w = noise(123, 1000);
        assert!(w.iter().all(|&v| (-1..=1).contains(&v)));
    }

    #[test]
    fn wavetable_basic() {
        let w = wavetable(&[1, 0, -1], 9);
        assert_eq!(w, vec![1, 0, -1, 1, 0, -1, 1, 0, -1]);
    }

    #[test]
    fn wavetable_empty() {
        let w = wavetable(&[], 5);
        assert_eq!(w, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn wavetable_zero_ticks() {
        let w = wavetable(&[1, -1], 0);
        assert!(w.is_empty());
    }

    #[test]
    fn wavetable_single() {
        let w = wavetable(&[1], 5);
        assert_eq!(w, vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn saw_zero_freq() {
        assert_eq!(saw(0, 4), vec![0, 0, 0, 0]);
    }

    #[test]
    fn triangle_zero_ticks() {
        assert!(triangle(3, 0).is_empty());
    }
}
