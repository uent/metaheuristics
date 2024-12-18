use nannou::prelude::*;

struct Model {
  points: usize,
}

fn view(app: &App, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(BLACK);

    // Draw a blue ellipse with default size and position.
    draw.polygon().color(STEELBLUE).points();

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

pub fn search_optimal() {
  nannou::sketch(view).run();
}