mod pga;

use pga::{Scalar, *};
use skulpin::skia_safe::*;

fn main() {
	assert_eq!(E1 - E1, E2 - E2);
	assert_eq!(E1 | E2, Z);
	assert_eq!(e2 ^ e0, E1);
	assert_eq!(e2 ^ e1, -E0);
	assert_eq!(!E1, e1);
	assert_eq!(!e2, E2);
	assert_eq!(e2 | e2, S);
	assert_eq!(e2 | e1, Z);
	assert_eq!(E0 & E1, e2);
	assert_eq!(2.0 * e0 * e1, 2.0 * E2);

	let scale = 5.0;
	skulpin::app::AppBuilder::new()
		.coordinate_system(skulpin::CoordinateSystem::VisibleRange(
			Rect::new(-scale, scale, scale, -scale),
			skulpin::skia_safe::matrix::ScaleToFit::Center,
		))
		.window_title("Geometric Algebra")
		.run(App::new());
}

enum Drawable {
	Point(Multivector, Color),
	Line(Multivector, Color),
	Segment(Multivector, Multivector, Color),
}

struct App {
	mouse: Point,
}

impl App {
	fn new() -> App {
		App { mouse: Point::new(0.0, 0.0) }
	}
}

impl skulpin::app::AppHandler for App {
	fn update(&mut self, update_args: skulpin::app::AppUpdateArgs) {
		if update_args.input_state.is_mouse_down(skulpin::app::MouseButton::Left) {
			let p = update_args.input_state.mouse_position();
			self.mouse = Point::new(p.x as f32, p.y as f32);
		}
	}

	fn fatal_error(&mut self, _error: &skulpin::app::AppError) {}

	fn draw(&mut self, draw_args: skulpin::app::AppDrawArgs) {
		let canvas = draw_args.canvas;

		let mouse = canvas.local_to_device_as_3x3().invert().unwrap().map_point(self.mouse);
		let mouse = mouse.x * E1 + mouse.y * E2 + E0;

		let norm_p = |p: Multivector| {
			let a: Vec<Scalar> = p.into_iter().collect();
			(a[5] / a[4]) * E1 + (a[6] / a[4]) * E2 + E0
		};
		let norm_l = |l: Multivector| {
			let a: Vec<Scalar> = l.into_iter().collect();
			(a[2] / a[1]) * e1 + (a[3] / a[1]) * e2 + e0
		};
		let dist_pl = |p, l| (norm_p(p) & norm_l(l)).into_iter().next().unwrap();
		let ngon = |p: Multivector, n, d, a| -> Vec<Multivector> {
			(0..n)
				.map(|i| i as f32 / n as f32)
				.map(|i| p.motor(std::f32::consts::PI * (a + i)) >> (p + d * E2))
				.collect()
		};
		let edges = |v: &[Multivector]| -> Vec<(Multivector, Multivector)> {
			v.windows(2)
				.map(|s| (s[0], s[1]))
				.chain(std::iter::once((v[v.len() - 1], v[0])))
				.collect()
		};
		let sat = |a: &[Multivector], b: &[Multivector]| -> Vec<Multivector> {
			edges(a)
				.into_iter()
				.chain(edges(b).into_iter())
				.map(|(a, b)| a & b)
				.filter(|&i| {
					let da = a.iter().map(|&j| dist_pl(j, i));
					let db = b.iter().map(|&j| dist_pl(j, i));
					da.clone().fold(f32::MAX, |a, b| a.min(b))
						> db.clone().fold(f32::MIN, |a, b| a.max(b))
						|| db.fold(f32::MAX, |a, b| a.min(b))
							> da.fold(f32::MIN, |a, b| a.max(b))
				})
				.collect()
		};

		let a = ngon(mouse, 7, 1.0, 0.0);
		let b = ngon(E0, 4, 2.0, 0.25);
		let c = sat(&a, &b);

		let b_color = if !c.is_empty() {
			Color::GREEN
		} else {
			Color::RED
		};

		let lines_a: Vec<Drawable> =
			edges(&a).into_iter().map(|(a, b)| Drawable::Segment(a, b, Color::BLACK)).collect();
		let lines_b: Vec<Drawable> =
			edges(&b).into_iter().map(|(a, b)| Drawable::Segment(a, b, b_color)).collect();
		let lines_c: Vec<Drawable> =
			c.into_iter().map(|c| Drawable::Line(c, Color::GRAY)).collect();
		let points_a: Vec<Drawable> =
			a.into_iter().map(|p| Drawable::Point(p, Color::BLACK)).collect();
		let points_b: Vec<Drawable> =
			b.into_iter().map(|p| Drawable::Point(p, b_color)).collect();
		let drawables: Vec<Drawable> =
			vec![lines_c, lines_a, lines_b, points_a, points_b].into_iter().flatten().collect();

		let b = canvas.local_clip_bounds().unwrap();
		let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);

		canvas.clear(Color::WHITE);
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

		paint.set_style(paint::Style::Fill);
		paint.set_stroke_width(0.025);
		for d in drawables {
			match d {
				Drawable::Point(v, c) => {
					paint.set_color(c);
					canvas.draw_circle(v.to_point().unwrap(), 0.05, &paint);
				}
				Drawable::Segment(a, b, c) => {
					paint.set_color(c);
					canvas.draw_line(a.to_point().unwrap(), b.to_point().unwrap(), &paint);
				}
				Drawable::Line(v, c) => {
					paint.set_color(c);
					let mut ps: Vec<Point> = [
						1.0 / b.top() * e2,
						1.0 / b.bottom() * e2,
						1.0 / b.left() * e1,
						1.0 / b.right() * e1,
					]
					.into_iter()
					.map(|edge| (edge + e0) ^ v)
					.filter_map(|v| v.to_point())
					.collect();
					ps.sort_by(|a, b| {
						a.length().partial_cmp(&b.length()).unwrap_or(std::cmp::Ordering::Equal)
					});
					canvas.draw_line(ps[0], ps[1], &paint);
				}
			}
		}

		paint.set_color(Color::BLACK);
		canvas.draw_circle(Point::new(0.0, 0.0), 0.05, &paint);
	}
}
