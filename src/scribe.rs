/*
* Module for stuff in `yx scribe`
*/

use walkdir::WalkDir;
use crate::classes::ProgramStatePathed;

pub enum ScribeMethod<'a> {
	SplitBy(&'a str),
}

pub fn import_from_names(st: &mut ProgramStatePathed, method: ScribeMethod) {
	for entry in WalkDir::new(&st.path) {
		match entry {
			Err(e)	=> {
				println!("Error walking file in dir {:?}... {}", &st.path, e);
			}

			Ok(i)	=> {
				let fname = i.file_name().to_string_lossy();
				let tags = process_into_tags(fname.as_ref(), &method);
			}
		}
	}
}

fn process_into_tags<'a>(path: &'a str, method: &ScribeMethod) -> Vec<&'a str> {
	match method {
		ScribeMethod::SplitBy(delim)	=> {
			path.split(delim).collect()
		}
	}
}
