mod app;
mod pga;
mod shapes;

use app::*;
use pga::*;
use shapes::*;
use skulpin::app::*;
use skulpin::skia_safe::*;

fn main() {
	let ground = Transformed::new(Rectangle::new(5.0, 1.0));
	ground.borrow_mut().compose(Multivector::translator(0.0, -1.0));
	let player = Player::new(ground.clone());
	let objects: Vec<Wrapped<dyn Object>> = vec![player, Filled::new(ground, Color::BLACK)];

	let scale = 5.0;
	AppBuilder::new()
		.coordinate_system(skulpin::CoordinateSystem::VisibleRange(
			Rect::new(-scale, scale, scale, -scale),
			skulpin::skia_safe::matrix::ScaleToFit::Center,
		))
		.window_title("Geometric Algebra")
		.run(app::App::new(objects));
}

struct Player<T: Polygon> {
	collider: Wrapped<Transformed<Rectangle>>,
	ground: Wrapped<T>,
}

impl<T: Polygon> Player<T> {
	fn new(ground: Wrapped<T>) -> Wrapped<Player<T>> {
		std::rc::Rc::new(std::cell::RefCell::new(Player {
			collider: Transformed::new(Rectangle::new(0.25, 0.5)),
			ground,
		}))
	}
}

impl<T: Polygon> Object for Player<T> {
	fn update(&mut self, args: &AppUpdateArgs) {
		let epsilon = scalar::EPSILON * 100.0;
		let approx = |a: scalar, b: scalar| (a - b).abs() < epsilon;
		let in_segment = |a: Multivector, b, c| approx(a.dist(c) + c.dist(b), a.dist(b));

		let mut x = args.input_state.is_key_down(VirtualKeyCode::D) as u32 as f32
			- args.input_state.is_key_down(VirtualKeyCode::A) as u32 as f32;
		let mut y = args.input_state.is_key_down(VirtualKeyCode::W) as u32 as f32
			- args.input_state.is_key_down(VirtualKeyCode::S) as u32 as f32;
		let r = (x * x + y * y).sqrt();
		if r > 1.0 {
			x /= r;
			y /= r;
		}

		let speed = 2.0;
		let l = self
			.collider
			.borrow()
			.points()
			.into_iter()
			.filter_map(|p| {
				let q = p + speed * args.time_state.previous_update_dt() * (x * E1 + y * E2);
				self.ground
					.borrow()
					.edges()
					.into_iter()
					.filter_map(|(a, b)| {
						let c = (a & b) ^ (p & q);
						if in_segment(a, b, c) && in_segment(p, q, c) {
							Some(c)
						} else {
							None
						}
					})
					.chain(std::iter::once(q))
					.map(|c| c & p)
					.reduce(|a, b| if a.length() < b.length() { a } else { b })
			})
			.reduce(|a, b| if a.length() < b.length() { a } else { b })
			.unwrap_or(Z);

		self.collider.borrow_mut().compose(S + 0.5 * E0 * (e0 ^ l));
	}

	fn draw(&mut self, args: &mut AppDrawArgs) {
		Filled::new(self.collider.clone(), Color::RED).borrow_mut().draw(args);
	}
}

struct Filled<T: Polygon> {
	polygon: Wrapped<T>,
	color: Color,
}

impl<T: Polygon> Filled<T> {
	fn new(polygon: Wrapped<T>, color: Color) -> Wrapped<Filled<T>> {
		std::rc::Rc::new(std::cell::RefCell::new(Filled { polygon, color }))
	}
}

impl<T: Polygon> Object for Filled<T> {
	fn draw(&mut self, args: &mut AppDrawArgs) {
		let points = self.polygon.borrow().points().into_iter().map(|p| p.to_point().unwrap());
		args.canvas.draw_path(
			&Path::polygon(&points.collect::<Vec<Point>>(), true, None, None),
			&new_paint(self.color),
		);
	}
}
