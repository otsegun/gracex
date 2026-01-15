pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Stroke {
    pub color: Option<Color>,
    pub width: f64,
}

pub enum DrawCommand {
    Circle {
        position: Point,
        radius: f64,
        fill: Option<Color>,
        stroke: Option<Stroke>,
    },

    Line {
        start: Point,
        end: Point,
        stroke: Option<Stroke>,
    },

    Rectangle {
        position: Point, // topleft position
        width: f64,
        height: f64,
        fill: Option<Color>,
        stroke: Option<Stroke>,
    },

    Polygon {
        points: Vec<Point>,
        fill: Option<Color>,
        stroke: Option<Stroke>,
    },

    #[allow(dead_code)]  // Not yet implemented for MWE
    Text {
        position: Point,
        content: String,
        font_size: f32,
        color: Option<Color>,
    },
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            // Create default stroke
            color: Some(Color::default()),
            width: 2.0,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}
