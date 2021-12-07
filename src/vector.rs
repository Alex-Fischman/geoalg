use std::ops::*;

type Scalar = f32;

trait Geometric
where
	Self: Sized + Add + Sub + Div<Scalar>,
	Scalar: Mul<Self>,
{
	fn inv(self) -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
	x: Scalar,
	y: Scalar,
}

pub const X: Vector = Vector { x: 1.0, y: 0.0 };
pub const Y: Vector = Vector { x: 0.0, y: 1.0 };

impl Add for Vector {
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
		Self { x: self.x + other.x, y: self.y + other.y }
	}
}

impl Sub for Vector {
	type Output = Self;
	fn sub(self, other: Self) -> Self::Output {
		Self { x: self.x - other.x, y: self.y - other.y }
	}
}

impl Mul<Vector> for Scalar {
	type Output = Vector;
	fn mul(self, other: Vector) -> Self::Output {
		Vector { x: self * other.x, y: self * other.y }
	}
}

impl Div<Scalar> for Vector {
	type Output = Self;
	fn div(self, other: Scalar) -> Self::Output {
		Vector { x: self.x / other, y: self.y / other }
	}
}

impl Geometric for Vector {
	fn inv(self) -> Self {
		self / self.norm()
	}
}

impl BitOr for Vector {
	type Output = Scalar;
	fn bitor(self, other: Self) -> Self::Output {
		self.x * other.x + self.y * other.y
	}
}

impl BitXor for Vector {
	type Output = Bivector;
	fn bitxor(self, other: Vector) -> Self::Output {
		(self.x * other.y - self.y * other.x) * I
	}
}

impl Vector {
	pub fn norm(self) -> Scalar {
		self.x * self.x + self.y * self.y
	}

	pub fn to_point(self) -> skulpin::skia_safe::Point {
		skulpin::skia_safe::Point::new(self.x, self.y)
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bivector {
	i: Scalar,
}

pub const I: Bivector = Bivector { i: 1.0 };

impl Add for Bivector {
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
		Self { i: self.i + other.i }
	}
}

impl Sub for Bivector {
	type Output = Self;
	fn sub(self, other: Self) -> Self::Output {
		Self { i: self.i - other.i }
	}
}

impl Mul<Bivector> for Scalar {
	type Output = Bivector;
	fn mul(self, other: Bivector) -> Self::Output {
		Bivector { i: self * other.i }
	}
}

impl Div<Scalar> for Bivector {
	type Output = Self;
	fn div(self, other: Scalar) -> Self::Output {
		Self { i: self.i / other }
	}
}

impl Geometric for Bivector {
	fn inv(self) -> Self {
		-1.0 * self / self.i
	}
}
