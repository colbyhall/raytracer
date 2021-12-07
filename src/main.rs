#![feature(const_fn_floating_point_arithmetic)]

mod math;

use {
	math::*,
	stb::image_write,
	std::{
		ffi::CString,
		mem::size_of,
		slice::from_raw_parts,
		time::SystemTime,
	},
};

fn hit_sphere(center: Point3, radius: Float, ray: &Ray) -> bool {
	let oc = ray.origin - center;
	let a = ray.direction.len_sq();
	let b = oc.dot(ray.direction);
	let c = oc.len_sq() - radius * radius;
	let d = b * b - a * c;
	d > 0.0
}

fn ray_color(ray: &Ray) -> Color {
	if hit_sphere(Vec3::FORWARD * -1.0, 0.5, ray) {
		return Color::new(1.0, 0.0, 0.0);
	}

	let direction = ray.direction.norm();
	let t = 0.5 * (direction.z + 1.0);
	(1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
	// Image Information
	const ASPECT_RATIO: Float = 16.0 / 9.0;
	const IMAGE_WIDTH: usize = 1280;
	const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as Float) / ASPECT_RATIO) as usize;

	// Camera Information
	let viewport_height = 2.0;
	let viewport_width = viewport_height * ASPECT_RATIO;
	let focal_length = 1.0;

	let origin = Point3::ZERO;
	let horizontal = viewport_width * Vec3::RIGHT;
	let vertical = viewport_height * Vec3::UP;
	let bottom_left = origin - horizontal / 2.0 - vertical / 2.0 - focal_length * Vec3::FORWARD;

	let mut pixels = vec![0; IMAGE_WIDTH * IMAGE_HEIGHT];
	for y in 0..IMAGE_HEIGHT {
		for x in 0..IMAGE_WIDTH {
			let u = (x as Float) / ((IMAGE_WIDTH - 1) as Float);
			let v = (y as Float) / ((IMAGE_HEIGHT - 1) as Float);

			let direction = bottom_left + u * horizontal + v * vertical - origin;
			let ray = Ray::new(origin, direction);
			let color = ray_color(&ray);

			let r = (color.x * 255.0) as u32;
			let g = (color.y * 255.0) as u32;
			let b = (color.z * 255.0) as u32;
			let a = 255;
			let color = (a << 24) | (b << 16) | (g << 8) | r;

			let y = (IMAGE_HEIGHT - 1) - y;
			pixels[x + y * IMAGE_WIDTH] = color;
		}
	}

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
