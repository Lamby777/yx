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
	let tag = tag.to_string();

	state.index.entry(path).and_modify(|record|
		record.tags.push(tag.to_owned())
	).or_insert(
		YxFileRecord::new(tag.to_owned())
	);
}

pub fn rm_tag_from(state: &mut ProgramState, path: PathBuf, tag: &str) {
	// WILL NOT CHECK IF THE TAG IS THERE!
	// Use `file_has_tag` first if you need to know.

	let tag = tag.to_string();

	state.index.entry(path).and_modify(|record| {
		record.tags.retain(|v| v.as_str() != tag)
	});
}

pub fn file_has_tag(state: &ProgramState, path: PathBuf, tag: &str) -> bool {
	let record = state.index.get(&path);

	if record.is_none() {
		panic!("File not in index!");
	}

	// shadow with unwrapped value
	let record = record.unwrap();

	record.tags.contains(&tag.to_string())
}

pub fn move_file_and_tags(state: &mut ProgramState, path_o: PathBuf, path_n: PathBuf) {
	// move the file...
	if let Err(_) = fs::rename(&path_o, &path_n) {
		panic!("There was a problem moving the file. Your tags were not affected.")
	}

	// then move tags
	move_tags(state, path_o, path_n);
}

pub fn move_tags(state: &mut ProgramState, path_o: PathBuf, path_n: PathBuf) {
	let index = &mut state.index;

	let val = index.remove(&path_o);

	if let Some(val) = val {
		index.insert(path_n, val);
	} else {
		println!("Failed to move tag {}; it doesn't exist!", path_o.display());
	}
}
