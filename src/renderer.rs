use crate::primitives::{Color, DrawCommand, Point};
use tiny_skia::{Color as SkiaColor, Paint, PathBuilder, Pixmap, Stroke as SkiaStroke, Transform};

pub trait Renderer {
    // should require the the method render
    fn render(&self, commands: &[DrawCommand]) -> Result<(), std::io::Error>;
}

pub struct PngRenderer {
    width: u32, // in px
    height: u32,
    file_path: String,
}

impl PngRenderer {
    pub fn new(width: u32, height: u32, file_path: &str) -> Self {
        PngRenderer {
            width,
            height,
            file_path: file_path.to_string(),
        }
    }

    /// Helper: Convert our Color to tiny-skia Color
    fn to_skia_color(color: &Color) -> SkiaColor {
        SkiaColor::from_rgba8(color.r, color.g, color.b, color.a)
    }

    /// Helper: Create a Paint from our Color
    fn create_paint(color: &Color) -> Paint<'static> {
        let mut paint = Paint::default();
        paint.set_color(Self::to_skia_color(color));
        paint.anti_alias = true;
        paint
    }

    /// Helper: Create a tiny-skia Stroke from our Stroke
    fn create_stroke(stroke: &crate::primitives::Stroke) -> Option<SkiaStroke> {
        let mut skia_stroke = SkiaStroke::default();
        skia_stroke.width = stroke.width as f32;
        Some(skia_stroke)
    }

    fn draw_circle(
        &self,
        pixmap: &mut Pixmap,
        position: &Point,
        radius: f64,
        fill: Option<&Color>,
        stroke: Option<&crate::primitives::Stroke>,
    ) -> Result<(), std::io::Error> {
        let mut path = PathBuilder::new();

        // Build circle path using bezier curves (tiny-skia doesn't have a circle primitive)
        // We approximate a circle with 4 cubic bezier curves
        let r = radius as f32;
        let cx = position.x as f32;
        let cy = position.y as f32;
        let k = 0.5522847498; // Magic constant for circle approximation
        let kr = k * r;

        path.move_to(cx - r, cy);
        path.cubic_to(cx - r, cy - kr, cx - kr, cy - r, cx, cy - r);
        path.cubic_to(cx + kr, cy - r, cx + r, cy - kr, cx + r, cy);
        path.cubic_to(cx + r, cy + kr, cx + kr, cy + r, cx, cy + r);
        path.cubic_to(cx - kr, cy + r, cx - r, cy + kr, cx - r, cy);
        path.close();

        let path = path.finish().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to build circle path")
        })?;

        // Fill if specified
        if let Some(fill_color) = fill {
            let paint = Self::create_paint(fill_color);
            pixmap.fill_path(
                &path,
                &paint,
                tiny_skia::FillRule::Winding,
                Transform::identity(),
                None,
            );
        }

        // Stroke if specified
        if let Some(stroke_spec) = stroke {
            if let Some(stroke_color) = &stroke_spec.color {
                let paint = Self::create_paint(stroke_color);
                let skia_stroke = Self::create_stroke(stroke_spec).ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to create stroke")
                })?;
                pixmap.stroke_path(&path, &paint, &skia_stroke, Transform::identity(), None);
            }
        }

        Ok(())
    }

    fn draw_line(
        &self,
        pixmap: &mut Pixmap,
        start: &Point,
        end: &Point,
        stroke: Option<&crate::primitives::Stroke>,
    ) -> Result<(), std::io::Error> {
        let mut path = PathBuilder::new();
        path.move_to(start.x as f32, start.y as f32);
        path.line_to(end.x as f32, end.y as f32);

        let path = path.finish().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to build line path")
        })?;

        if let Some(stroke_spec) = stroke {
            if let Some(stroke_color) = &stroke_spec.color {
                let paint = Self::create_paint(stroke_color);
                let skia_stroke = Self::create_stroke(stroke_spec).ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to create stroke")
                })?;
                pixmap.stroke_path(&path, &paint, &skia_stroke, Transform::identity(), None);
            }
        }

        Ok(())
    }

    fn draw_rectangle(
        &self,
        pixmap: &mut Pixmap,
        position: &Point,
        width: f64,
        height: f64,
        fill: Option<&Color>,
        stroke: Option<&crate::primitives::Stroke>,
    ) -> Result<(), std::io::Error> {
        let mut path = PathBuilder::new();
        let x = position.x as f32;
        let y = position.y as f32;
        let w = width as f32;
        let h = height as f32;

        path.move_to(x, y);
        path.line_to(x + w, y);
        path.line_to(x + w, y + h);
        path.line_to(x, y + h);
        path.close();

        let path = path.finish().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to build rectangle path")
        })?;

        // Fill if specified
        if let Some(fill_color) = fill {
            let paint = Self::create_paint(fill_color);
            pixmap.fill_path(
                &path,
                &paint,
                tiny_skia::FillRule::Winding,
                Transform::identity(),
                None,
            );
        }

        // Stroke if specified
        if let Some(stroke_spec) = stroke {
            if let Some(stroke_color) = &stroke_spec.color {
                let paint = Self::create_paint(stroke_color);
                let skia_stroke = Self::create_stroke(stroke_spec).ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to create stroke")
                })?;
                pixmap.stroke_path(&path, &paint, &skia_stroke, Transform::identity(), None);
            }
        }

        Ok(())
    }

    fn draw_polygon(
        &self,
        pixmap: &mut Pixmap,
        points: &[Point],
        fill: Option<&Color>,
        stroke: Option<&crate::primitives::Stroke>,
    ) -> Result<(), std::io::Error> {
        if points.is_empty() {
            return Ok(());
        }

        let mut path = PathBuilder::new();
        path.move_to(points[0].x as f32, points[0].y as f32);

        for point in &points[1..] {
            path.line_to(point.x as f32, point.y as f32);
        }
        path.close();

        let path = path.finish().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to build polygon path")
        })?;

        // Fill if specified
        if let Some(fill_color) = fill {
            let paint = Self::create_paint(fill_color);
            pixmap.fill_path(
                &path,
                &paint,
                tiny_skia::FillRule::Winding,
                Transform::identity(),
                None,
            );
        }

        // Stroke if specified
        if let Some(stroke_spec) = stroke {
            if let Some(stroke_color) = &stroke_spec.color {
                let paint = Self::create_paint(stroke_color);
                let skia_stroke = Self::create_stroke(stroke_spec).ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to create stroke")
                })?;
                pixmap.stroke_path(&path, &paint, &skia_stroke, Transform::identity(), None);
            }
        }

        Ok(())
    }
}

