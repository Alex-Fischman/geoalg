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

	let player = Transformed::new(Rectangle::new(1.0, 1.0));

	let objects: Vec<Wrapped<dyn Object>> = vec![
		Filled::new(ground, Color::BLACK),
		Filled::new(player, Color::BLUE),
	];

	let scale = 5.0;
	AppBuilder::new()
		.coordinate_system(skulpin::CoordinateSystem::VisibleRange(
			Rect::new(-scale, scale, scale, -scale),
			skulpin::skia_safe::matrix::ScaleToFit::Center,
		))
		.window_title("Geometric Algebra")
		.run(app::App::new(objects));
}

struct Filled {
	polygon: Wrapped<dyn Polygon>,
	color: Color4f,
}

impl Filled {
	fn new(polygon: Wrapped<dyn Polygon>, color: Color) -> Wrapped<Filled> {
		let color = Color4f::new(
			color.r() as f32 / 255.0,
			color.g() as f32 / 255.0,
			color.b() as f32 / 255.0,
			color.a() as f32 / 255.0,
		);
		std::rc::Rc::new(std::cell::RefCell::new(Filled { polygon, color }))
	}
}

impl Object for Filled {
	fn draw(&mut self, args: &mut AppDrawArgs) {
		let points = self.polygon.borrow().points().into_iter().map(|p| p.to_point().unwrap());
		args.canvas.draw_path(
			&Path::polygon(&points.collect::<Vec<Point>>(), true, None, None),
			&Paint::new(self.color, None),
		);
	}
}
