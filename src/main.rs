// Panes main file
// Created by https://github.com/kaubu

use phf::phf_map;
use chrono::prelude::*;
use std::{io::{self, Write}};

mod password;
mod account;
mod file;

// OS Variables
pub const VERSION: &str = "0.1.0";
pub const AUTHOR: &str = "kaubu";
pub const SOURCE_CODE: &str = "https://github.com/kaubu";
pub const OS_NAME: &str = "Panes";
pub const CURSOR: &str = "> ";

// Directory paths
pub const ROOT_PATH: &str = "./";
pub const PANES_PATH: &str = "./panes/";

pub const USERS_PATH: &str = "./panes/users/";

// File paths
pub const ACCOUNT_DATABASE_PATH: &str = "./panes/account.psys";
pub const LOCAL_AUTHENTICATION_PATH: &str = "./panes/auth.psys";

// Files
pub const HIDDEN_FILE_TYPES: [&'static str; 4] = [ // Change length to number of items in array
	"psys", // Panes system file
	"fmtd", // File metadata file
	"dmtd", // Directory metadata file
	"hd" // User created hidden file
]; // File and directory metadata should be added to the file, in the future

// Check if the user is trying to create a file ending with this
pub const SYSTEM_ONLY_FILE_TYPES: [&'static str; 3] = ["psys", "fmtd", "dmtd"];

pub const FILE_ACCOUNT_PERMISSIONS: phf::Map<&'static str, i8> = phf_map!{
	// "SYSTEM" => -1,
	"NONE" => 0,
	"READ_ONLY" => 1,
	// "WRITE_ONLY" => 2,
	"FULL" => 2
};

pub const DEFAULT_USER_PERMISSIONS: phf::Map<&'static str, i8> = phf_map!{
	"USER" => 0, // NONE
	"GUEST" => 0
};

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
	let mut input = String::new();

	print!("{}", &message);
	io::stdout().flush().unwrap();
	
	io::stdin().read_line(&mut input).expect("system: Failed to read line");

	input.trim().to_owned()
}

fn create_panes_dir() {
	// Creates the panes directory in case it wasn't there already
	match file::create_dir(PANES_PATH.to_string()) {
		Ok(_) => {},
		Err(_) => {
			println!("system: Error when creating ./panes/ directory");
		}
	};
}

fn main() {
	create_panes_dir();
	// account::test();
	
	let current_directory: &str = "panes/";

	loop {
		let cursor = [current_directory, CURSOR].concat();
		let command: String = input(&cursor);
		println!("{}", command);
	}
}