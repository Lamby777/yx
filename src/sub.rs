/*
* Subcommands for yx
*
* "Time to get more borrow checker errors!"
* - Dex, 1:32 AM, 12/30/2022
*/

use crate::{PathBuf, fs, Serialize, Deserialize};

pub fn create_index(path: PathBuf) {
	let ser = 1;

	// Make the file
	fs::write(path, "test");
}
