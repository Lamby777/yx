/*
* Subcommands for yx
*
* "Time to get more borrow checker errors!"
* - Dex, 1:32 AM, 12/30/2022
*/

use std::{fs, path::{Path, PathBuf}};
use crate::{HashMap, YxIndexIter, get_closest_index,
			ProgramState, YxFileRecord, IDFC, classes::YxTag};
use pathdiff::diff_paths;
use path_absolutize::*;

/// Converts any path into a relative path based on the .yx_index location
pub fn path_relative_to_index(path: impl AsRef<Path>) -> IDFC<PathBuf> {
	let mut current_index	= get_closest_index().unwrap();
	current_index.pop();
	
	let cleaned_path	= path.as_ref().absolutize()?;
	let fname = cleaned_path.file_name().ok_or_else(
		|| "Could not get file name"
	)?;
	
	let cleaned_path_parent = cleaned_path.parent().ok_or_else(
		|| "Could not get path parent"
	)?;

	let mut res =
		diff_paths(&cleaned_path_parent, &current_index).ok_or_else(
			|| "Failed to parse path as relative to index"
		)?;
	
	res.push(fname);

	Ok(res)
}

/// given program state, write it to the index file to save information
pub fn write_to_index(path: &Path, state: &ProgramState) -> IDFC<()> {
	let ser = serde_json::to_string(&state).unwrap();

	// Make the file
	fs::write(&path, ser).map_err(|e| {
		println!("Failed to write to {:?}!", path);
		e.into()
	})
}

pub fn get_all_tags_from(state: &ProgramState) -> IDFC<Vec<YxTag>> {
	let index = &state.index;

	let v: Vec<String> = vec![];

	let unique = index.into_iter().fold(
		v,
		|vec, (_, v)| {
			let mut closure_res = vec.clone();

			closure_res.extend(
				v.tags.iter().filter(|t| !vec.contains(t)).cloned()
			);

			closure_res
		}
	);

	Ok(unique)
}

/// Actually really different from the non-sorted version!
/// Make sure you're using the right one!
pub fn get_tag_counts_from(state: &ProgramState) -> IDFC<HashMap<String, u32>> {
	let index = &state.index;

	let mut totals: HashMap<String, u32> = HashMap::new();
	for (_, YxFileRecord { tags }) in index {
		for tag in tags {
			*totals.entry(tag.to_string()).or_insert(0) += 1;
		}
	}

	Ok(totals)
}

pub fn add_tags_to(state: &mut ProgramState, path: &Path, tags: &[&str]) -> IDFC<()> {
	let path_rel = path_relative_to_index(&path)?;

	
	let mapped = tags.iter()
		.map(|s| s.to_string());

	state.index.entry(path_rel).and_modify(|record| {
		record.tags.extend(mapped.clone());
	}).or_insert({
		YxFileRecord::new(
			&mapped.collect::<Vec<String>>()
		)
	});

	Ok(())
}

pub fn rm_tags_from(state: &mut ProgramState, path: &Path, tags: &[&str]) -> IDFC<()> {
	// WILL NOT CHECK IF THE TAG IS THERE!
	// Use `file_has_tag` first if you need to know.
	let path_rel = path_relative_to_index(&path)?;

	state.index.entry(path_rel).and_modify(|record| {
		record.tags.retain(|v| !(tags.contains(&v.as_str())));
	});

	Ok(())
}

pub fn file_has_tag(state: &ProgramState, path: PathBuf, tag: &str) -> IDFC<bool> {
	let path_rel = path_relative_to_index(&path)?;
	let record = state.index.get(&path_rel).ok_or_else(
		|| "File not in index!"
	)?;

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

/// Get all files matching predicate
pub fn retrieve_where<C>(it: YxIndexIter, pred: C) -> YxIndexIter
	where C: Fn(&(PathBuf, YxFileRecord)) -> bool
{
		
	let it = it.filter(pred);

	let temp = it.collect::<HashMap<_, _>>();
	temp.into_iter()
}
