
fn main() {
	let sdl_context = sdl2::init().expect("SDL initialization failed");
	let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");
	let width = 600;
	let height = 800;
	
	let sdl_context = sdl2::init().expect("SDL initialization failed");
	let mut tetris = Tetris::new();
	let mut timer = SystemTime::now();

	let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

	let grid_x = (width - TETRIS_HEIGHT as u32 * 10) as i32 / 2;
	let grid_y = (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2;

	let window = video_subsystem.window("Tetris", width, height)
		.position_centered()
		.build()
		.expect("Failed to create window");

	let mut canvas = window.into_canvas()
		.target_texture()
		.present_vsync()
		.build()
		.expect("Couldn't get window's canvas");

	let texture_creator: TextureCreator<_> = canvas.texture_creator();

	let grid = create_texture_rect(&mut canvas, &texture_creator, 0, 0, 0, 
			TETRIS_HEIGHT as u32 * 10,
			TETRIS_HEIGHT as u32 * 16).expect("Failed to create a texture");

	let border = create_texture_rect(&mut canvas, &texture_creator,
					255, 255, 255,
					TETRIS_HEIGHT as u32 * 10 + 20,
					TETRIS_HEIGHT as i32 * 16 + 20).expect("Failed to create a texture");

	macro_rules! texture {
		($r:expr, $g: expr, $b: expr => (
			create_texture_rect(&mut canvas,
				&texture_creator,
				$r, $g, $b,
				TETRIS_HEIGHT as 32,
				TETRIS_HEIGHT as u32).unwrap()
		)
	}

	let textures = [texture!(255, 69, 69), texture!(255, 220, 69),
									texture!(237, 150, 37), texture!(171, 99, 237), texture!(77, 149, 239), texture!(39, 218, 225), texture!(45, 216, 47)];

	let window = video_subsystem.window("Tetris", width, height)
			.position_centered()
			.build()
			.expect("Failed to create window");

	let mut canvas = window.into_canvas()
			.target_texture()
			.present_vsync()
			.build()
			.expect("Couldn't get window's canvas");

	let texture_creator = TextureCreator<_> = canvas.texture_creator();
}
