use image::Rgb;

use dcc_lsystem::renderer::ImageRendererOptionsBuilder;
use dcc_lsystem::renderer::Renderer;
use dcc_lsystem::turtle::{TurtleAction, TurtleLSystemBuilder};

fn main() {
    let mut builder = TurtleLSystemBuilder::new();

    builder
        .token("X", TurtleAction::Nothing)
        .token("Y", TurtleAction::Nothing)
        .token("F", TurtleAction::Forward(30))
        .token("+", TurtleAction::Rotate(-90))
        .token("-", TurtleAction::Rotate(90))
        .axiom("F X")
        .rule("X => X + Y F +")
        .rule("Y => - F X - Y");

    let (mut system, renderer) = builder.finish();
    system.step_by(15);

    let options = ImageRendererOptionsBuilder::new()
        .padding(10)
        .thickness(8.0)
        .fill_color(Rgb([255u8, 255u8, 255u8]))
        .line_color(Rgb([100u8, 0u8, 0u8]))
        .build();

    renderer
        .render(&system, &options)
        .save("dragon_curve.png")
        .expect("Failed to save to dragon_curve.png");
}
