/*
* Subcommands for yx
*
* "Time to get more borrow checker errors!"
* - Dex, 1:32 AM, 12/30/2022
*/

use std::collections::{HashMap, hash_map::IntoIter};

use crate::{PathBuf, fs, ProgramState, classes::YxFileRecord, indoc, read};

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

pub fn copy_file_and_tags(state: &mut ProgramState, path_o: PathBuf, path_n: PathBuf) {
	// copy the file...
	if let Err(_) = fs::copy(&path_o, &path_n) {
		panic!("There was a problem copying the file. Your tags were not affected.")
	}

	// then copy tags
	copy_tags(state, path_o, path_n);
}

// Overwrite N's tags with O's
pub fn copy_tags(state: &mut ProgramState, path_o: PathBuf, path_n: PathBuf) {
	let index = &mut state.index;

	let val = index.get(&path_o);

	if let Some(val) = val {
		index.insert(path_n, (*val).clone());
	} else {
		println!("Failed to copy from tag {}; it doesn't exist!", path_o.display());
	}
}

// Let N keep its tags, but add any O has that N doesn't
pub fn append_tags(state: &mut ProgramState, path_o: PathBuf, path_n: PathBuf) {
	let index = &mut state.index;

	let got = index.get_many_mut([
		&path_o, &path_n
	]);

	if let Some(got) = got {
		let [rec_o, rec_n] = got;

		rec_n.tags.extend(rec_o.tags.clone());

		rec_n.tags.dedup();
	} else {
		println!("One of those 2 files doesn't exist!");
	}
}

// Append to N from O, but remove said tags from O
pub fn append_tags_rm_old(state: &mut ProgramState, path_o: PathBuf, path_n: PathBuf) {
	//
}

pub fn confirm_purge(closest: &PathBuf) -> bool {
	println!( indoc! {"
		Are you sure? This will clear out every tag from the index!
		Just to be clear, you'll be clearing this index:
		{}
		(found closest to the current working directory)
	
	[Y/N]"}, closest.display());
	
	let res: char;

	loop {
		let res_attempt: String = read!();
		let res_attempt = res_attempt.to_lowercase().chars().nth(0);

		if let Some(res_n) = res_attempt {
			res = res_n;
			break
		} else {
			println!("Really? Come on! Type something!");
		}
	}

	res == 'y'
}

pub fn retrieve_where<C>(it: IntoIter<PathBuf, YxFileRecord>, pred: C)
	-> IntoIter<PathBuf, YxFileRecord>
	where C: Fn(&(PathBuf, YxFileRecord)) -> bool {
		
	let it = it.filter(pred);

	let temp = it.collect::<HashMap<_, _>>();
	temp.into_iter()
}
