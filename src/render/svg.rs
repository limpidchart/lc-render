pub const DEFAULT_POINT_SIZE: i32 = 5;

pub const DEFAULT_FONT_COLOR: &str = "#080808";
pub const DEFAULT_FONT_FAMILY: &str = "sans-serif";
pub const DEFAULT_DY: &str = ".35em";
pub const DEFAULT_STROKE_WIDTH: i32 = 1;
pub const DEFAULT_STROKE_COLOR: &str = "#bbbbbb";

pub const X_ATTR: &str = "x";
pub const X1_ATTR: &str = "x1";
pub const X2_ATTR: &str = "x2";
pub const Y_ATTR: &str = "y";
pub const Y1_ATTR: &str = "y1";
pub const Y2_ATTR: &str = "y2";

pub const CX_ATTR: &str = "cx";
pub const CY_ATTR: &str = "cy";
pub const R_ATTR: &str = "r";

pub const D_ATTR: &str = "d";
pub const DY_ATTR: &str = "dy";

pub const WIDTH_ATTR: &str = "width";
pub const HEIGHT_ATTR: &str = "height";

pub const STROKE_ATTR: &str = "stroke";
pub const STROKE_WIDTH_ATTR: &str = "stroke-width";

pub const FILL_ATTR: &str = "fill";
pub const TRANSFORM_ATTR: &str = "transform";

pub const TEXT_ANCHOR_ATTR: &str = "text-anchor";
pub const TEXT_ANCHOR_START: &str = "start";
pub const TEXT_ANCHOR_MIDDLE: &str = "middle";
pub const TEXT_ANCHOR_END: &str = "end";

pub const FONT_SIZE_ATTR: &str = "font-size";
pub const FONT_FAMILY_ATTR: &str = "font-family";

pub const SHAPE_RENDERING_ATTR: &str = "shape-rendering";
pub const SHAPE_RENDERING_CRISP_EDGES: &str = "crispEdges";

pub const CLASS_ATTR: &str = "class";
pub const CLASS_AREA: &str = "area";
pub const CLASS_BAR: &str = "bar";
pub const CLASS_CHART: &str = "chart";
pub const CLASS_VIEWS: &str = "views";
pub const CLASS_X_AXIS: &str = "x-axis";
pub const CLASS_Y_AXIS: &str = "y-axis";
pub const CLASS_TICK: &str = "tick";
pub const CLASS_TITLE: &str = "title";
pub const CLASS_POINT: &str = "point";
pub const CLASS_LINE: &str = "line";

pub const VIEW_BOX_ATTR: &str = "viewBox";

pub const START: f32 = 0_f32;

pub const FILL_NONE: &str = "none";

pub fn translate_x_y<T: std::fmt::Display>(x: T, y: T) -> String {
    format!("translate({},{})", x, y)
}

pub fn rotate_a_x_y<T: std::fmt::Display>(a: T, x: T, y: T) -> String {
    format!("rotate({},{},{})", a, x, y)
}

pub fn rotate_a<T: std::fmt::Display>(a: T) -> String {
    format!("rotate({})", a)
}

pub fn pair_x_y<T: std::fmt::Display>(x: T, y: T) -> String {
    format!("({},{})", x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate() {
        let translated = translate_x_y(1_f32, 2_f32);
        assert_eq!(translated, "translate(1,2)");
    }

    #[test]
    fn rotate() {
        let rotated_1 = rotate_a_x_y(1_f32, 2_f32, 3_f32);
        assert_eq!(rotated_1, "rotate(1,2,3)");

        let rotated_2 = rotate_a(45_f32);
        assert_eq!(rotated_2, "rotate(45)");
    }

    #[test]
    fn pair() {
        let paired = pair_x_y(12.1_f32, 21.1_f32);
        assert_eq!(paired, "(12.1,21.1)");
    }
}
