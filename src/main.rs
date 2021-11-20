use draw::*;
use rand::{thread_rng, Rng};

fn main() {
    let mut canvas = Canvas::new(250, 250);

    let mut rng = thread_rng();
    let positions: Vec<f32> = vec![rng.gen_range(0., 250.), rng.gen_range(0., 250.)];

    for i in (0..1000).step_by(2) {
        let circle = Drawing::new()
            .with_shape(Shape::Circle { radius: i })
            .with_xy(positions[0], positions[0])
            .with_style(Style::stroked(1, Color::gray(11)));
        canvas.display_list.add(circle);

        let s_circle = Drawing::new()
            .with_shape(Shape::Circle { radius: i + 27 })
            .with_xy(positions[1], positions[1])
            .with_style(Style::stroked(1, Color::black()));
        canvas.display_list.add(s_circle);
    }

    render::save(
        &canvas,
        &*format!("{}/result.svg", env!("CARGO_MANIFEST_DIR")),
        SvgRenderer::new(),
    )
    .expect("Failed to save");
}
