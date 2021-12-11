use std::ops::*;

type Scalar = f32;

#[allow(non_snake_case)]
#[derive(Clone, Copy, PartialEq)]
pub struct Multivector {
	s: Scalar,
	e0: Scalar,
	e1: Scalar,
	e2: Scalar,
	E0: Scalar,
	E1: Scalar,
	E2: Scalar,
	I: Scalar,
}

impl std::fmt::Debug for Multivector {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self.into_iter()
				.zip(["", "e0", "e1", "e2", "E0", "E1", "E2", "I"])
				.filter_map(|(s, l)| if s != 0.0 {
					Some(s.to_string() + l)
				} else {
					None
				})
				.collect::<Vec<String>>()
				.join("+")
		)
	}
}

pub const Z: Multivector =
	Multivector { s: 0.0, e0: 0.0, e1: 0.0, e2: 0.0, E0: 0.0, E1: 0.0, E2: 0.0, I: 0.0 };
pub const S: Multivector = Multivector { s: 1.0, ..Z };
#[allow(non_upper_case_globals)]
pub const e0: Multivector = Multivector { e0: 1.0, ..Z };
#[allow(non_upper_case_globals)]
pub const e1: Multivector = Multivector { e1: 1.0, ..Z };
#[allow(non_upper_case_globals)]
pub const e2: Multivector = Multivector { e2: 1.0, ..Z };
pub const E0: Multivector = Multivector { E0: 1.0, ..Z };
pub const E1: Multivector = Multivector { E1: 1.0, ..Z };
pub const E2: Multivector = Multivector { E2: 1.0, ..Z };
pub const I: Multivector = Multivector { I: 1.0, ..Z };

impl Multivector {
	pub fn to_point(self) -> Option<skulpin::skia_safe::Point> {
		if self.s == 0.0 && self.e0 == 0.0 && self.e1 == 0.0 && self.e2 == 0.0 && self.I == 0.0
		{
			Some(skulpin::skia_safe::Point::new(
				self.E1 / self.E0,
				self.E2 / self.E0,
			))
		} else {
			None
		}
	}
}

impl IntoIterator for Multivector {
	type Item = Scalar;
	type IntoIter = MultivectorIterator;
	fn into_iter(self) -> MultivectorIterator {
		MultivectorIterator { m: self, i: 0 }
	}
}

impl FromIterator<Scalar> for Multivector {
	fn from_iter<T: IntoIterator<Item = Scalar>>(i: T) -> Multivector {
		let mut iter = i.into_iter();
		Multivector {
			s: iter.next().unwrap(),
			e0: iter.next().unwrap(),
			e1: iter.next().unwrap(),
			e2: iter.next().unwrap(),
			E0: iter.next().unwrap(),
			E1: iter.next().unwrap(),
			E2: iter.next().unwrap(),
			I: iter.next().unwrap(),
		}
	}
}

pub struct MultivectorIterator {
	m: Multivector,
	i: usize,
}

impl Iterator for MultivectorIterator {
	type Item = Scalar;
	fn next(&mut self) -> Option<Scalar> {
		self.i += 1;
		match self.i {
			1 => Some(self.m.s),
			2 => Some(self.m.e0),
			3 => Some(self.m.e1),
			4 => Some(self.m.e2),
			5 => Some(self.m.E0),
			6 => Some(self.m.E1),
			7 => Some(self.m.E2),
			8 => Some(self.m.I),
			_ => None,
		}
	}
}

impl Add for Multivector {
	type Output = Multivector;
	fn add(self, other: Multivector) -> Multivector {
		self.into_iter().zip(other.into_iter()).map(|(a, b)| a + b).collect()
	}
}

impl Mul<Multivector> for Scalar {
	type Output = Multivector;
	fn mul(self, other: Multivector) -> Multivector {
		other.into_iter().map(|b| self * b).collect()
	}
}

impl Neg for Multivector {
	type Output = Multivector;
	fn neg(self) -> Multivector {
		-1.0 * self
	}
}

impl Sub for Multivector {
	type Output = Multivector;
	fn sub(self, other: Multivector) -> Multivector {
		self + -other
	}
}

impl Mul for Multivector {
	type Output = Multivector;
	fn mul(self, other: Multivector) -> Multivector {
		self.into_iter()
			.enumerate()
			.map(|(i, a)| {
				other
					.into_iter()
					.enumerate()
					.map(|(j, b)| {
						a * b
							* [
								[S, e0, e1, e2, E0, E1, E2, I],
								[e0, Z, E2, -E1, I, Z, Z, Z],
								[e1, -E2, S, E0, e2, I, -e0, E1],
								[e2, E1, -E0, S, -e1, e0, I, E2],
								[E0, I, -e2, e1, -S, -E2, E1, -e0],
								[E1, Z, I, -e0, E2, Z, Z, Z],
								[E2, Z, e0, I, -E1, Z, Z, Z],
								[I, Z, E1, E2, -e0, Z, Z, Z],
							][i][j]
					})
					.reduce(Multivector::add)
					.unwrap()
			})
			.reduce(Multivector::add)
			.unwrap()
	}
}

type Grade = i32;

impl Multivector {
	fn grade(self) -> Grade {
		let has_scalar = self.s != 0.0;
		let has_vector = self.e0 != 0.0 || self.e1 != 0.0 || self.e2 != 0.0;
		let has_bivector = self.E0 != 0.0 || self.E1 != 0.0 || self.E2 != 0.0;
		let has_trivector = self.I != 0.0;
		match (has_scalar, has_vector, has_bivector, has_trivector) {
			(true, false, false, false) => 0,
			(false, true, false, false) => 1,
			(false, false, true, false) => 2,
			(false, false, false, true) => 3,
			(false, false, false, false) => 0,
			_ => panic!("{:?} has multiple grades", self),
		}
	}

	fn component(self, grade: Grade) -> Multivector {
		match grade {
			0 => self.s * S,
			1 => self.e0 * e0 + self.e1 * e1 + self.e2 * e2,
			2 => self.E0 * E0 + self.E1 * E1 + self.E2 * E2,
			3 => self.I * I,
			_ => Z,
		}
	}
}

impl BitOr for Multivector {
	type Output = Multivector;
	fn bitor(self, other: Multivector) -> Multivector {
		(self * other).component(other.grade() - self.grade())
	}
}

impl BitXor for Multivector {
	type Output = Multivector;
	fn bitxor(self, other: Multivector) -> Multivector {
		(self * other).component(other.grade() + self.grade())
	}
}

impl Not for Multivector {
	type Output = Multivector;
	fn not(self) -> Multivector {
		if self.grade() == 1 {
			self.e0 * E0 + self.e1 * E1 + self.e2 * E2
		} else if self.grade() == 2 {
			self.E0 * e0 + self.E1 * e1 + self.E2 * e2
		} else {
			todo!();
		}
	}
}

impl BitAnd for Multivector {
	type Output = Multivector;
	fn bitand(self, other: Multivector) -> Multivector {
		!((!self) ^ (!other))
	}
}