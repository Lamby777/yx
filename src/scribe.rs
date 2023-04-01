/*
* Module for stuff in `yx scribe`
*/

use std::path::Path;
use walkdir::WalkDir;
use crate::{classes::{ProgramState}, sub::add_tags_to, IDFC};

pub enum ScribeMethod<'a> {
	SplitBy(&'a str),
}

pub fn import_from_names(
	st: &mut ProgramState,
	target: impl AsRef<Path>,
	method: ScribeMethod
) -> IDFC<()> {
	for entry in WalkDir::new(&target) {
		match entry {
			Err(e)	=> {
				println!("Error walking file in dir {:?}... {}", target.as_ref(), e);
			}

			Ok(i)	=> {
				if i.path() == target.as_ref() { continue; }

				let fname = i.path().file_stem().ok_or_else(
					|| "No file name"
				)?.to_string_lossy();

				let tags = process_into_tags(fname.as_ref(), &method);
				dbg!(&fname, &tags);

				add_tags_to(st, target.as_ref(), &tags)?;
			}
		}
	}

	Ok(())
}

fn process_into_tags<'a>(path: &'a str, method: &ScribeMethod) -> Vec<&'a str> {
	match method {
		ScribeMethod::SplitBy(delim)	=> {
			path.split(delim).collect()
		}
	}
}
