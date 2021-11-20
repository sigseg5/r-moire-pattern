use draw::*;

fn main() {
    let mut canvas = Canvas::new(250, 250);

    for i in (0..1000).step_by(2) {

        let circle = Drawing::new()
            .with_shape(Shape::Circle { radius: i })
            .with_xy(125., 125.)
            .with_style(Style::stroked(1, Color::gray(11)));
        canvas.display_list.add(circle);

        let s_circle = Drawing::new()
            .with_shape(Shape::Circle { radius: i + 27 })
            .with_xy(70., 160.)
            .with_style(Style::stroked(1, Color::black()));
        canvas.display_list.add(s_circle);
    }

    render::save(
        &canvas,
        &*format!("{}/target/debug/test.svg", env!("CARGO_MANIFEST_DIR")),
        SvgRenderer::new(),
    )
    .expect("Failed to save");
}
