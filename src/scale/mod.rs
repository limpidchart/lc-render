use std::cmp::PartialEq;

pub mod band;

/// ScaleKind represents supported scales.
#[derive(Debug, PartialEq)]
pub enum ScaleKind {
    Band,
}

/// Scale represents an axis scale that is used in views and chart.
pub trait Scale<T> {
    /// Scale the provided domain value for a scale range.
    fn scale(&self, domain: &T) -> f32;

    /// Get the list of ticks that represent the scale on an axis.
    fn ticks(&self) -> &Vec<T>;

    /// Get the scale kind.
    fn kind(&self) -> ScaleKind;

    /// Get the scale bandwidth.
    fn bandwidth(&self) -> f32;

    /// Check if scale range is reversed.
    fn is_range_reversed(&self) -> bool;

    /// Get the offset for each tick.
    fn tick_offset(&self) -> f32;
}
