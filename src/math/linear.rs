const DEFAULT_NORMALIZE: f32 = 0.5;

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
