use std::ops::*;

pub use skulpin::skia_safe::scalar;

#[allow(non_snake_case)]
#[derive(Clone, Copy)]
pub struct Multivector {
	s: scalar,
	e0: scalar,
	e1: scalar,
	e2: scalar,
	E0: scalar,
	E1: scalar,
	E2: scalar,
	I: scalar,
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
		let is_grade_2 = self.s == 0.0
			&& self.e0 == 0.0
			&& self.e1 == 0.0
			&& self.e2 == 0.0
			&& self.I == 0.0;
		if is_grade_2 && self.E0 != 0.0 {
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
	type Item = scalar;
	type IntoIter = MultivectorIterator;
	fn into_iter(self) -> MultivectorIterator {
		MultivectorIterator { m: self, i: 0 }
	}
}

impl FromIterator<scalar> for Multivector {
	fn from_iter<T: IntoIterator<Item = scalar>>(i: T) -> Multivector {
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
	type Item = scalar;
	fn next(&mut self) -> Option<scalar> {
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

impl Multivector {
	fn product(self, other: Multivector, cayley: [[Multivector; 8]; 8]) -> Multivector {
		self.into_iter()
			.enumerate()
			.map(|(i, a)| {
				other
					.into_iter()
					.enumerate()
					.map(|(j, b)| a * b * cayley[i][j])
					.fold(Z, Multivector::add)
			})
			.fold(Z, Multivector::add)
	}

	fn reverse(self) -> Multivector {
		Multivector {
			s: self.s,
			e0: self.e0,
			e1: self.e1,
			e2: self.e2,
			E0: -self.E0,
			E1: -self.E1,
			E2: -self.E2,
			I: -self.I,
		}
	}

	fn conjugate(self) -> Multivector {
		Multivector {
			s: self.s,
			e0: -self.e0,
			e1: -self.e1,
			e2: -self.e2,
			E0: -self.E0,
			E1: -self.E1,
			E2: -self.E2,
			I: self.I,
		}
	}

	fn length(self) -> scalar {
		let s = (self * self.conjugate()).s;
		s.abs().sqrt().copysign(s)
	}

	fn normalized(self) -> Multivector {
		1.0 / self.length() * self
	}

	pub fn dist(self, other: Multivector) -> scalar {
		(self.normalized() & other.normalized()).length()
	}

	pub fn rotor(self, a: scalar) -> Multivector {
		a.cos() * S + a.sin() * self
	}

	pub fn translator(x: scalar, y: scalar) -> Multivector {
		S - x / 2.0 * E2 + y / 2.0 * E1
	}
}

impl Add for Multivector {
	type Output = Multivector;
	fn add(self, other: Multivector) -> Multivector {
		self.into_iter().zip(other.into_iter()).map(|(a, b)| a + b).collect()
	}
}

impl Mul<Multivector> for scalar {
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
		self.product(
			other,
			[
				[S, e0, e1, e2, E0, E1, E2, I],
				[e0, Z, E2, -E1, I, Z, Z, Z],
				[e1, -E2, S, E0, e2, I, -e0, E1],
				[e2, E1, -E0, S, -e1, e0, I, E2],
				[E0, I, -e2, e1, -S, -E2, E1, -e0],
				[E1, Z, I, -e0, E2, Z, Z, Z],
				[E2, Z, e0, I, -E1, Z, Z, Z],
				[I, Z, E1, E2, -e0, Z, Z, Z],
			],
		)
	}
}

impl BitOr for Multivector {
	type Output = Multivector;
	fn bitor(self, other: Multivector) -> Multivector {
		self.product(
			other,
			[
				[S, Z, Z, Z, Z, Z, Z, Z],
				[e0, Z, Z, Z, Z, Z, Z, Z],
				[e1, Z, S, Z, Z, Z, Z, Z],
				[e2, Z, Z, S, Z, Z, Z, Z],
				[E0, Z, -e2, e1, -S, Z, Z, Z],
				[E1, Z, Z, -e0, Z, Z, Z, Z],
				[E2, Z, e0, Z, Z, Z, Z, Z],
				[I, Z, E1, E2, -e0, Z, Z, Z],
			],
		)
	}
}

impl BitXor for Multivector {
	type Output = Multivector;
	fn bitxor(self, other: Multivector) -> Multivector {
		self.product(
			other,
			[
				[S, e0, e1, e2, E0, E1, E2, I],
				[e0, Z, E2, -E1, I, Z, Z, Z],
				[e1, -E2, Z, E0, Z, I, Z, Z],
				[e2, E1, -E0, Z, Z, Z, I, Z],
				[E0, I, Z, Z, Z, Z, Z, Z],
				[E1, Z, I, Z, Z, Z, Z, Z],
				[E2, Z, Z, I, Z, Z, Z, Z],
				[I, Z, Z, Z, Z, Z, Z, Z],
			],
		)
	}
}

impl Not for Multivector {
	type Output = Multivector;
	fn not(self) -> Multivector {
		Multivector {
			s: self.I,
			e0: self.E0,
			e1: self.E1,
			e2: self.E2,
			E0: self.e0,
			E1: self.e1,
			E2: self.e2,
			I: self.s,
		}
	}
}

impl BitAnd for Multivector {
	type Output = Multivector;
	fn bitand(self, other: Multivector) -> Multivector {
		!((!self) ^ (!other))
	}
}

impl Shr for Multivector {
	type Output = Multivector;
	fn shr(self, other: Multivector) -> Multivector {
		self * other * self.reverse()
	}
}
