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

	if !handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump {
		if let Some(ref mut piece) = tetris.current_piece {
			for (line_nb, line) in piece.states[piece.current_state as usize].iter().enumerate() {
					for (case_nb, case) in line.iter().enumerate() {
								if *case == 0 {
									canvas.copy(&textures[*case as usize - 1],
										None,
										Rect::new(grid_x + (piece.x + case_nb as isize) as i32 * TETRIS_HEIGHT as i32, grid_y + (piece.y + line_nb) as i32 * TETRIS_HEIGHT as i32, TETRIS_HEIGHT as i32, TETRIS_HEIGHT as i32))
							.expect("Couldn't copy texture into window");
								}
							}
						}
				}
		}
	}
	for (line_nb, line) in tetris.game_map.iter().enumerate() {
		if *case == 0 {
			continue
		}
		canvas.copy(&textures[*case as usize - 1],
				None, Rect::new(grid_x + case_nb as i32 * TETRIS_HEIGHT as i32, grid_y + line_nb as i32 * TETRIS_HEIGHT as i32, TETRIS_HEIGHT as u32 TETRIS_HEIGHT as u32))
				.expect("Couldn't copy texture into window");
	}
	canvas.present();
}

