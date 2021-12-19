mod pga;

use pga::*;
use skulpin::app::*;
use skulpin::skia_safe::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
	let central_polygon = wrap(Polygon { sides: 3, radius: 2.0, center: E0 });
	let moving_polygon = wrap(Polygon { sides: 5, radius: 1.0, center: E0 });
	let separating_axes =
		wrap(SeparatingAxes { a: central_polygon.clone(), b: moving_polygon.clone() });
	let polygon_mover =
		wrap(PolygonMover { polygon: moving_polygon.clone(), matrix: Matrix::new_identity() });
	let moving_polygon_fill =
		wrap(FilledPolygon { polygon: moving_polygon.clone(), sat: separating_axes.clone() });
	let central_polygon_fill =
		wrap(FilledPolygon { polygon: central_polygon.clone(), sat: separating_axes.clone() });
	let app = App {
		objects: vec![
			separating_axes,
			central_polygon_fill,
			central_polygon,
			polygon_mover,
			moving_polygon_fill,
			moving_polygon,
		],
	};

	let scale = 5.0;
	AppBuilder::new()
		.coordinate_system(skulpin::CoordinateSystem::VisibleRange(
			Rect::new(-scale, scale, scale, -scale),
			skulpin::skia_safe::matrix::ScaleToFit::Center,
		))
		.window_title("Geometric Algebra")
		.run(app);
}

trait Object {
	fn update(&mut self, _args: &AppUpdateArgs) {}
	fn draw(&mut self, _args: &mut AppDrawArgs) {}
}

type Wrapped<T> = Rc<RefCell<T>>;

fn wrap<T>(t: T) -> Wrapped<T> {
	Rc::new(RefCell::new(t))
}

struct App {
	objects: Vec<Wrapped<dyn Object>>,
}

impl AppHandler for App {
	fn update(&mut self, mut args: AppUpdateArgs) {
		for object in &mut self.objects {
			object.borrow_mut().update(&mut args);
		}
	}

	fn draw(&mut self, mut args: AppDrawArgs) {
		args.canvas.clear(Color::WHITE);
		for object in &mut self.objects {
			object.borrow_mut().draw(&mut args);
		}
	}

	fn fatal_error(&mut self, _error: &AppError) {}
}

struct Polygon {
	sides: usize,
	radius: scalar,
	center: Multivector,
}

impl Polygon {
	fn points(&self) -> Vec<Multivector> {
		(0..self.sides)
			.map(|i| i as f32 / self.sides as f32)
			.map(|i| {
				self.center.motor(std::f32::consts::PI * i) >> (self.center + self.radius * E2)
			})
			.collect()
	}

	fn edges(&self) -> Vec<(Multivector, Multivector)> {
		let mut points = self.points();
		points.push(points[0]);
		points.windows(2).map(|s| (s[0], s[1])).collect()
	}
}

impl Object for Polygon {
	fn update(&mut self, _args: &AppUpdateArgs) {}

	fn draw(&mut self, args: &mut AppDrawArgs) {
		let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);
		paint.set_stroke_width(0.025);
		for (a, b) in self.edges() {
			args.canvas.draw_line(a.to_point().unwrap(), b.to_point().unwrap(), &paint);
			args.canvas.draw_circle(a.to_point().unwrap(), 0.05, &paint);
		}
	}
}

struct PolygonMover {
	polygon: Wrapped<Polygon>,
	matrix: Matrix,
}

impl Object for PolygonMover {
	fn update(&mut self, args: &AppUpdateArgs) {
		if args.input_state.is_mouse_down(MouseButton::Left) {
			let p = args.input_state.mouse_position();
			let p = self.matrix.invert().unwrap().map_point(Point::new(p.x as f32, p.y as f32));
			self.polygon.borrow_mut().center = p.x * E1 + p.y * E2 + E0;
		}
	}

	fn draw(&mut self, args: &mut AppDrawArgs) {
		self.matrix = args.canvas.local_to_device_as_3x3();
	}
}

struct SeparatingAxes {
	a: Wrapped<Polygon>,
	b: Wrapped<Polygon>,
}

impl SeparatingAxes {
	fn axes(&self) -> Vec<Multivector> {
		let a = self.a.borrow();
		let b = self.b.borrow();
		a.edges()
			.into_iter()
			.chain(b.edges().into_iter())
			.map(|(a, b)| a & b)
			.filter(|&i| {
				let da = a.points().into_iter().map(|j| (j & i).into_iter().next().unwrap());
				let db = b.points().into_iter().map(|j| (j & i).into_iter().next().unwrap());
				da.clone().fold(f32::MAX, |a, b| a.min(b))
					> db.clone().fold(f32::MIN, |a, b| a.max(b))
					|| db.fold(f32::MAX, |a, b| a.min(b)) > da.fold(f32::MIN, |a, b| a.max(b))
			})
			.collect()
	}
}

impl Object for SeparatingAxes {
	fn update(&mut self, _args: &AppUpdateArgs) {}

	fn draw(&mut self, args: &mut AppDrawArgs) {
		let mut paint = Paint::new(Color4f::new(0.5, 0.5, 0.5, 1.0), None);
		paint.set_stroke_width(0.025);
		for axis in self.axes() {
			let b = args.canvas.local_clip_bounds().unwrap();
			let mut ps: Vec<skia_safe::Point> = [
				1.0 / b.top() * e2,
				1.0 / b.bottom() * e2,
				1.0 / b.left() * e1,
				1.0 / b.right() * e1,
			]
			.into_iter()
			.map(|edge| (edge + e0) ^ axis)
			.filter_map(|v| v.to_point())
			.collect();
			ps.sort_by(|a, b| a.length().partial_cmp(&b.length()).unwrap());
			args.canvas.draw_line(ps[0], ps[1], &paint);
		}
	}
}

struct FilledPolygon {
	polygon: Wrapped<Polygon>,
	sat: Wrapped<SeparatingAxes>,
}

impl Object for FilledPolygon {
	fn update(&mut self, _args: &AppUpdateArgs) {}

	fn draw(&mut self, args: &mut AppDrawArgs) {
		let color = if self.sat.borrow().axes().is_empty() {
			Color4f::new(1.0, 0.0, 0.0, 0.5)
		} else {
			Color4f::new(0.0, 1.0, 0.0, 0.5)
		};
		args.canvas.draw_path(
			&Path::polygon(
				&self
					.polygon
					.borrow()
					.points()
					.into_iter()
					.map(|p| p.to_point().unwrap())
					.collect::<Vec<Point>>(),
				true,
				None,
				None,
			),
			&Paint::new(color, None),
		);
	}
}
