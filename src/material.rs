use {
	crate::{
		math::*,
		world::*,
	},
	rand::prelude::*,
	std::rc::Rc,
};

pub type MaterialRef = Rc<dyn Material>;

pub trait Material {
	fn bounce(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
	albedo: Color,
}

impl Lambertian {
	pub fn new(albedo: Color) -> Self {
		Self { albedo }
	}
}

impl Material for Lambertian {
	fn bounce(&self, _ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
		let mut direction = hit.normal + Vec3::rand_in_unit_sphere().norm();
		if direction.is_near_zero() {
			direction = hit.normal;
		}

		let scattered = Ray::new(hit.impact, direction);
		Some((self.albedo, scattered))
	}
}

pub struct Metal {
	albedo: Color,
	fuzz: Float,
}

impl Metal {
	pub fn new(albedo: Color, fuzz: Float) -> Self {
		Self { albedo, fuzz }
	}
}

impl Material for Metal {
	fn bounce(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
		let direction = ray.direction.reflect(hit.normal).norm();
		let scattered = Ray::new(
			hit.impact,
			direction + self.fuzz * Vec3::rand_in_unit_sphere(),
		);

		if direction.dot(hit.normal) > 0.0 {
			Some((self.albedo, scattered))
		} else {
			None
		}
	}
}

pub struct Dielectric {
	ir: Float,
}

impl Dielectric {
	pub fn new(ir: Float) -> Self {
		Self { ir }
	}

	fn reflectance(cosine: Float, ref_idx: Float) -> Float {
		// Use Schlick's approximation for reflectance
		let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
		r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
	}
}

impl Material for Dielectric {
	fn bounce(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
		let refraction_ratio = if hit.front_face {
			1.0 / self.ir
		} else {
			self.ir
		};

		let direction = ray.direction.norm();
		let cos_theta = ((-1.0) * direction).dot(hit.normal).min(1.0);
		let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

		let mut rng = rand::thread_rng();
		let cannot_refract = refraction_ratio * sin_theta > 1.0;
		let will_reflect = rng.gen::<Float>() < Self::reflectance(cos_theta, refraction_ratio);

		let direction = if cannot_refract || will_reflect {
			direction.reflect(hit.normal)
		} else {
			direction.refract(hit.normal, refraction_ratio)
		};

		let scattered = Ray::new(hit.normal, direction);

		Some((Color::new(1.0, 1.0, 1.0), scattered))
	}
}
