pub use crate::scale::band::BandScale;
pub use crate::scale::linear::LinearScale;
pub use crate::scale::{Scale, ScaleKind};

pub mod color;
pub mod error;
pub mod scale;

pub(crate) mod math;
pub(crate) mod render;
