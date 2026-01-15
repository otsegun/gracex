mod primitives;
mod renderer;

use primitives::{Color, DrawCommand, Point, Stroke};
use renderer::{PngRenderer, Renderer};

fn main() {
    println!("Testing PngRenderer...");

    // Create some test draw commands
    let commands = vec![
        // Red circle
        DrawCommand::Circle {
            position: Point { x: 100.0, y: 100.0 },
            radius: 50.0,
            fill: Some(Color { r: 255, g: 0, b: 0, a: 255 }),
            stroke: Some(Stroke {
                color: Some(Color { r: 0, g: 0, b: 0, a: 255 }),
                width: 2.0,
            }),
        },
        // Blue rectangle
        DrawCommand::Rectangle {
            position: Point { x: 200.0, y: 50.0 },
            width: 100.0,
            height: 80.0,
            fill: Some(Color { r: 0, g: 0, b: 255, a: 255 }),
            stroke: None,
        },
        // Green triangle (polygon)
        DrawCommand::Polygon {
            points: vec![
                Point { x: 350.0, y: 150.0 },
                Point { x: 400.0, y: 50.0 },
                Point { x: 450.0, y: 150.0 },
            ],
            fill: Some(Color { r: 0, g: 255, b: 0, a: 200 }),
            stroke: Some(Stroke {
                color: Some(Color { r: 0, g: 128, b: 0, a: 255 }),
                width: 3.0,
            }),
        },
        // Black line
        DrawCommand::Line {
            start: Point { x: 50.0, y: 200.0 },
            end: Point { x: 450.0, y: 200.0 },
            stroke: Some(Stroke {
                color: Some(Color { r: 0, g: 0, b: 0, a: 255 }),
                width: 4.0,
            }),
        },
    ];

    // Create renderer and render
    let renderer = PngRenderer::new(500, 250, "test_output.png");

    match renderer.render(&commands) {
        Ok(_) => println!("✓ Successfully rendered to test_output.png"),
        Err(e) => eprintln!("✗ Error rendering: {}", e),
    }
}
