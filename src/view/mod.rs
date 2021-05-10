pub mod horizontal_bar;
pub mod line;
pub mod vertical_bar;

/// Orientation is used for views that use shapes that are configured by orientation.
#[derive(Copy, Clone, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}
