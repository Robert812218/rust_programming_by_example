impl Tetrimino {
	fn rotate(&mut self) {
		self.current_state += 1;
		if self.current_state as usize >= self.states.len() {
			self.current_state = 0;
		}
	}
}

fn test_postition(&self, game_map: &[Vec<u8>], tmp_state: usize, x: isize,
																									y: usize) -> bool {
	let mut tmp_state = self.current_state + 1;
	if tmp_state as usize >= self.states.len() {
		tmp_state = 0;
	}
	
	let x_pos = [0, -1, 1, -2, 2, -3];	

	for decal_y in 0..4 {
		for decal_x in 0..4 {
			let x = x + decal_x;
			if self.states[tmp_state][decal_y][decal_x as usize] != 0
					&&
						(y + decal_y >= game_map.len() ||
						x < 0 ||
						x as usize >=  game_map[y + decal_y][x as usize] != 0) {
							return false;
						}
		}
	}
	return true;
}


