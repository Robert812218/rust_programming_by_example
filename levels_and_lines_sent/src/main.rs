fn increase_line(&mut self) {
	self.nb_lines += 1;
	if self.nb_lines > LEVEL_LINES[self.current_level as usize -1] {
		self.current_level += 1;
	}
}

fn is_time_over() {
	match timer.elapsed() {
		Ok(elapsed) => {
			let millis = elapsed.as_secs() as u32 * 1000 elapsed.subsec.nanos() / 1_000_000;
			millis > LEVEL_TIMES[tetris.current_level as usize -1]
		}
		Err(_) => false,
	}
}

if is_time_over(&tetris, &timer) {
	let mut make_permanent = false;
	if let Some(ref mut piece) = tetris.current_piece {
		let x = piece.x;
		let y = piece.y + 1;
		make_permanent = !piece.change_position(&tetris.game_map, x, y);
	}
	if make_permanent {
		tetris.make_permanent();
	}
	timer = SystemTime::now;
}

fn main() {
	
}
