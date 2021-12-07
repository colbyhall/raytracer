use rand::prelude::*;
use std::ops::{
	Add,
	AddAssign,
	Div,
	DivAssign,
	Mul,
	MulAssign,
	Neg,
	Range,
	Sub,
	SubAssign,
};

pub type Float = f64;

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Vec3 {
	pub x: Float,
	pub y: Float,
	pub z: Float,
}

impl Vec3 {
	pub const ZERO: Self = Self::splat(0.0);
	pub const ONE: Self = Self::splat(1.0);

	pub const FORWARD: Self = Self::new(1.0, 0.0, 0.0);
	pub const RIGHT: Self = Self::new(0.0, 1.0, 0.0);
	pub const UP: Self = Self::new(0.0, 0.0, 1.0);

	pub const fn new(x: Float, y: Float, z: Float) -> Self {
		Self { x, y, z }
	}

	pub const fn splat(xyz: Float) -> Self {
		Self {
			x: xyz,
			y: xyz,
			z: xyz,
		}
	}

	pub fn rand(range: Range<Float>) -> Self {
		let mut rng = rand::thread_rng();
		Self {
			x: rng.gen_range(range.clone()),
			y: rng.gen_range(range.clone()),
			z: rng.gen_range(range),
		}
	}

	pub fn rand_in_unit_sphere() -> Self {
		loop {
			let v = Vec3::rand(-1.0..1.0);
			if v.len() < 1.0 {
				return v;
			}
		}
	}

	pub const fn dot(self, other: Self) -> Float {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	pub const fn cross(self, other: Self) -> Self {
		Self {
			x: self.y * other.z - other.y * self.z,
			y: self.z * other.x - other.z * self.x,
			z: self.x * other.y - other.x * self.y,
		}
	}

	pub const fn len_sq(self) -> Float {
		self.dot(self)
	}

	pub fn len(self) -> Float {
		self.len_sq().sqrt()
	}

	pub fn is_empty(self) -> bool {
		self.len() <= 0.0
	}

	pub fn norm(self) -> Self {
		self / self.len()
	}

	pub fn abs(self) -> Self {
		Self::new(self.x.abs(), self.y.abs(), self.z.abs())
	}
}

impl Add for Vec3 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}

impl Add<Float> for Vec3 {
	type Output = Self;

	fn add(self, rhs: Float) -> Self::Output {
		Self {
			x: self.x + rhs,
			y: self.y + rhs,
			z: self.z + rhs,
		}
	}
}

impl AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

impl AddAssign<Float> for Vec3 {
	fn add_assign(&mut self, rhs: Float) {
		self.x += rhs;
		self.y += rhs;
		self.z += rhs;
	}
}

impl Sub for Vec3 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}

impl Sub<Float> for Vec3 {
	type Output = Self;

	fn sub(self, rhs: Float) -> Self::Output {
		Self {
			x: self.x - rhs,
			y: self.y - rhs,
			z: self.z - rhs,
		}
	}
}

impl SubAssign for Vec3 {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}

impl SubAssign<Float> for Vec3 {
	fn sub_assign(&mut self, rhs: Float) {
		self.x -= rhs;
		self.y -= rhs;
		self.z -= rhs;
	}
}

impl Mul for Vec3 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
			z: self.z * rhs.z,
		}
	}
}

impl Mul<Float> for Vec3 {
	type Output = Self;

	fn mul(self, rhs: Float) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}

impl Mul<Vec3> for Float {
	type Output = Vec3;

	fn mul(self, rhs: Vec3) -> Self::Output {
		Self::Output {
			x: self * rhs.x,
			y: self * rhs.y,
			z: self * rhs.z,
		}
	}
}

impl MulAssign for Vec3 {
	fn mul_assign(&mut self, rhs: Self) {
		self.x *= rhs.x;
		self.y *= rhs.y;
		self.z *= rhs.z;
	}
}

impl MulAssign<Float> for Vec3 {
	fn mul_assign(&mut self, rhs: Float) {
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
	}
}

impl Div for Vec3 {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
			z: self.z / rhs.z,
		}
	}
}

impl Div<Float> for Vec3 {
	type Output = Self;

	fn div(self, rhs: Float) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
		}
	}
}

impl DivAssign for Vec3 {
	fn div_assign(&mut self, rhs: Self) {
		self.x /= rhs.x;
		self.y /= rhs.y;
		self.z /= rhs.z;
	}
}

impl DivAssign<Float> for Vec3 {
	fn div_assign(&mut self, rhs: Float) {
		self.x /= rhs;
		self.y /= rhs;
		self.z /= rhs;
	}
}

impl Neg for Vec3 {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Ray {
	pub origin: Point3,
	pub direction: Vec3,
}

impl Ray {
	pub fn new(origin: Point3, direction: Vec3) -> Self {
		Self { origin, direction }
	}

	pub fn at(&self, t: Float) -> Point3 {
		self.origin + self.direction * t
	}
}
