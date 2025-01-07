use draw::{render, SvgRenderer, Shape, Drawing, Canvas, Style, Color};
use rand::{thread_rng, Rng};

fn dot(x: f32, y: f32, canvas: &mut Canvas) {
    let dot = Drawing::new().with_shape(Shape::Circle { radius: 1 }).with_xy(x, y)
        .with_style(Style::filled(Color::black()));
    canvas.display_list.add(dot);
}

fn main() {

    let size = 800.0_f32;

    let mut canvas = Canvas::new(size as u32, size as u32);

    let peak = size - ((size * size) - (size/2.0)*(size/2.0)).sqrt();

    let points = vec![(0.0, size ), (size , size ), (size/2.0, peak)];

    for point in &points {
        dot(point.0, point.1, &mut canvas);
    }

    let idx = thread_rng().gen_range(0, points.len());
    let mut point = points[idx];
    for _ in 1..100000 {
        let next = points[thread_rng().gen_range(0, points.len())];
	let scaling = 2.0;
        point = ((point.0 + next.0)/scaling, (point.1 + next.1)/scaling);
        dot(point.0, point.1, &mut canvas);
    }

    // save as an svg
    render::save(&canvas, "drawings/svg/sierpinksi.svg", SvgRenderer::new()).expect("Failed to save")

}
