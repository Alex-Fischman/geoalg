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
	ground.borrow_mut().transform = E1.motor(-1.0);
	let player = Player::new();
	let objects: Vec<Wrapped<dyn Object>> = vec![Filled::new(ground, Color::BLACK), player];

	let scale = 5.0;
	AppBuilder::new()
		.coordinate_system(skulpin::CoordinateSystem::VisibleRange(
			Rect::new(-scale, scale, scale, -scale),
			skulpin::skia_safe::matrix::ScaleToFit::Center,
		))
		.window_title("Geometric Algebra")
		.run(app::App::new(objects));
}

struct Player {
	collider: Wrapped<Transformed>,
}

impl Player {
	fn new() -> Wrapped<Player> {
		std::rc::Rc::new(std::cell::RefCell::new(Player {
			collider: Transformed::new(Rectangle::new(0.1, 0.2)),
		}))
	}
}

impl Polygon for Player {
	fn points(&self) -> Vec<Multivector> {
		self.collider.borrow().points()
	}
}

impl Object for Player {
	fn update(&mut self, args: &AppUpdateArgs) {
		let dt = args.time_state.previous_update_dt();

		let mut x = args.input_state.is_key_down(VirtualKeyCode::A) as u32 as f32
			- args.input_state.is_key_down(VirtualKeyCode::D) as u32 as f32;
		let mut y = args.input_state.is_key_down(VirtualKeyCode::W) as u32 as f32
			- args.input_state.is_key_down(VirtualKeyCode::S) as u32 as f32;
		let r = (x * x + y * y).sqrt();
		if r > 1.0 {
			x /= r;
			y /= r;
		}

		self.collider.borrow_mut().transform += dt * (x * E2 + y * E1);
	}

	fn draw(&mut self, args: &mut AppDrawArgs) {
		Filled::new(self.collider.clone(), Color::RED).borrow_mut().draw(args)
	}
}

struct Filled {
	polygon: Wrapped<dyn Polygon>,
	color: Color,
}

impl Filled {
	fn new(polygon: Wrapped<dyn Polygon>, color: Color) -> Wrapped<Filled> {
		std::rc::Rc::new(std::cell::RefCell::new(Filled { polygon, color }))
	}
}

impl Object for Filled {
	fn draw(&mut self, args: &mut AppDrawArgs) {
		let points = self.polygon.borrow().points().into_iter().map(|p| p.to_point().unwrap());
		args.canvas.draw_path(
			&Path::polygon(&points.collect::<Vec<Point>>(), true, None, None),
			&new_paint(self.color),
		);
	}
}
