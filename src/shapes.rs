use crate::app::*;
use crate::pga::*;

pub trait Polygon {
	fn points(&self) -> Vec<Multivector>;

	fn edges(&self) -> Vec<(Multivector, Multivector)> {
		let mut points = self.points();
		points.push(points[0]);
		points.windows(2).map(|s| (s[0], s[1])).collect()
	}
}

pub struct Transformed {
	polygon: Wrapped<dyn Polygon>,
	transform: Multivector,
}

impl Transformed {
	pub fn new(polygon: Wrapped<dyn Polygon>) -> Wrapped<Transformed> {
		std::rc::Rc::new(std::cell::RefCell::new(Transformed {
			polygon,
			transform: S,
		}))
	}

	pub fn compose(&mut self, other: Multivector) {
		self.transform = other * self.transform;
	}
}

impl Polygon for Transformed {
	fn points(&self) -> Vec<Multivector> {
		self.polygon.borrow().points().into_iter().map(|p| self.transform >> p).collect()
	}
}

pub struct Rectangle {
	width: scalar,
	height: scalar,
}

impl Rectangle {
	pub fn new(width: scalar, height: scalar) -> Wrapped<Rectangle> {
		std::rc::Rc::new(std::cell::RefCell::new(Rectangle { width, height }))
	}
}

impl Polygon for Rectangle {
	fn points(&self) -> Vec<Multivector> {
		vec![
			E0 + self.width / 2.0 * E1 + self.height / 2.0 * E2,
			E0 - self.width / 2.0 * E1 + self.height / 2.0 * E2,
			E0 - self.width / 2.0 * E1 - self.height / 2.0 * E2,
			E0 + self.width / 2.0 * E1 - self.height / 2.0 * E2,
		]
	}
}
