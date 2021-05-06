const DEFAULT_NORMALIZE: f32 = 0.5_f32;

/// Normalize a value x in [a; b] and return the corresponding value from the [0; 1] range.
pub fn normalize(start: f32, end: f32, x: f32) -> f32 {
    if (end - start).abs() < f32::EPSILON {
        return DEFAULT_NORMALIZE;
    }

    (x - start) / range(start, end)
}

/// Interpolate a value x in [0; 1] and return the corresponding value from the [start; end] range.
pub fn interpolate(start: f32, end: f32, x: f32) -> f32 {
    range(start, end) * x + start
}

/// Calculate a range from the provided start and end values.
pub fn range(start: f32, end: f32) -> f32 {
    end - start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_basic() {
        let result = normalize(0_f32, 100_f32, 2_f32);
        assert!((result - 0.02_f32).abs() < f32::EPSILON);
    }

    #[test]
    fn interpolate_basic() {
        let result = interpolate(100_f32, 400_f32, 2_f32);
        assert!((result - 700_f32).abs() < f32::EPSILON);
    }

    #[test]
    fn range_basic() {
        let result = range(11_f32, 54_f32);
        assert!((result - 43_f32).abs() < f32::EPSILON);
    }
}
