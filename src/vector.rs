use std::ops::*;

pub type Scalar = f32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
	e1: Scalar,
	e2: Scalar,
	e3: bool,
}

pub const E1: Vector = Vector { e1: 1.0, e2: 0.0, e3: true };
pub const E2: Vector = Vector { e1: 0.0, e2: 1.0, e3: true };
pub const E3: Vector = Vector { e1: 0.0, e2: 0.0, e3: false };

impl Vector {
	pub fn to_point(self) -> Option<skulpin::skia_safe::Point> {
		if self.e3 {
			None
		} else {
			Some(skulpin::skia_safe::Point::new(self.e1, self.e2))
		}
	}
}

impl Add for Vector {
	type Output = Vector;
	fn add(self, other: Vector) -> Vector {
		match (self.e3, other.e3) {
			(false, false) => Vector {
				e1: (self.e1 + other.e1) / 2.0,
				e2: (self.e2 + other.e2) / 2.0,
				e3: false,
			},
			(true, true) => Vector { e1: self.e1 + other.e1, e2: self.e2 + other.e2, e3: true },
			(false, true) | (true, false) => {
				Vector { e1: self.e1 + other.e1, e2: self.e2 + other.e2, e3: false }
			}
		}
	}
}

impl Mul<Vector> for Scalar {
	type Output = Vector;
	fn mul(self, other: Vector) -> Vector {
		Vector { e1: self * other.e1, e2: self * other.e2, e3: other.e3 }
	}
}

impl Neg for Vector {
	type Output = Vector;
	fn neg(self) -> Vector {
		self
	}
}

impl Sub for Vector {
	type Output = Vector;
	fn sub(self, other: Vector) -> Vector {
		self + -1.0 * other
	}
}

impl BitOr for Vector {
	type Output = Scalar;
	fn bitor(self, other: Vector) -> Scalar {
		self.e1 * other.e1 + self.e2 * other.e2
	}
}

impl BitXor for Vector {
	type Output = Bivector;
	fn bitxor(self, other: Vector) -> Bivector {
		let a = if self.e3 { 0.0 } else { 1.0 };
		let b = if other.e3 { 0.0 } else { 1.0 };
		Bivector {
			e23: self.e2 * b - a * other.e2,
			e31: a * other.e1 - self.e1 * b,
			e12: self.e1 * other.e2 - self.e2 * other.e1,
		}
	}
}

impl Not for Vector {
	type Output = Bivector;
	fn not(self) -> Bivector {
		match self.e3 {
			false => Bivector { e23: -self.e1, e31: -self.e2, e12: -1.0 },
			true => Bivector { e23: -self.e1, e31: -self.e2, e12: 0.0 },
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bivector {
	e23: Scalar,
	e31: Scalar,
	e12: Scalar,
}

pub const E23: Bivector = Bivector { e23: 1.0, e31: 0.0, e12: 0.0 };
pub const E31: Bivector = Bivector { e23: 0.0, e31: 1.0, e12: 0.0 };
pub const E12: Bivector = Bivector { e23: 0.0, e31: 0.0, e12: 1.0 };

impl Add for Bivector {
	type Output = Bivector;
	fn add(self, other: Bivector) -> Bivector {
		Bivector {
			e23: self.e23 + other.e23,
			e31: self.e31 + other.e31,
			e12: self.e12 + other.e12,
		}
	}
}

impl Mul<Bivector> for Scalar {
	type Output = Bivector;
	fn mul(self, other: Bivector) -> Bivector {
		Bivector { e23: self * other.e23, e31: self * other.e31, e12: self * other.e12 }
	}
}

impl Neg for Bivector {
	type Output = Bivector;
	fn neg(self) -> Bivector {
		Bivector { e23: -self.e23, e31: -self.e31, e12: -self.e12 }
	}
}

impl Sub for Bivector {
	type Output = Bivector;
	fn sub(self, other: Bivector) -> Bivector {
		Bivector {
			e23: self.e23 - other.e23,
			e31: self.e31 - other.e31,
			e12: self.e12 - other.e12,
		}
	}
}

impl BitOr for Bivector {
	type Output = Scalar;
	fn bitor(self, other: Bivector) -> Scalar {
		-(self.e23 * other.e23 + self.e31 * other.e31 + self.e12 * other.e12)
	}
}

impl BitAnd for Bivector {
	type Output = Vector;
	fn bitand(self, other: Bivector) -> Vector {
		!((!self) ^ (!other))
	}
}

impl Not for Bivector {
	type Output = Vector;
	fn not(self) -> Vector {
		if self.e12 == 0.0 {
			Vector { e1: self.e23, e2: self.e31, e3: true }
		} else {
			Vector { e1: self.e23 / self.e12, e2: self.e31 / self.e12, e3: false }
		}
	}
}
