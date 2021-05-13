pub mod area;
pub mod horizontal_bar;
pub mod line;
pub mod scatter;
pub mod vertical_bar;

/// Orientation is used for views that use shapes that are configured by orientation.
#[derive(Copy, Clone, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// View contains data representation on a chart.
pub trait View {
    fn to_svg(&self) -> svg::node::element::Group;
}
