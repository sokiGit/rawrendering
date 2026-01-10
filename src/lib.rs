// Modules
pub mod canvas;
pub mod graphics_2d;
pub mod object_2d;
pub mod shape_2d;
pub mod utils;

// Re-export structs, etc.
pub use canvas::Canvas;
pub use graphics_2d::Graphics2D;
pub use object_2d::Object2D;
pub use shape_2d::{
    Shape2D,
    circle::Circle,
    path_trace::{
        PathTrace2D,
        line::Line2D,
        cubic_bezier_curve::CubicBezierCurve2D
    },
    rect::Rect,
    vector_path::VectorPath2D,
};
pub use utils::{color::Color, vec2::Vec2};
