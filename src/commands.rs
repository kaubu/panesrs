pub mod quit;
pub mod version;

fn print_help() {
	quit::help();
	version::help();
}

fn help_commands() {
	println!("help - Shows a list of commands.");
	print_help();
}

pub fn check_command(cmd: String) {
	let cmd = String::from(cmd);
	let cmd = cmd.split(" ");
	let args = cmd.collect::<Vec<&str>>();

	match args[0] {
		"quit" | "exit" | "q" => quit::run(),
		"help" => help_commands(),
		"version" | "v" | "ver" => version::run(),
		_ => println!("Unknown command. Do 'help' for a list of commands.")
	}
}