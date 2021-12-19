mod app;
mod pga;
mod shapes;

use app::*;
use pga::*;
use shapes::*;
use skulpin::app::*;
use skulpin::skia_safe::*;

fn main() {
	let central_polygon = RegularPolygon::new(3, 2.0);
	let moving_polygon = TransformedPolygon::new(RegularPolygon::new(5, 1.0));
	let separating_axes = SeparatingAxes::new(central_polygon.clone(), moving_polygon.clone());
	let moving_polygon_fill =
		FilledPolygon::new(moving_polygon.clone(), separating_axes.clone());
	let central_polygon_fill =
		FilledPolygon::new(central_polygon.clone(), separating_axes.clone());
	let polygon_mover = PolygonMover::new(moving_polygon.clone());
	let app = app::App::new(vec![
		polygon_mover,
		separating_axes,
		central_polygon_fill,
		central_polygon,
		moving_polygon_fill,
		moving_polygon,
	]);

	let scale = 5.0;
	AppBuilder::new()
		.coordinate_system(skulpin::CoordinateSystem::VisibleRange(
			Rect::new(-scale, scale, scale, -scale),
			skulpin::skia_safe::matrix::ScaleToFit::Center,
		))
		.window_title("Geometric Algebra")
		.run(app);
}

struct PolygonMover {
	polygon: Wrapped<TransformedPolygon>,
	matrix: Matrix,
}

impl PolygonMover {
	fn new(polygon: Wrapped<TransformedPolygon>) -> Wrapped<PolygonMover> {
		wrap(PolygonMover { polygon, matrix: Matrix::new_identity() })
	}
}

impl Object for PolygonMover {
	fn update(&mut self, args: &AppUpdateArgs) {
		if args.input_state.is_mouse_down(MouseButton::Left) {
			let p = args.input_state.mouse_position();
			let p = self.matrix.invert().unwrap().map_point(Point::new(p.x as f32, p.y as f32));
			self.polygon.borrow_mut().transform = p.y / 2.0 * E1 - p.x / 2.0 * E2 + S;
		}
	}

	fn draw(&mut self, args: &mut AppDrawArgs) {
		self.matrix = args.canvas.local_to_device_as_3x3();
	}
}

struct SeparatingAxes {
	a: Wrapped<dyn Polygon>,
	b: Wrapped<dyn Polygon>,
}

impl SeparatingAxes {
	fn new(a: Wrapped<dyn Polygon>, b: Wrapped<dyn Polygon>) -> Wrapped<SeparatingAxes> {
		wrap(SeparatingAxes { a, b })
	}

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
	polygon: Wrapped<dyn Polygon>,
	sat: Wrapped<SeparatingAxes>,
}

impl FilledPolygon {
	fn new(
		polygon: Wrapped<dyn Polygon>,
		sat: Wrapped<SeparatingAxes>,
	) -> Wrapped<FilledPolygon> {
		wrap(FilledPolygon { polygon, sat })
	}
}

impl Object for FilledPolygon {
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
