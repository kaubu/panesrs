// Panes main file
// Created by https://github.com/kaubu

use chrono::prelude::*;
use std::{
	collections::HashMap,
	fs,
	io::{self, Write},
	path::PathBuf,
	// thread::current,
	process::Command,
};

mod password;
mod account;
mod file;
mod commands;
mod consts;

// Functions
pub fn get_date_and_time_now() -> DateTime<Local> {
	Local::now()
}

pub fn get_date_and_time_now_fmt() -> String {
	Local::now().to_string()
}

pub fn clear_screen() {
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn input(message: &str) -> String {
	let mut input: String = String::new();

	print!("{}", &message);
	io::stdout().flush().unwrap();
	
	io::stdin().read_line(&mut input).expect("system: Failed to read line");

	input.trim().to_owned()
}

fn create_panes_dir() {
	// Creates the panes directory in case it wasn't there already
	match file::create_dir(consts::PANES_PATH.to_string()) {
		Ok(_) => {},
		Err(_) => {
			println!("system: Error when creating ./panes/ directory");
		}
	};

	match file::create_dir(consts::USERS_PATH.to_string()) {
		Ok(_) => {},
		Err(_) => {
			println!("system: Error when creating ./panes/users/ directory");
		}
	};
}

fn get_absolute_path(path: &str) -> String {
	let srcdir = PathBuf::from(path);
	format!("{:?}", fs::canonicalize(&srcdir).unwrap())
}

fn prelude_authentication() -> (String, account::AccountDatabase) { // HashMap<String, account::AccountDatabase>
	let current_user: String;
	
	if file::check_file_or_dir_exists(consts::ACCOUNT_DATABASE_PATH) {
		// Load database if it exists
		let mut account_database: account::AccountDatabase = account::AccountDatabase::load(
			account::load_accounts(String::from(consts::ACCOUNT_DATABASE_PATH)),
			consts::ACCOUNT_DATABASE_PATH
		);

		println!("Log into an account.");
		// Log in loop
		loop {
			let username_attempt: String = input("Username: ");
			let password_attempt: String = input("Password: ");

			match account_database.verify_login(username_attempt.clone(), password_attempt) {
				true => {
					println!("Welcome {}!", account_database.get_account(&username_attempt).unwrap().username);
					current_user = username_attempt.clone();
					break
				},
				false => continue,
			}
		}
		// let mut return_users = HashMap::new();
		// return_users.insert(current_user, account_database);
		// return_users
		(current_user, account_database)
		// TODO, if the user folder isn't already created, create it
	} else {
		// Initial setup IF /panes/account.psys doesn't exist
		println!("Create an account.");
		let current_user: String = input("Username: ");
		let password: String = input("Password: ");

		let new_acc: account::Account = account::Account::new(
			current_user.clone(),
			password,
			account::AccountType::Admin
		);

		let mut account_database: account::AccountDatabase = account::AccountDatabase{
			accounts: HashMap::new(),
			path: consts::ACCOUNT_DATABASE_PATH,
		};

		account_database.add(new_acc);
		account_database.save_database();
		// let mut return_users = HashMap::new();
		// return_users.insert(current_user, account_database);
		// return_users
		(current_user, account_database)
		// TODO, create user folder with the name of the created username
	}
}

fn prelude() {
	// Set the title of the window like OG PanesOS, but only on windows, since there isn't a command for linux that was build-in
	if cfg!(target_os = "windows") {
		Command::new("cmd")
			.args(&["/C", "title", consts::OS_NAME, consts::VERSION])
			.output()
			.expect("system: Failed to change window title");
	}

	// Create folders needed
	create_panes_dir();
}

fn main() {
	prelude(); // Do all of the setting up before the main loop
	let (current_user, account_database) = prelude_authentication();
	
	let display_directory: &str = "panes/";
	let relative_directory: &str = "\\panes\\";
	let absolute_directory: String = get_absolute_path("./panes/"); // True directory, that will be appended to the current one

	// Main command checking loop
	loop {
		let cursor: String = format!("{}: {}", current_user, [display_directory, consts::CURSOR].concat());
		let command: String = input(&cursor);

		commands::check_command(command);
	}
}
