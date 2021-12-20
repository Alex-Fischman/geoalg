use crate::app::*;
use crate::pga::*;
use skulpin::skia_safe::*;

pub trait Polygon {
	fn points(&self) -> Vec<Multivector>;

	fn edges(&self) -> Vec<(Multivector, Multivector)> {
		let mut points = self.points();
		points.push(points[0]);
		points.windows(2).map(|s| (s[0], s[1])).collect()
	}
}

pub fn _collision(a: Wrapped<dyn Polygon>, b: Wrapped<dyn Polygon>) -> bool {
	let a = a.borrow();
	let b = b.borrow();
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
		.next()
		.is_none()
}

pub struct Transformed {
	polygon: Wrapped<dyn Polygon>,
	pub transform: Multivector,
}

impl Transformed {
	pub fn new(polygon: Wrapped<dyn Polygon>) -> Wrapped<Transformed> {
		std::rc::Rc::new(std::cell::RefCell::new(Transformed {
			polygon,
			transform: S,
		}))
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
			E0 + self.width * E1 + self.height * E2,
			E0 - self.width * E1 + self.height * E2,
			E0 - self.width * E1 - self.height * E2,
			E0 + self.width * E1 - self.height * E2,
		]
	}
}
