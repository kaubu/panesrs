// password.rs by https://github.com/kaubu
// You can change the config as you see fit

use argon2::{self, Config, Variant, ThreadMode};
use ring::rand::{self, SecureRandom};
use data_encoding::HEXUPPER;

const SALT_LENGTH: usize = 32;

fn generate_salt() -> String {
	let rng = rand::SystemRandom::new();
	let mut salt = [0u8; SALT_LENGTH];

	rng.fill(&mut salt).unwrap();

	HEXUPPER.encode(&salt)
}

pub fn hash_password(password: String) -> String {
	let password = password.as_bytes();
	let default_config = Config::default();

	let config = Config {
		variant: Variant::Argon2id,
		version: default_config.version,
		mem_cost: default_config.mem_cost, // 102400 for release,
		time_cost: 4, // Amount of passes
		lanes: 4, // Amount of cores
		thread_mode: ThreadMode::Parallel,
		secret: default_config.secret, // Might change this later, idk
		ad: default_config.ad,
		hash_length: 32 // Length of hash
	};

	let salt = generate_salt();

	argon2::hash_encoded(password, &salt.as_bytes(), &config).unwrap() // Returns the hash
}

pub fn is_password(hash: String, password: String) -> bool {
	let password = password.as_bytes();

	argon2::verify_encoded(&hash, password).expect("Invalid hash or encoding fail") // Returns bool
}
