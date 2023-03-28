/*
* Subcommands for yx
*
* "Time to get more borrow checker errors!"
* - Dex, 1:32 AM, 12/30/2022
*/

use std::path::{Path, PathBuf};
use crate::{LINE_SEPARATOR, HashMap, YxIndexIter, cwd_index_path,
			fs, ProgramState, YxFileRecord, indoc, read, IDFC};
use pathdiff::diff_paths;
use path_absolutize::*;

// converts any path into a relative path based on the .yx_index location
pub fn path_relative_to_index<T: AsRef<Path>>(path: T) -> IDFC<PathBuf> {
	let current_index	= cwd_index_path();
	let cleaned_path	= path.as_ref().absolutize()?;

	dbg!(&current_index);
	dbg!(&cleaned_path);

	let res = diff_paths(cleaned_path, current_index).ok_or_else(
		|| "Failed to parse path as relative to index".into()
	);

	dbg!(&res);

	res
}

// given program state, write it to the index file to save information
pub fn write_to_index(path: PathBuf, state: ProgramState) {
	let ser = serde_json::to_string(&state).unwrap();

	// Make the file
	if let Err(_) = fs::write(&path, ser) {
		panic!("Failed to write to {:?}!", path);
	}
}

pub fn add_tag_to(state: &mut ProgramState, path: PathBuf, tag: &str) -> IDFC<()> {
	let tag = tag.to_string();

	println!("Pre");
	let path_rel = path_relative_to_index(&path)?;
	dbg!(&path_rel);

	state.index.entry(path_rel).and_modify(|record| {
		record.tags.insert(tag.to_owned());
	}).or_insert(
		YxFileRecord::new(tag.to_owned())
	);

	Ok(())
}

pub fn rm_tag_from(state: &mut ProgramState, path: PathBuf, tag: &str) -> IDFC<()> {
	// WILL NOT CHECK IF THE TAG IS THERE!
	// Use `file_has_tag` first if you need to know.
	let tag = tag.to_string();
	let path_rel = path_relative_to_index(&path)?;

	state.index.entry(path_rel).and_modify(|record| {
		record.tags.retain(|v| v.as_str() != tag)
	});

	Ok(())
}

pub fn file_has_tag(state: &ProgramState, path: PathBuf, tag: &str) -> IDFC<bool> {
	let path_rel = path_relative_to_index(&path)?;
	let record = state.index.get(&path_rel);

	if record.is_none() {
		panic!("File not in index!");
	}

	// shadow with unwrapped value
	let record = record.unwrap();

	Ok(record.tags.contains(&tag.to_string()))
}

pub mod fedit {
	use crate::{ProgramState, PathBuf, fs, YxFileRecord};

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
		get_tags_of_two_files_and_then(state, path_o, path_n, |o, n| {
			n.tags.extend(o.tags.clone());
		})
	}

	// Append to N from O, but remove said tags from O
	pub fn append_tags_rm_old(state: &mut ProgramState, path_o: PathBuf, path_n: PathBuf) {
		get_tags_of_two_files_and_then(state, path_o, path_n, |o, n| {
			n.tags.extend(o.tags.clone());
			o.tags.clear();
		})
	}

	fn get_tags_of_two_files_and_then(
		state:	&mut ProgramState,
		path_o:	PathBuf,
		path_n:	PathBuf,
		then:	fn(&mut YxFileRecord, &mut YxFileRecord) -> ()
	) {
		let index = &mut state.index;

		let got = index.get_many_mut([
			&path_o, &path_n
		]);

		if let Some(got) = got {
			let [rec_o, rec_n] = got;

			then(rec_o, rec_n);
		} else {
			println!("One of those 2 paths doesn't exist!");
		}
	}
}

pub fn confirm_purge(closest: &PathBuf) -> bool {
	println!( indoc! {"

		{}
		Are you sure? This will clear out every tag from the index!
		Just to be clear, you'll be clearing this index:
		{}
		(found closest to the current working directory)
		{}
	
		[Y/N]"}, LINE_SEPARATOR, closest.display(), LINE_SEPARATOR);
	
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

pub fn retrieve_where<C>(it: YxIndexIter, pred: C)
	-> YxIndexIter
	where C: Fn(&(PathBuf, YxFileRecord)) -> bool {
		
	let it = it.filter(pred);

	let temp = it.collect::<HashMap<_, _>>();
	temp.into_iter()
}

pub mod render {
	use crate::{fs, ProgramState};

use super::retrieve_where;

	pub fn copied(st: &ProgramState, rename: bool) {
		//
	}

	pub fn hardlinked(st: &ProgramState, rename: bool) {
		//
	}

	fn rename_to_its_tags() {
		//
	}
}
