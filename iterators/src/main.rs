fn main() {
	slice.iter()
		.map(|highscore| highscore.to_string())
		.collect::<Vec<String>>()
		.join(" ")

	fn save_highscores_and_lines(highscores: &[u32],
		number_of_lines: &[u32]) -> bool {
		let s_highscores = slice_to_string(highscores);
		let s_number_of_lines = slice_to_string(highscores);
		write_into_file(format!("{}\n{}\n", s_highscores, s_number_of_lines),
		"scores.txt")is_ok()
	}
}