impl Renderer for PngRenderer {
    fn render(&self, commands: &[DrawCommand]) -> Result<(), std::io::Error> {
        // Create a pixmap (the canvas)
        let mut pixmap = Pixmap::new(self.width, self.height).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to create pixmap")
        })?;

        // Fill with white background
        pixmap.fill(SkiaColor::WHITE);

        // Process each draw command
        for command in commands {
            match command {
                DrawCommand::Circle {
                    position,
                    radius,
                    fill,
                    stroke,
                } => {
                    self.draw_circle(
                        &mut pixmap,
                        position,
                        *radius,
                        fill.as_ref(),
                        stroke.as_ref(),
                    )?;
                }
                DrawCommand::Line { start, end, stroke } => {
                    self.draw_line(&mut pixmap, start, end, stroke.as_ref())?;
                }
                DrawCommand::Rectangle {
                    position,
                    width,
                    height,
                    fill,
                    stroke,
                } => {
                    self.draw_rectangle(
                        &mut pixmap,
                        position,
                        *width,
                        *height,
                        fill.as_ref(),
                        stroke.as_ref(),
                    )?;
                }
                DrawCommand::Polygon {
                    points,
                    fill,
                    stroke,
                } => {
                    self.draw_polygon(&mut pixmap, points, fill.as_ref(), stroke.as_ref())?;
                }
                DrawCommand::Text {
                    position,
                    content,
                    font_size: _,
                    color: _,
                } => {
                    // Text rendering is complex, we'll skip for MWE
                    // In a real implementation, you'd use a text shaping library
                    println!(
                        "Text rendering not yet implemented: {} at ({}, {})",
                        content, position.x, position.y
                    );
                }
            }
        }

        // Save to PNG
        pixmap
            .save_png(&self.file_path)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }
}
