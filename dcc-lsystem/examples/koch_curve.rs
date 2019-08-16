use image::Rgb;

use dcc_lsystem::renderer::{ImageRendererOptions, Renderer};
use dcc_lsystem::turtle::{TurtleAction, TurtleLSystemBuilder};

fn main() {
    let mut builder = TurtleLSystemBuilder::new();

    builder
        .token("F", TurtleAction::Forward(30))
        .token("+", TurtleAction::Rotate(90))
        .token("-", TurtleAction::Rotate(-90))
        .axiom("F")
        .rule("F => F + F - F - F + F");

    let (mut system, renderer) = builder.finish();
    system.step_by(7);

    let options =
        ImageRendererOptions::new(10, 4.0, Rgb([255u8, 255u8, 255u8]), Rgb([0u8, 0u8, 100u8]));

    renderer
        .render(&system, &options)
        .save("koch_curve.png")
        .expect("Failed to save koch_curve.png");
}