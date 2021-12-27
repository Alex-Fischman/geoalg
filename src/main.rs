use std::ops::*;

type Scalar = f32;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vector<const P: usize, const N: usize, const Z: usize> {
	p: [Scalar; P],
	n: [Scalar; N],
	z: [Scalar; Z],
}

use std::iter::Chain;
use std::array::IntoIter;
impl<const P: usize, const N: usize, const Z: usize> IntoIterator for Vector<P, N, Z> {
    type Item = Scalar;
    type IntoIter = Chain<Chain<IntoIter<f32, P>, IntoIter<f32, N>>, IntoIter<f32, Z>>;
    fn into_iter(self) -> Self::IntoIter {
        self.p.into_iter().chain(self.n).chain(self.z)
    }
}

impl<const P: usize, const N: usize, const Z: usize> FromIterator<Scalar> for Vector<P, N, Z> {
    fn from_iter<I: IntoIterator<Item=Scalar>>(i: I) -> Self {
        let mut iter = i.into_iter();
        Vector {
            p: iter.by_ref().take(P).collect::<Vec<_>>().try_into().unwrap(),
            n: iter.by_ref().take(N).collect::<Vec<_>>().try_into().unwrap(),
            z: iter.by_ref().take(Z).collect::<Vec<_>>().try_into().unwrap(),
        }
    }
}

impl<const P: usize, const N: usize, const Z: usize> Add for Vector<P, N, Z> {
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
        self.into_iter().zip(other).map(|(a, b)| a + b).collect()
	}
}

impl<const P: usize, const N: usize, const Z: usize> Mul<Vector<P, N, Z>> for Scalar {
	type Output = Vector<P, N, Z>;
	fn mul(self, other: Self::Output) -> Self::Output {
        other.into_iter().map(|a| self * a).collect()
	}
}

impl<const P: usize, const N: usize, const Z: usize> BitOr for Vector<P, N, Z> {
	type Output = Scalar;
	fn bitor(self, other: Self) -> Self::Output {
		self.p.iter().zip(other.p).map(|(a, b)| a * b).fold(0.0, Scalar::add)
			- self.n.iter().zip(other.n).map(|(a, b)| a * b).fold(0.0, Scalar::add)
	}
}

impl<const P: usize, const N: usize, const Z: usize> Normed for Vector<P, N, Z> {}

trait Normed: Copy + BitOr<Output = Scalar> {
    fn norm(self) -> Scalar {
        (self | self).abs().sqrt()
    }
}

fn main() {
	let x = Vector { p: [1.0, 0.0], n: [], z: [0.0] };
	let y = Vector { p: [0.0, 1.0], n: [], z: [0.0] };

	assert_eq!((3.0 * x + 4.0 * y).norm(), 5.0);

	println!("Hello, world!");
}
