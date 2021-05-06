use crate::math::linear::{interpolate, normalize, range};
use crate::{Scale, ScaleKind};

const DEFAULT_TICK_COUNT: usize = 11;

/// LinearScale represents axis scale with numerical values.
#[derive(Clone)]
pub struct LinearScale {
    /// Start of the scale domain.
    domain_start: f32,

    /// End of the scale domain.
    domain_end: f32,

    /// Start of the scale range.
    range_start: i32,

    /// End of the scale range.
    range_end: i32,

    /// Count of ticks on scale axis.
    tick_count: usize,
}

impl LinearScale {
    /// Create a new LinearScale.
    pub fn new(domain_start: f32, domain_end: f32, range_start: i32, range_end: i32) -> Self {
        Self {
            domain_start,
            domain_end,
            range_start,
            range_end,
            tick_count: DEFAULT_TICK_COUNT,
        }
    }

    /// Get scale range start.
    pub fn range_start(&self) -> i32 {
        self.range_start
    }

    /// Get scale range end.
    pub fn range_end(&self) -> i32 {
        self.range_end
    }

    // Compute the step for each tick.
    fn compute_tick_step(&self, start: f32, end: f32) -> f32 {
        let mut step_denominator = 0_f32;
        if self.tick_count as f32 > step_denominator {
            step_denominator = self.tick_count as f32;
        }

        let step = range(start, end) / step_denominator;
        let power = (step.ln() / 10_f32.ln()).trunc() as i32;
        let error = step / 10_f32.powi(power);

        let mut dynamic = 1;
        if error >= 50_f32.sqrt() {
            dynamic = 10;
        } else if error >= 10_f32.sqrt() {
            dynamic = 5;
        } else if error >= 2_f32.sqrt() {
            dynamic = 2;
        };

        if power < 0 {
            return -(10_f32.powi(-power)) / dynamic as f32;
        }

        dynamic as f32 * 10_f32.powi(power)
    }

    // Calculate vector of ticks for positive step.
    fn ticks_positive_step(&self, step: f32) -> Vec<f32> {
        let mut res = Vec::new();

        let start = (self.domain_start / step).ceil();
        let end = (self.domain_end / step).floor();
        let ticks_count = (range(start, end) + 1_f32).ceil() as i32;
        for i in 0..ticks_count {
            res.push((start + i as f32) * step);
        }

        res
    }

    // Calculate vector of ticks for negative step.
    fn ticks_negative_step(&self, step: f32) -> Vec<f32> {
        let mut res = Vec::new();

        let start = (self.domain_start as f32 * step).floor();
        let end = (self.domain_end as f32 * step).ceil();
        let ticks_count = (range(end, start) + 1_f32).ceil() as i32;
        for i in 0..ticks_count {
            res.push((start - i as f32) / step);
        }

        res
    }
}

impl Scale<f32> for LinearScale {
    fn scale(&self, domain: &f32) -> f32 {
        let normalized = normalize(self.domain_start, self.domain_end, *domain);
        interpolate(self.range_start as f32, self.range_end as f32, normalized)
    }

    fn ticks(&self) -> Vec<f32> {
        if (self.domain_end - self.domain_start).abs() < f32::EPSILON && self.tick_count > 0 {
            return vec![self.domain_start as f32];
        }

        let step = self.compute_tick_step(self.domain_start, self.domain_end);
        if step > 0_f32 {
            return self.ticks_positive_step(step);
        }

        self.ticks_negative_step(step)
    }

    fn kind(&self) -> ScaleKind {
        ScaleKind::Linear
    }

    fn bandwidth(&self) -> f32 {
        0_f32
    }

    fn is_range_reversed(&self) -> bool {
        self.range_start > self.range_end
    }

    fn tick_offset(&self) -> f32 {
        0_f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_scale_basic() {
        let linear_scale = LinearScale::new(0_f32, 200_f32, 540, 0);

        assert_eq!(linear_scale.range_start(), 540);
        assert_eq!(linear_scale.range_end(), 0);
        assert_eq!(
            *linear_scale.ticks(),
            vec![
                0_f32, 20_f32, 40_f32, 60_f32, 80_f32, 100_f32, 120_f32, 140_f32, 160_f32, 180_f32,
                200_f32
            ]
        );
        assert!((linear_scale.scale(&24_f32) - 475.2_f32).abs() < f32::EPSILON);
        assert_eq!(linear_scale.kind(), ScaleKind::Linear);
        assert!((linear_scale.bandwidth() - 0_f32).abs() < f32::EPSILON);
        assert_eq!(linear_scale.is_range_reversed(), true);
        assert!((linear_scale.tick_offset() - 0_f32).abs() < f32::EPSILON);
    }
}
