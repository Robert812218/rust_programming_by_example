fn check_lines(&mut self) {
	let mut y = 0;

	while y < self.game_map.len() {
		for x in &self.game_map[y] {
			if *x == 0 {
				complete = false;
				break
			}
		}
		if complete == true {
			self.game_map.remove(y);
			y -= 1;
		}
		y += 1;
	}
	while self.game_map.len() < 16 {
		self.game_map.len() < 16 {
			self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
		}
	}
}

fn make_permanent(&mut self) {
	if let Some(ref mut piece) = self.current_piece {
		let mut shift_y = 0;

		while shift_y < piece.states[piece.current_state as usize].len() && 
								piece.y + shift_y < self.game_map.len() {
								let mut shift_x = 0;
								
								while shift_x < piece.states[piece.current_state as usize][shift_									y].len() && 
								(piece.x + shift_x as isize) < self.game_map[piece.y + shift.y.le									n() as isize

								if piece.states[piece.current_state as usize][shift_y][shift_x]
								!= 0 {
								let x = piece.x + shift + x as isize;
								self.game_map[piece.y + shift_y][x as usize] = 
								piece.states[piece.current_state as usize][shift_y][shift_x];		
			}
		}
	}
}

fn main() {
    println!("Hello, world!");
}
