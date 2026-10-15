fn main() {
	canvas.set_draw_color(Color::RGB(255, 0, 0));
	canvas.clear();
	canvas.copy(&border,
		None,
		Rect::new((width - TETRIS_HEIGHT as u32 * 10) as i32 / 2 - 10, (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2 - 10,
		TETRIS_HEIGHT as u32 * 10 + 20, TETRIS_HEIGHT as u32 * 16 + 20))
				.expect("Couldn't copy texture into window);
			canvas.copy(&grid, 
						None,
						Rect::new((width - TETRIS_HEIGHT as u32 * 10) as i32 / 2,
			(height - TETRIS_HEIGHT as u32 * 16) as i32 /2,
			TETRIS_HEIGHT as u32 * 10, TETRIS_HEIGHT as u32 * 16))
				.expect("Couldn't copy texture into window);
}
