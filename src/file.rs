use std::fs;

pub fn create_dir(path: String) -> std::io::Result<()> {
	fs::create_dir_all(path)?;
	Ok(())
}

pub fn read_file_to_string(path: String) -> String {
	fs::read_to_string(path).expect("system: Error reading file to string")
}

pub fn write_file(path: String, contents: String) {
	fs::write(path, contents).expect("system: Error writing to file");
}
