use crate::{
	ACCOUNT_DATABASE_PATH,
	password::{self, is_password}
};

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
	pub username: String,
	pub password: String,
	created: String,
	username_modified: Vec<String>,
	password_modified: Vec<String>,
	pub account_type: AccountType,
}

impl Account {
	pub fn new(
		new_username: String, 
		new_password: String, 
		new_account_type: AccountType
	) -> Account {
		Account {
			username: new_username,
			password: password::hash_password(new_password),
			created: crate::get_date_and_time_now_fmt(),
			username_modified: vec![String::from("NEVER")],
			password_modified: vec![String::from("NEVER")],
			account_type: new_account_type
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
	System,
	Admin,
	User,
	Guest
}

pub struct AccountDatabase {
	pub accounts: HashMap<String, Account>,
	pub path: &'static str
}

impl AccountDatabase {
	pub fn load(load_accounts: HashMap<String, Account>, load_path: &'static str) -> AccountDatabase {
		AccountDatabase {
			accounts: load_accounts,
			path: load_path
		}
	}
	
	pub fn add(&mut self, account: Account) {
		self.accounts.insert(
			account.username.clone(),
			account
		);
	}

	pub fn remove(&mut self, username: String) {
		self.accounts.remove(&username);
	}

	pub fn change_username(&mut self, old_username: String, new_username: String) -> bool {
		// Check if the old username doesn't exist
		if let false = self.accounts.contains_key(&old_username) {
			println!("system: The current username doesn't exist");
			return false;
		};
		
		// Check if the new username already exists
		if let true = self.accounts.contains_key(&new_username) {
			println!("system: The new username already exists");
			return false;
		};

		let mut account: Account = self.get_account(&old_username).unwrap();
		self.remove(old_username);

		account.username = new_username;
		account.username_modified.push(crate::get_date_and_time_now_fmt());
		self.add(account);

		true
	}

	pub fn change_password(&mut self, username: String, new_password: String) -> bool {
		// Check if the username doesn't exist
		if let false = self.accounts.contains_key(&username) {
			println!("system: The current username doesn't exist");
			return false;
		};

		let mut account: Account = self.get_account(&username).unwrap();
		self.remove(username);

		account.password = password::hash_password(new_password);
		account.password_modified.push(crate::get_date_and_time_now_fmt());
		self.add(account);

		true
	}

	// Don't forget to .unwrap() the Result
	pub fn get_account(&mut self, username: &String) -> core::result::Result<Account, bool> {
		// Check if the username doesn't exist
		if let false = self.accounts.contains_key(username) {
			println!("system: The current username doesn't exist");
			return Err(false);
		};

		Ok(self.accounts[username].clone())
	}

	// Change to save to a file
	pub fn save_database(&mut self) {
		let accounts = serde_json::to_string(&self.accounts).unwrap();
		crate::file::write_file(ACCOUNT_DATABASE_PATH.to_string(), accounts);
	}

	pub fn verify_login(&mut self, username: String, password: String) -> bool {
		// Check if the username doesn't exist
		if let false = self.accounts.contains_key(&username) {
			// Use this so as not to give any hints away
			println!("system: The username or password is incorrect");
			return false;
		};

		let account: Account = self.get_account(&username).unwrap();
		match is_password(account.password, password) {
			true => true,
			false => {
				println!("system: The username or password is incorrect");
				false
			}
		}
	}
}

pub fn load_accounts(path: String) -> HashMap<String, Account> {
	serde_json::from_str(&crate::file::read_file_to_string(path.to_string())).unwrap()
}
