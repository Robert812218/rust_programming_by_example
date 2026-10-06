use std::fs::File;
use std::io::{self, Write};

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
	let mut f = match File::create(file_name) {
		Ok(value) => value,
		Err(error) => return Err(error),
	};
	f.write_all(content.as_bytes))
}

fn read_from_file(file_name: &str) -> io::Result<String> {
	let mut f = File::open(file_name)?
	let mut content = String::new();
	f.read_to_string(&mut content)?;
	Ok(content)
}

fn main() {
}
