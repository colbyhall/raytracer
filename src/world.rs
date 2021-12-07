use crate::math::*;

pub struct Hit {
	pub impact: Point3,
	pub normal: Vec3,
	pub time: Float,
	pub front_face: bool,
}

pub trait Hittable {
	fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Hit>;
}

pub struct Sphere {
	pub center: Point3,
	pub radius: Float,
}

impl Sphere {
	pub fn new(center: Point3, radius: Float) -> Self {
		Self { center, radius }
	}
}

impl Hittable for Sphere {
	fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Hit> {
		let oc = ray.origin - self.center;
		let a = ray.direction.len_sq();
		let b = oc.dot(ray.direction);
		let c = oc.len_sq() - self.radius * self.radius;
		let d = b * b - a * c;

		if d < 0.0 {
			return None;
		}

		let d = d.sqrt();
		let mut time = (-b - d) / a;
		if time < t_min || time > t_max {
			time = (-b + d) / a;
			if time < t_min || time > t_max {
				return None;
			}
		}

		let impact = ray.at(time);
		let normal = (impact - self.center) / self.radius;
		let front_face = ray.direction.dot(normal) < 0.0;
		let normal = if front_face { normal } else { -normal };
		Some(Hit {
			impact,
			normal,
			time,
			front_face,
		})
	}
}

pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
	fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Hit> {
		let mut result = None;
		let mut closest = t_max;

		for object in self.iter() {
			if let Some(hit) = object.hit(ray, t_min, closest) {
				closest = hit.time;
				result = Some(hit);
			}
		}

		result
	}
}
