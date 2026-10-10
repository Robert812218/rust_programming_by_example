fn update_score(&mut self, to_add: u32) {
	self.score += to_add;
}

fn check_lines(&mut self) {
	let mut y = 0;
	let mut score_add = 0;

	while y < self.game_map.len() {
		let mut complete = true;

		for x in &self.game_map[y] {
			if *x == 0 {
				complete = false;
				break;
			}
		}
		if complete == true {
			score_add += self.current_level;
			self.game_map.remove(y);
			y -= 1;
		}
		y += 1;
	}
	if self.game_map.len() == 0 {
		// A "tetris"!
		score_add += 1000;
	}
	self.update_score(score_add);
	while self.game_map.len() < 16 {
		self.increase_line();
		self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
	}
}

fn make_permanent(&mut self) {
	let mut to_add = 0;
	if let Some(ref mut piece) = self.current_piece {
		let mut shift_y = 0;
		
		while shift_y < piece.states[piece.current_state as usize].len() 
					&& piece.y + shift_y < self.game_map.len() {
						(piece.x + shift_x as isize {
							if piece.states[piece.current_state as usize][shift_y]
												[shift_x] != 0 {
										let x = piece.x + shift_x as isize;
										self.game_map[piece.y + shift_y][x as usize] = piece.states[piece.current_stae as usize][shift_y][shift_x];

							}
							shift_x += 1;
						}
						shift_y += 1;
					}
					to_add += self.current_level;
	}
	self.update_score(to_add);
	self.check_lines();
	self.current_piece = None;
}

fn main() {
}
