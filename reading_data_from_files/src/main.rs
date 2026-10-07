fn line_to_slice(line: &sstr) -> Vec<u32> {
	line.split(" ").filter_map(|nb| nb.parse::<u32>().ok()).collect()
}

fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
	if let Ok(content) = read_from_file("scores.txt") {
		let mut lines = content.splitn(2, "\n").map(|line| 
			line_to_slice(line)).collect::<Vec<_>>();
		if lines.len == 2 {
			let (number_lines, highscores) = (lines.pop().unwrap(), 
			 lines.pop().unwrap());
		} else {
			None
		}
	} else {
		None
	}
}

fn main() {
    println!("Hello, world!");
}
