#![feature(const_fn_floating_point_arithmetic)]

use std::rc::Rc;

mod material;
mod math;
mod world;

use {
	material::*,
	math::*,
	rand::prelude::*,
	stb::image_write,
	std::{
		ffi::CString,
		mem::size_of,
		slice::from_raw_parts,
		time::SystemTime,
	},
	world::*,
};

fn ray_color(ray: &Ray, world: &World, depth: usize) -> Color {
	if depth == 0 {
		// If we've exceeded the ray bounce limit, no more light is gathered
		return Color::new(0.0, 0.0, 0.0);
	}

	if let Some(hit) = world.hit(ray, 0.001, Float::INFINITY) {
		if let Some((attenuation, scattered)) = hit.material.bounce(ray, &hit) {
			attenuation * ray_color(&scattered, world, depth - 1)
		} else {
			Color::new(0.0, 0.0, 0.0)
		}
	} else {
		let direction = ray.direction.norm();
		let t = 0.5 * (direction.z + 1.0);
		(1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
	}
}

fn main() {
	// Image Information
	const ASPECT_RATIO: Float = 16.0 / 9.0;
	const IMAGE_WIDTH: usize = 720;
	const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as Float) / ASPECT_RATIO) as usize;
	const SAMPLES_PER_PIXEL: usize = 10;
	const MAX_DEPTH: usize = 50;

	// World data
	let mut world = World::new();
	let mat_ground: MaterialRef = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
	let mat_center: MaterialRef = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
	let mat_left: MaterialRef = Rc::new(Dielectric::new(1.5));
	let mat_left_inner: MaterialRef = Rc::new(Dielectric::new(1.5));
	let mat_right: MaterialRef = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

	let sphere_ground = Sphere::new(Point3::new(-1.0, 0.0, -100.5), 100.0, &mat_ground);
	let sphere_center = Sphere::new(Point3::new(-1.0, 0.0, 0.0), 0.5, &mat_center);
	let sphere_left = Sphere::new(Point3::new(-1.0, -1.0, 0.0), 0.5, &mat_left);
	let sphere_left_inner = Sphere::new(Point3::new(-1.0, -1.0, 0.0), -0.4, &mat_left_inner);
	let sphere_right = Sphere::new(Point3::new(-1.0, 1.0, 0.0), 0.5, &mat_right);

	world.push(Box::new(sphere_ground));
	world.push(Box::new(sphere_center));
	world.push(Box::new(sphere_left));
	world.push(Box::new(sphere_left_inner));
	world.push(Box::new(sphere_right));

	let camera = Camera::new(
		Point3::new(1.0, -2.0, 2.0),
		Point3::new(-1.0, 0.0, 0.0),
		20.0,
		ASPECT_RATIO,
	);

	let mut rng = rand::thread_rng();
	let mut pixels = vec![0; IMAGE_WIDTH * IMAGE_HEIGHT];
	for y in 0..IMAGE_HEIGHT {
		eprintln!("Scanlines remaining: {}", IMAGE_HEIGHT - y - 1);

		for x in 0..IMAGE_WIDTH {
			let mut color = Color::ZERO;
			for _ in 0..SAMPLES_PER_PIXEL {
				let random_u: f64 = rng.gen();
				let random_v: f64 = rng.gen();

				let u = ((x as Float) + random_u) / ((IMAGE_WIDTH - 1) as Float);
				let v = ((y as Float) + random_v) / ((IMAGE_HEIGHT - 1) as Float);

				let ray = camera.ray_at(u, v);
				color += ray_color(&ray, &world, MAX_DEPTH);
			}
			let color = color / SAMPLES_PER_PIXEL as Float;

			let r = (color.x.sqrt().clamp(0.0, 0.999) * 255.0) as u32;
			let g = (color.y.sqrt().clamp(0.0, 0.999) * 255.0) as u32;
			let b = (color.z.sqrt().clamp(0.0, 0.999) * 255.0) as u32;
			let a = 255;
			let color = (a << 24) | (b << 16) | (g << 8) | r;

			let y = (IMAGE_HEIGHT - 1) - y;
			pixels[x + y * IMAGE_WIDTH] = color;
		}
	}
	eprintln!("Done.");

	let now = SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.unwrap();
	let path = format!("image_{:?}.png", now.as_millis());
	let path = CString::new(path).unwrap();

	let pixels = unsafe {
		from_raw_parts(
			pixels.as_ptr() as *const u8,
			pixels.len() * size_of::<u32>(),
		)
	};
	image_write::stbi_write_png(
		&path,
		IMAGE_WIDTH as i32,
		IMAGE_HEIGHT as i32,
		size_of::<u32>() as i32,
		pixels,
		(IMAGE_WIDTH * size_of::<u32>()) as i32,
	);
}
