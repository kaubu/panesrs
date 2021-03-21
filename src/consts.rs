use phf::phf_map;

// OS Variables
pub const VERSION: &str = "0.2.1";
pub const AUTHOR: &str = "kaubu";
pub const SOURCE_CODE: &str = "https://github.com/kaubu";
pub const OS_NAME: &str = "PanesOS";
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
];

pub const HIDDEN_FILE_TYPES_GREP: &str = "!(*.psys|*.fmtd|*.dmtd|*.hd)";

// File and directory metadata should be added to the file, in the future

// Check if the user is trying to create a file ending with this
pub const SYSTEM_ONLY_FILE_TYPES: [&'static str; 3] = [
	"psys",
	"fmtd",
	"dmtd"
];

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