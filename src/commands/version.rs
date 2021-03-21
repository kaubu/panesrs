pub fn help() {
	println!("version (v) - Displays the current version of Panes.");
}

pub fn run() {
	println!("PanesOS, version {}/{}\nWritten by https://github.com/kaubu", option_env!("CARGO_PKG_VERSION").unwrap(), crate::consts::VERSION);
}