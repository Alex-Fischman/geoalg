use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Multivector {
	pub s: f32,
	pub x: f32,
	pub y: f32,
	pub i: f32,
}

pub const S: Multivector = Multivector { s: 1.0, x: 0.0, y: 0.0, i: 0.0 };
pub const X: Multivector = Multivector { s: 0.0, x: 1.0, y: 0.0, i: 0.0 };
pub const Y: Multivector = Multivector { s: 0.0, x: 0.0, y: 1.0, i: 0.0 };
pub const I: Multivector = Multivector { s: 0.0, x: 0.0, y: 0.0, i: 1.0 };

impl Add for Multivector {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		Self {
			s: self.s + other.s,
			x: self.x + other.x,
			y: self.y + other.y,
			i: self.i + other.i,
		}
	}
}

impl Sub for Multivector {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		Self {
			s: self.s - other.s,
			x: self.x - other.x,
			y: self.y - other.y,
			i: self.i - other.i,
		}
	}
}

impl Mul<Multivector> for f32 {
	type Output = Multivector;
	fn mul(self, other: Multivector) -> Multivector {
		Multivector {
			s: self * other.s,
			x: self * other.x,
			y: self * other.y,
			i: self * other.i,
		}
	}
}

impl Div<f32> for Multivector {
	type Output = Self;
	fn div(self, other: f32) -> Self {
		Multivector {
			s: self.s / other,
			x: self.x / other,
			y: self.y / other,
			i: self.i / other,
		}
	}
}

impl Neg for Multivector {
	type Output = Self;
	fn neg(self) -> Self {
		Multivector { s: -self.s, x: -self.x, y: -self.y, i: -self.i }
	}
}

impl Mul for Multivector {
	type Output = Self;
	fn mul(self, other: Self) -> Self {
		Multivector {
			s: self.s * other.s + self.x * other.x + self.y * other.y - self.i * other.i,
			x: self.s * other.x + self.x * other.s - self.y * other.i + self.i * other.y,
			y: self.s * other.y + self.x * other.i + self.y * other.s - self.i * other.x,
			i: self.s * other.i + self.x * other.y - self.y * other.x + self.i * other.s,
		}
	}
}

impl BitOr for Multivector {
	type Output = Self;
	fn bitor(self, other: Self) -> Self {
		Multivector {
			s: self.s * other.s + self.x * other.x + self.y * other.y - self.i * other.i,
			x: self.s * other.x + self.x * other.s,
			y: self.s * other.y + self.y * other.s,
			i: self.s * other.i + self.i * other.s,
		}
	}
}

impl BitXor for Multivector {
	type Output = Self;
	fn bitxor(self, other: Self) -> Self {
		Multivector {
			s: self.s * other.s,
			x: self.s * other.x + self.x * other.s - self.y * other.i + self.i * other.y,
			y: self.s * other.y + self.x * other.i + self.y * other.s - self.i * other.x,
			i: self.s * other.i + self.x * other.y - self.y * other.x + self.i * other.s,
		}
	}
}

impl Shl for Multivector {
	type Output = Multivector;
	fn shl(self, other: Self) -> Self {
		other.inv() * self * other
	}
}

impl Multivector {
	pub fn rotor(theta: f32) -> Self {
		(theta / 2.0).cos() * S + (theta / 2.0).sin() * I
	}

	pub fn dual(self) -> Self {
		Multivector { s: self.i, x: self.y, y: -self.x, i: -self.s }
	}

	pub fn inv(self) -> Self {
		Self { s: self.s, x: -self.x, y: -self.y, i: -self.i }
			/ (self.s * self.s - self.x * self.x - self.y * self.y + self.i * self.i)
	}
}
