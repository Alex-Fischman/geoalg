use crate::app::*;
use crate::pga::*;
use skulpin::app::*;
use skulpin::skia_safe::*;

pub trait Polygon {
	fn points(&self) -> Vec<Multivector>;

	fn edges(&self) -> Vec<(Multivector, Multivector)> {
		let mut points = self.points();
		points.push(points[0]);
		points.windows(2).map(|s| (s[0], s[1])).collect()
	}
}

impl<T: Polygon> Object for T {
	fn draw(&mut self, args: &mut AppDrawArgs) {
		let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);
		paint.set_stroke_width(0.025);
		for (a, b) in self.edges() {
			args.canvas.draw_line(a.to_point().unwrap(), b.to_point().unwrap(), &paint);
			args.canvas.draw_circle(a.to_point().unwrap(), 0.05, &paint);
		}
	}
}

pub struct RegularPolygon {
	sides: usize,
	radius: scalar,
}

impl RegularPolygon {
	pub fn new(sides: usize, radius: scalar) -> Wrapped<RegularPolygon> {
		wrap(RegularPolygon { sides, radius })
	}
}

impl Polygon for RegularPolygon {
	fn points(&self) -> Vec<Multivector> {
		(0..self.sides)
			.map(|i| i as f32 / self.sides as f32)
			.map(|i| E0.rotor(std::f32::consts::PI * i) >> (self.radius * E2 + E0))
			.collect()
	}
}

pub struct TransformedPolygon {
	polygon: Wrapped<dyn Polygon>,
	pub transform: Multivector,
}

impl TransformedPolygon {
	pub fn new(polygon: Wrapped<dyn Polygon>) -> Wrapped<TransformedPolygon> {
		wrap(TransformedPolygon { polygon, transform: S })
	}
}

impl Polygon for TransformedPolygon {
	fn points(&self) -> Vec<Multivector> {
		self.polygon.borrow().points().into_iter().map(|p| self.transform >> p).collect()
	}
}
