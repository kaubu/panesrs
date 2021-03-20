pub mod quit;
pub mod version;

fn help_commands() {
	println!("help - Shows a list of commands.");
	quit::help();
	version::help();
}

pub fn check_command(cmd: String) {
	let cmd = String::from(cmd);
	let cmd = cmd.split(" ");
	let command_vector = cmd.collect::<Vec<&str>>();

	match command_vector[0] {
		"quit" | "exit" => quit::run(),
		"help" => help_commands(),
		"version" | "v" => version::run(),
		_ => println!("Unknown command. Do 'help' for a list of commands.")
	}
}