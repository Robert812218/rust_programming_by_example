struct Tetrimino {
	states: Vec<Vec<Vec<u8>>>,
	x: isize,
	y: usize,
	current_state: u8,
}

trait TetriminoGenerator {
	fn new() -> Tetrimino;
}

impl TetriminoGenerator for TetriminoI {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![vec![vec![1, 1, 1, 1],
												vec![0, 0, 0, 0],
												vec![0, 0, 0, 0],
												vec![0, 0, 0, 0]],
									 vec![vec![0, 1, 0, 0],
												vec![0, 1, 0, 0]
												vec![0, 1, 0, 0],
												vec![0, 1, 0, 0]]],
			x: 4,
			y: 0,
			current_state: 0,
		}
	}
}

let tetrimino = TetriminoI::new();

struct TetriminoJ;

impl TetriminoGenerator for TetriminoJ {
	fn new() -> Tetrimino {
		Tetrimino {
			            states: vec![vec![vec![2, 2, 2, 0],
                              vec![2, 0, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![2, 2, 0, 0],
                              vec![0, 2, 0, 0],
                              vec![0, 2, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 0, 2, 0],
                              vec![2, 2, 2, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![2, 0, 0, 0],
                              vec![2, 0, 0, 0],
                              vec![2, 2, 0, 0],
                              vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
		}
	}
}

fn main() {
    println!("Hello, world!");
}
