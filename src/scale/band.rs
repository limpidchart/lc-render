use crate::math::linear::range;
use crate::{Scale, ScaleKind};
use itertools::Itertools;
use std::collections::HashMap;

const DEFAULT_PADDING: f32 = 0.1_f32;
const DEFAULT_ALIGN: f32 = 0.5_f32;
const DEFAULT_STEP: f32 = 1_f32;
const DEFAULT_BANDWIDTH: f32 = 1_f32;

/// BandScale represents axis scale with categories.
#[derive(Clone)]
pub struct BandScale {
    /// Scale categories.
    domain: Vec<String>,

    /// Start of the scale range.
    range_start: i32,

    /// End of the scale range.
    range_end: i32,

    /// Step value between categories.
    step: f32,

    /// Bandwidth of each category.
    bandwidth: f32,

    /// Inner padding of each category.
    padding_inner: f32,

    /// Outer padding of each category.
    padding_outer: f32,

    /// Scale align value.
    align: f32,

    /// Index of each category on a scale.
    index: HashMap<String, usize>,

    /// Offsets for each category on a scale.
    offsets: Vec<f32>,

    /// Does this scale needs an offset from the start and end of an axis.
    /// This is usually need for an area or line views.
    no_boundaries_offset: bool,
}

impl BandScale {
    /// Create a new BandScale.
    pub fn new(domain: Vec<String>, range_start: i32, range_end: i32) -> Self {
        let mut band = BandScale {
            domain: domain.iter().unique().map(|s| s.to_string()).collect(),
            range_start,
            range_end,
            step: DEFAULT_STEP,
            bandwidth: DEFAULT_BANDWIDTH,
            padding_inner: DEFAULT_PADDING,
            padding_outer: DEFAULT_PADDING,
            align: DEFAULT_ALIGN,
            index: HashMap::new(),
            offsets: Vec::new(),
            no_boundaries_offset: false,
        };
        band.rescale();
        band
    }

    /// Get scale range start.
    pub fn range_start(&self) -> i32 {
        self.range_start
    }

    /// Get scale range end.
    pub fn range_end(&self) -> i32 {
        self.range_end
    }

    /// Set scale inner padding.
    pub fn set_inner_padding(mut self, padding: f32) -> Self {
        self.padding_inner = padding;
        self.rescale();
        self
    }

    /// Set scale inner padding.
    pub fn set_outer_padding(mut self, padding: f32) -> Self {
        self.padding_outer = padding;
        self.rescale();
        self
    }

    /// Change scale no boundaries offset parameter.
    pub fn set_no_boundaries_offset(mut self, no_boundaries_offset: bool) -> Self {
        self.no_boundaries_offset = no_boundaries_offset;
        self
    }

    // Calculate the step, bandwidth, offsets and index for the scale.
    fn rescale(&mut self) {
        // We need an additional category after the axis in case no_boundaries_offset is set to true.
        let (domain_len, offsets_count) = if self.no_boundaries_offset {
            (self.domain.len() - 1, self.domain.len())
        } else {
            (self.domain.len(), self.domain.len() - 1)
        };

        let padded_domain_len = domain_len as f32 - self.padding_inner;
        let mut start = self.range_start as f32;
        let mut end = self.range_end as f32;
        let reverse = end < start;
        if reverse {
            std::mem::swap(&mut start, &mut end)
        }

        // Calculate scale step.
        let computed_step = padded_domain_len + self.padding_outer * 2_f32;
        let mut step_denominator = 1_f32;
        if computed_step > 1_f32 {
            step_denominator = computed_step;
        }
        let range = range(start, end);
        self.step = range / step_denominator;

        // Calculate start and bandwidth with the updated step value.
        start += (range - self.step * padded_domain_len) * self.align;
        self.bandwidth = self.step * (1_f32 - self.padding_inner);

        // Calculate offset value for each category.
        self.offsets.clear();
        for i in 0..=offsets_count {
            self.offsets.push(start + self.step * i as f32);
        }
        if reverse {
            self.offsets.reverse();
        }

        // Add categories to the index.
        self.index.clear();
        let mut processed_domains = Vec::new();
        for domain in self.domain.iter() {
            if !self.index.contains_key(domain) {
                self.index.insert(domain.clone(), processed_domains.len());
                processed_domains.push(domain.clone());
            }
        }

        // Update the domain with the new domain that does not contains duplicates.
        self.domain.clear();
        self.domain = processed_domains;
    }
}

impl Scale<String> for BandScale {
    fn scale(&self, domain: &String) -> f32 {
        return match self.index.get(domain) {
            Some(offset_idx) => self.offsets[*offset_idx],
            None => 0_f32,
        };
    }

    fn ticks(&self) -> Vec<String> {
        self.domain.clone()
    }

    fn kind(&self) -> ScaleKind {
        ScaleKind::Band
    }

    fn bandwidth(&self) -> f32 {
        self.bandwidth
    }

    fn is_range_reversed(&self) -> bool {
        self.range_start > self.range_end
    }

    fn tick_offset(&self) -> f32 {
        if self.no_boundaries_offset {
            return 0_f32;
        }

        // Views with boundaries offset will have a tick in the middle of a category.
        self.bandwidth() / 2_f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn band_scale_with_boundaries_offset() {
        let band_scale = BandScale::new(
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "C".to_string(),
                "D".to_string(),
                "A".to_string(),
            ],
            10,
            200,
        );

        assert_eq!(band_scale.range_start(), 10);
        assert_eq!(band_scale.range_end(), 200);
        assert_eq!(
            *band_scale.ticks(),
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string()
            ]
        );
        assert_eq!(band_scale.kind(), ScaleKind::Band);
        assert!((band_scale.bandwidth() - 41.707317_f32).abs() < f32::EPSILON);
        assert_eq!(band_scale.is_range_reversed(), false);
        assert!((band_scale.tick_offset() - 20.853659_f32).abs() < f32::EPSILON);
    }

    #[test]
    fn band_scale_without_boundaries_offset() {
        let band_scale = BandScale::new(
            vec![
                "a1".to_string(),
                "a1".to_string(),
                "a2".to_string(),
                "a3".to_string(),
                "a4".to_string(),
                "a2".to_string(),
            ],
            0,
            100,
        )
        .set_no_boundaries_offset(true)
        .set_inner_padding(0_f32)
        .set_outer_padding(0_f32);

        assert_eq!(band_scale.range_start(), 0);
        assert_eq!(band_scale.range_end(), 100);
        assert_eq!(
            *band_scale.ticks(),
            vec![
                "a1".to_string(),
                "a2".to_string(),
                "a3".to_string(),
                "a4".to_string()
            ]
        );
        assert_eq!(band_scale.kind(), ScaleKind::Band);
        assert!((band_scale.bandwidth() - 33.333332_f32).abs() < f32::EPSILON);
        assert_eq!(band_scale.is_range_reversed(), false);
        assert!((band_scale.tick_offset() - 0_f32).abs() < f32::EPSILON);
    }
}
