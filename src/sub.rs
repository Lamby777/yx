/*
* Subcommands for yx
*
* "Time to get more borrow checker errors!"
* - Dex, 1:32 AM, 12/30/2022
*/

use crate::{PathBuf, fs, ProgramState, classes::YxFileRecord};

pub fn write_to_index(path: PathBuf, state: ProgramState) {
	let ser = serde_json::to_string(&state).unwrap();

	// Make the file
	if let Err(_) = fs::write(&path, ser) {
		panic!("Failed to write to {:?}!", path);
	}
}

pub fn add_tag_to(state: &mut ProgramState, path: PathBuf, tag: &str) {
	let mut record = YxFileRecord::new();
	record.tags.push(tag.to_string());

	state.index.entry(path).and_modify(|v|
		*v = record
	);
}
