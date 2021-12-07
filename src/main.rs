use {
	stb::image_write,
	std::{
		ffi::CString,
		mem::size_of,
		slice::from_raw_parts,
		time::SystemTime,
	},
};

fn main() {
	const WIDTH: usize = 256;
	const HEIGHT: usize = 256;

	let mut pixels = vec![0; WIDTH * HEIGHT];

	for y in 0..HEIGHT {
		for x in 0..WIDTH {
			let r = (x as f64) / ((WIDTH - 1) as f64);
			let g = (y as f64) / ((HEIGHT - 1) as f64);
			let b = 0.25;

			let r = (r * 255.0) as u32;
			let g = (g * 255.0) as u32;
			let b = (b * 255.0) as u32;
			let a = 255;

			let color = (a << 24) | (b << 16) | (g << 8) | r;
			pixels[x + y * WIDTH] = color;
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
		WIDTH as i32,
		HEIGHT as i32,
		size_of::<u32>() as i32,
		pixels,
		(WIDTH * size_of::<u32>()) as i32,
	);
}
