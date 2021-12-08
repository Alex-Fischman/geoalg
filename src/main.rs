mod vector;

use skulpin::skia_safe::*;
use vector::{Bivector, Vector, E1, E12, E2, E23, E3, E31};

fn main() {
	assert_eq!(E1 - E1, E2 - E2);
	assert_eq!(E1 | E2, 0.0);
	assert_eq!(E2 ^ E3, E23);
	assert_eq!(E2 ^ E1, -E12);
	assert_eq!(!E1, -E23);
	assert_eq!(!E31, -E2);
	assert_eq!(E31 | E31, -1.0);
	assert_eq!(E31 | E23, 0.0);
	assert_eq!(E23 & E12, E2);

	let v = |x, y| x * E1 + y * E2 + E3;
	let label = |s: &str, x, y| Some((s.to_string(), Point::new(x, y)));

	let a = v(1.0, 2.0);
	let b = v(-2.0, 3.0);
	let c = v(-4.0, -3.0);

	let ab = a ^ b;
	let bc = b ^ c;
	let ca = c ^ a;

	let scale = 5.0;
	let vectors = vec![
		(a, Color::RED, label("a", 0.0, 0.2)),
		(b, Color::GREEN, label("b", 0.0, -0.2)),
		(c, Color::BLUE, label("c", 0.2, 0.0)),
	];
	let bivectors = vec![
		(ab, Color::RED, label("ab", -0.2, 2.6)),
		(bc, Color::GREEN, label("bc", -3.2, 0.2)),
		(ca, Color::BLUE, label("ca", -1.2, 0.2)),
	];

	skulpin::app::AppBuilder::new()
		.coordinate_system(skulpin::CoordinateSystem::VisibleRange(
			Rect::new(-scale, scale, scale, -scale),
			skulpin::skia_safe::matrix::ScaleToFit::Center,
		))
		.window_title("Geometric Algebra")
		.run(App { vectors, bivectors });
}

struct App {
	vectors: Vec<(Vector, Color, Option<(String, Point)>)>,
	bivectors: Vec<(Bivector, Color, Option<(String, Point)>)>,
}

impl skulpin::app::AppHandler for App {
	fn update(&mut self, _update_args: skulpin::app::AppUpdateArgs) {}

	fn fatal_error(&mut self, _error: &skulpin::app::AppError) {}

	fn draw(&mut self, draw_args: skulpin::app::AppDrawArgs) {
		let canvas = draw_args.canvas;
		let b = canvas.local_clip_bounds().unwrap();
		canvas.clear(Color::WHITE);

		let font = Font::new(
			Typeface::new("Computer Modern", FontStyle::normal()).unwrap(),
			Some(18.0),
		);

		let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);

		paint.set_style(paint::Style::Stroke);
		paint.set_stroke_width(0.01);
		paint.set_color(Color::GRAY);
		for i in b.left.floor() as i32..b.right.floor() as i32 + 1 {
			canvas.draw_line(
				Point::new(i as f32, b.bottom),
				Point::new(i as f32, b.top),
				&paint,
			);
		}
		for i in b.top.floor() as i32..b.bottom.floor() as i32 + 1 {
			canvas.draw_line(
				Point::new(b.left, i as f32),
				Point::new(b.right, i as f32),
				&paint,
			);
		}
		paint.set_color(Color::BLACK);
		canvas.draw_line(Point::new(b.left, 0.0), Point::new(b.right, 0.0), &paint);
		canvas.draw_line(Point::new(0.0, b.bottom), Point::new(0.0, b.top), &paint);

		paint.set_stroke_width(0.025);
		for (bv, color, label) in &self.bivectors {
			paint.set_color(*color);
			let lt = b.left() * E1 + b.top() * E2 + E3;
			let rt = b.right() * E1 + b.top() * E2 + E3;
			let lb = b.left() * E1 + b.bottom() * E2 + E3;
			let rb = b.right() * E1 + b.bottom() * E2 + E3;
			let ps: Vec<Point> = vec![lt ^ rt, lb ^ rb, lt ^ lb, rt ^ rb]
				.into_iter()
				.map(|edge| edge & *bv)
				.filter_map(Vector::to_point)
				.collect();
			if ps.len() >= 2 {
				canvas.draw_line(ps[0], ps[1], &paint);
				if let Some((string, position)) = label {
					paint.set_style(paint::Style::Fill);
					let origin = canvas.local_to_device_as_3x3().map_point(*position);
					let r = font.measure_str(string, Some(&paint)).1;
					let offset = Point::new(r.width(), -r.height()) / 2.0;
					canvas.save();
					canvas.reset_matrix();
					canvas.draw_str(string, origin - offset, &font, &paint);
					canvas.restore();
					paint.set_style(paint::Style::Stroke);
				}
			}
		}

		paint.set_style(paint::Style::Fill);
		for (v, color, label) in &self.vectors {
			if let Some(tip) = v.to_point() {
				paint.set_color(*color);
				canvas.draw_circle(tip, 0.05, &paint);
				if let Some((string, position)) = label {
					let origin = canvas.local_to_device_as_3x3().map_point(tip + *position);
					let r = font.measure_str(string, Some(&paint)).1;
					let offset = Point::new(r.width(), -r.height()) / 2.0;
					canvas.save();
					canvas.reset_matrix();
					canvas.draw_str(string, origin - offset, &font, &paint);
					canvas.restore();
				}
			}
		}

		paint.set_color(Color::BLACK);
		canvas.draw_circle(Point::new(0.0, 0.0), 0.05, &paint);
	}
}
