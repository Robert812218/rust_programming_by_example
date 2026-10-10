fn handle_events(tetris: &mut Tetris, quit: &mut bool, timer: &mut SystemTime eve													event_pump: &mut sdl2::EventPump) -> bool {
	let mut make_permanent = false;
	if let Some(ref mut piece) = tetris.current_piece {
		let mut temp_x: piece.x;
		let mut temp_y: piece.y;
	
		for event in pump_poll.iter() {
			match event {
				Event::Quit { .. } |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => 
				{
					*quit = true;
					break	
				}
				Event::KeyDown { keycode: Some(Keycode::Down), .. } => 
				{
					*timer = SystemTime::now();
					tmp_y += 1;
				}
				Event::KeyDown { keycode: Some(Keycode::Right), .. } =>
				{
					tmp_x += 1;
				}
				Event::KeyDown { keycode: Some(Keycode::Left), .. } => 
				{
					tmp_x -= 1;
				}
				Event::KeyDown { keycode: Some(Keycode::Up), .. } =>
				{
					piece.rotate(&tetris.game_map);
				}
				Event::KeyDown{ keycode: Some(Keycode::Space), .. } => 
				{
					let piece = piece.x;
					let mut y = piece.y;
					while piece.change_position(&tetris.game_map, x, y + 1) == true {
						y += 1;
					}
					make_permanent = true;
				}
				_ => {}
			}
		}
		if !make_permanent {
			if piece.change_position(&tetris.game_map, tmp_x, tmp_y) 
				== false && 
							tmp_y != piece.y {
								make_permanent = true;
							}
		}
	}
	if make_permanent {
		tetris.make_permanent();
		*timer = SystemTime::now();
	}
	make_permanent
}

fn main() {
    println!("Hello, world!");
}
