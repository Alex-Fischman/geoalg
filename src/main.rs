mod vector;

use skulpin::skia_safe::*;
use vector::{Vector, I, X, Y};

fn main() {
	assert_eq!(X ^ Y, I);
	assert_eq!(Y ^ X, -1.0 * I);

	let scale = 5.0;
	let vs = vec![
		(X, Color::RED, Some(("x".to_string(), (0.1 * Y).to_point()))),
		(
			Y,
			Color::BLUE,
			Some(("y".to_string(), (-0.1 * X).to_point())),
		),
		(X - Y, Color::GREEN, None),
	];

	skulpin::app::AppBuilder::new()
		.coordinate_system(skulpin::CoordinateSystem::VisibleRange(
			Rect::new(-scale, scale, scale, -scale),
			skulpin::skia_safe::matrix::ScaleToFit::Center,
		))
		.run(App { vs });
}

struct App {
	vs: Vec<(Vector, Color, Option<(String, Point)>)>,
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
		for (v, color, label) in &self.vs {
			paint.set_style(paint::Style::Stroke);
			paint.set_color(*color);
			let tip = v.to_point();
			canvas.draw_line(Point::new(0.0, 0.0), tip, &paint);
			let a = f32::atan2(tip.y, tip.x) + 0.5;
			canvas.draw_line(tip, tip - Point::new(a.cos(), a.sin()).scaled(0.1), &paint);
			let a = f32::atan2(tip.y, tip.x) - 0.5;
			canvas.draw_line(tip, tip - Point::new(a.cos(), a.sin()).scaled(0.1), &paint);
			if let Some((string, offset)) = label {
				paint.set_style(paint::Style::Fill);
				let origin = canvas.local_to_device_as_3x3().map_point(tip / 2.0 + *offset);
				let r = font.measure_str(string, Some(&paint)).1;
				let offset = Point::new(r.width(), -r.height()) / 2.0;
				canvas.save();
				canvas.reset_matrix();
				canvas.draw_str(string, origin - offset, &font, &paint);
				canvas.restore();
			}
		}

		paint.set_style(paint::Style::Fill);
		paint.set_color(Color::BLACK);
		canvas.draw_circle(Point::new(0.0, 0.0), 0.05, &paint);
	}
}
