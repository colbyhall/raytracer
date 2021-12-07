#![feature(const_fn_floating_point_arithmetic)]

mod math;
mod world;

use {
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
	if depth <= 0 {
		// If we've exceeded the ray bounce limit, no more light is gathered
		return Color::new(0.0, 0.0, 0.0);
	}

	if let Some(hit) = world.hit(ray, 0.001, Float::INFINITY) {
		let target = hit.impact + hit.normal + Vec3::rand_in_unit_sphere();
		let ray = Ray::new(hit.impact, target - hit.impact);
		0.5 * ray_color(&ray, world, depth - 1)
	} else {
		let direction = ray.direction.norm();
		let t = 0.5 * (direction.z + 1.0);
		(1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
	}
}

fn main() {
	// Image Information
	const ASPECT_RATIO: Float = 16.0 / 9.0;
	const IMAGE_WIDTH: usize = 1280;
	const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as Float) / ASPECT_RATIO) as usize;
	const SAMPLES_PER_PIXEL: usize = 10;
	const MAX_DEPTH: usize = 50;

	// Camera Information
	let viewport_height = 2.0;
	let viewport_width = viewport_height * ASPECT_RATIO;
	let focal_length = 1.0;

	let origin = Point3::ZERO;
	let horizontal = viewport_width * Vec3::RIGHT;
	let vertical = viewport_height * Vec3::UP;
	let bottom_left = origin - horizontal / 2.0 - vertical / 2.0 - focal_length * Vec3::FORWARD;

	// World data
	let mut world = World::new();
	world.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, 0.0), 0.5)));
	world.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -100.5), 100.0)));

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

				let direction = bottom_left + u * horizontal + v * vertical - origin;
				let ray = Ray::new(origin, direction);
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
