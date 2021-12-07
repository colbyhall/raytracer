use crate::{
	material::MaterialRef,
	math::*,
};

pub struct Hit {
	pub impact: Point3,
	pub normal: Vec3,
	pub time: Float,
	pub material: MaterialRef,
	pub front_face: bool,
}

pub trait Hittable {
	fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Hit>;
}

pub struct Sphere {
	pub center: Point3,
	pub radius: Float,
	pub material: MaterialRef,
}

impl Sphere {
	pub fn new(center: Point3, radius: Float, material: &MaterialRef) -> Self {
		Self {
			center,
			radius,
			material: material.clone(),
		}
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
			material: self.material.clone(),
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

pub struct Camera {
	origin: Point3,
	horizontal: Vec3,
	vertical: Vec3,
	bottom_left: Vec3,
}

impl Camera {
	pub fn new(origin: Point3, look_at: Point3, vfov: Float, aspect_ratio: Float) -> Self {
		// Vertical field-of-view in degrees
		let theta = PI / 180.0 * vfov;
		let viewport_height = 2.0 * (theta / 2.0).tan();
		let viewport_width = aspect_ratio * viewport_height;

		let cw = (origin - look_at).norm();
		let cu = Vec3::UP.cross(cw).norm();
		let cv = cw.cross(cu);

		let horizontal = viewport_width * cu;
		let vertical = viewport_height * cv;

		let bottom_left = origin - horizontal / 2.0 - vertical / 2.0 - cw;

		Self {
			origin,
			horizontal,
			vertical,
			bottom_left,
		}
	}

	pub fn ray_at(&self, u: Float, v: Float) -> Ray {
		Ray::new(
			self.origin,
			self.bottom_left + u * self.horizontal + v * self.vertical - self.origin,
		)
	}
}
