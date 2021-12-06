mod vector;
use skulpin::skia_safe::*;
use vector::{Multivector, I, S, X, Y};

fn main() {
	let a = 3.0 * X + 4.0 * Y;
	let b = X + X - Y;
	let u = 1.0 * S + 2.0 * X + 3.0 * Y + 4.0 * I;
	let v = 5.0 * S - 3.0 * X + 6.0 * Y + 2.0 * I;
	let w = 2.0 * S + 1.0 * X - 7.0 * Y - 3.0 * I;

	assert_eq!(u * (v + w), u * v + u * w);
	assert_eq!((v + w) * u, v * u + w * u);
	assert_eq!((-1.5 * u) * v, u * (-1.5 * v));
	assert_eq!(u * (v * w), (u * v) * w);
	assert_eq!(1.0 * u, u);

	assert_eq!(u, u.inv().inv());
	assert_eq!(u * u.inv(), S);
	assert_eq!(u * u.inv(), u.inv() * u);

	assert_eq!(a * a, a | a);
	assert_eq!(a * b, (a | b) + (a ^ b));
	assert_eq!(a.inv(), a * (a | a).inv());

	assert_eq!(X ^ Y, -1.0 * Y ^ X);
	assert_eq!(I * I, -1.0 * S);

	assert_eq!(I.inv(), -I);
	assert_eq!(u.dual(), u * I.inv());

	assert_eq!(u | v, v | u); // for all multivectors
	assert_eq!(a ^ I, -I ^ a); // for non-multi-vectors
	assert_eq!(a ^ b, -(b ^ a)); // for vectors only?
	assert_eq!(a ^ (b ^ a), (a ^ b) ^ a); // for vectors only?

	assert_eq!((a | b).dual(), a ^ b.dual()); // for vectors only?
	assert_eq!((a ^ b).dual(), a | b.dual()); // for vectors only?

	let r = Multivector::rotor(std::f32::consts::PI * 3.0 / 2.0);
	let vw = X * (Y - X);
	skulpin::app::AppBuilder::new()
		.coordinate_system(skulpin::CoordinateSystem::VisibleRange(
			Rect::new(-1.0, 1.0, 1.0, -1.0),
			skulpin::skia_safe::matrix::ScaleToFit::Center,
		))
		.run(App {
			vs: vec![
				(X, Color::RED),
				(X << vw, Color::GREEN),
				(X << r, Color::BLUE),
			],
		});
}

fn paint_color(color: Color) -> Color4f {
	Color4f::new(
		color.r() as f32 / 255.0,
		color.g() as f32 / 255.0,
		color.b() as f32 / 255.0,
		color.a() as f32 / 255.0,
	)
}

struct App {
	vs: Vec<(Multivector, Color)>,
}

impl skulpin::app::AppHandler for App {
	fn update(&mut self, _update_args: skulpin::app::AppUpdateArgs) {}

	fn fatal_error(&mut self, _error: &skulpin::app::AppError) {}

	fn draw(&mut self, draw_args: skulpin::app::AppDrawArgs) {
		let canvas = draw_args.canvas;
		canvas.clear(Color::WHITE);

		let mut paint = Paint::new(paint_color(Color::RED), None);
		paint.set_anti_alias(true);
		paint.set_stroke_width(0.01);

		let _font = Font::new(
			Typeface::new(
				"Computer Modern",
				FontStyle::new(
					font_style::Weight::NORMAL,
					font_style::Width::NORMAL,
					font_style::Slant::Upright,
				),
			)
			.unwrap(),
			Some(24.0),
		);

		for (v, color) in &self.vs {
			paint.set_color(*color);
			paint.set_style(paint::Style::Stroke);
			let tip = Point::new(v.x, v.y);
			canvas.draw_line(Point::new(0.0, 0.0), tip, &paint);
			let angle = f32::atan2(tip.y, tip.x) + 0.5;
			let head = Point::new(angle.cos(), angle.sin()).scaled(0.1);
			canvas.draw_line(tip, tip - head, &paint);
			let angle = f32::atan2(tip.y, tip.x) - 0.5;
			let head = Point::new(angle.cos(), angle.sin()).scaled(0.1);
			canvas.draw_line(tip, tip - head, &paint);
		}
	}
}
