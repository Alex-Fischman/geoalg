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
		let dist_pl = |p, l| (norm_p(p) ^ norm_l(l)).into_iter().collect::<Vec<Scalar>>()[7];
		let rotor = |p: Multivector, a: Scalar| {
			let p = p.to_point().unwrap();
			(a / 2.0).cos() * S + (a / 2.0).sin() * (p.x * E1 + p.y * E2 + E0)
		};
		let ngon = |p, n, d, a| -> Vec<Multivector> {
			(0..n)
				.map(|i| {
					rotor(p, a + 2.0 * std::f32::consts::PI * (i as f32) / (n as f32))
						>> (p + d * E1)
				})
				.collect()
		};
		let edges = |v: &[Multivector]| -> Vec<Multivector> {
			v.windows(2).map(|s| s[0] & s[1]).chain(std::iter::once(v[v.len() - 1] & v[0])).collect()
		};
		let sat = |a: &[Multivector], b: &[Multivector]| {
			edges(a).into_iter().chain(edges(b).into_iter()).find(|i| {
				let da = a.iter().map(|j| dist_pl(*j, *i));
				let db = b.iter().map(|j| dist_pl(*j, *i));
				da.clone().into_iter().reduce(Scalar::min).unwrap()
					> db.clone().into_iter().reduce(Scalar::max).unwrap()
					|| db.clone().into_iter().reduce(Scalar::min).unwrap()
						> da.clone().into_iter().reduce(Scalar::max).unwrap()
			})
		};

		let a = ngon(mouse, 7, 1.0, 0.0);
		let b = ngon(E0, 4, 2.0, 1.0);
		let c = sat(&a, &b);

		let b_color = if let Some(_) = c {
			Color::GREEN
		} else {
			Color::RED
		};

		let _label = |s: &str, x, y| Some((s.to_string(), Point::new(x, y)));
		let bivectors: Vec<(Multivector, Color, Option<(String, Point)>)> = edges(&a)
			.into_iter()
			.map(|p| (p, Color::BLACK, None))
			.chain(edges(&b).into_iter().map(|p| (p, b_color, None)))
			.collect();
		let vectors: Vec<(Multivector, Color, Option<(String, Point)>)> = a
			.into_iter()
			.map(|p| (p, Color::BLACK, None))
			.chain(b.into_iter().map(|p| (p, b_color, None)))
			.collect();

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

		let font = Font::new(
			Typeface::new("Computer Modern", FontStyle::normal()).unwrap(),
			Some(18.0),
		);

		paint.set_stroke_width(0.025);
		for (bv, color, label) in bivectors {
			paint.set_color(color);
			let mut ps: Vec<Point> = vec![
				1.0 / b.top() * e2,
				1.0 / b.bottom() * e2,
				1.0 / b.left() * e1,
				1.0 / b.right() * e1,
			]
			.into_iter()
			.map(|edge| (edge + e0) ^ bv)
			.filter_map(|v| v.to_point())
			.collect();
			ps.sort_by(|a, b| {
				a.length().partial_cmp(&b.length()).unwrap_or(std::cmp::Ordering::Equal)
			});
			if ps.len() >= 2 {
				canvas.draw_line(ps[0], ps[1], &paint);
				if let Some((string, offset)) = label {
					paint.set_style(paint::Style::Fill);
					let dir = ps[1] - ps[0];
					let mut nor = Point::new(dir.y, -dir.x);
					nor.set_length(offset.y);
					let midpoint = ps[0] + dir.scaled(offset.x) + nor;
					let origin = canvas.local_to_device_as_3x3().map_point(midpoint);
					let r = font.measure_str(&string, Some(&paint)).1;
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
		for (v, color, label) in vectors {
			if let Some(tip) = v.to_point() {
				paint.set_color(color);
				canvas.draw_circle(tip, 0.05, &paint);
				if let Some((string, position)) = label {
					let origin = canvas.local_to_device_as_3x3().map_point(tip + position);
					let r = font.measure_str(&string, Some(&paint)).1;
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
