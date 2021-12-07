use std::ops::*;

type Scalar = f32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
	x: Scalar,
	y: Scalar,
}

pub const X: Vector = Vector { x: 1.0, y: 0.0 };
pub const Y: Vector = Vector { x: 0.0, y: 1.0 };

impl Add for Vector {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		Self { x: self.x + other.x, y: self.y + other.y }
	}
}

impl Sub for Vector {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		Self { x: self.x - other.x, y: self.y - other.y }
	}
}

impl Mul<Vector> for Scalar {
	type Output = Vector;
	fn mul(self, other: Vector) -> Vector {
		Vector { x: self * other.x, y: self * other.y }
	}
}

impl Div<Scalar> for Vector {
	type Output = Self;
	fn div(self, other: Scalar) -> Self {
		Vector { x: self.x / other, y: self.y / other }
	}
}

impl Neg for Vector {
	type Output = Self;
	fn neg(self) -> Self {
		Vector { x: -self.x, y: -self.y }
	}
}

impl Vector {
	pub fn norm(self) -> Scalar {
		self.x * self.x + self.y * self.y
	}

	pub fn inv(self) -> Self {
		self / self.norm()
	}

	pub fn to_point(self) -> skulpin::skia_safe::Point {
		skulpin::skia_safe::Point::new(self.x, self.y)
	}
}
