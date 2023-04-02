/*
* Module for stuff in `yx scribe`
*/

use std::{path::{Path, PathBuf}, collections::HashSet};
use walkdir::WalkDir;
use crate::{classes::{ProgramState}, sub::{add_tags_to, path_relative_to_index}, IDFC};

pub enum ScribeMethod<'a> {
	SplitBy(&'a str),
}

pub fn import_from_names(
	st: &mut ProgramState,
	target: impl AsRef<Path>,
	method: ScribeMethod
) -> IDFC<()> {
	let ignores_list = &st.ignores.clone();

	let walker =
		WalkDir::new(&target).into_iter()
		.filter_map(|f| {
			if let Err(ref e) = f {
				println!("Error loading a file... {}", e);
			}

			f.ok().and_then(|e| {
				if !check_ignored(ignores_list, e.path()).ok()? {
					Some(e)
				} else {
					None
				}
			})
		});
	
	for entry in walker {
		if !entry.file_type().is_file() { continue; }

		let fname = entry.path().file_stem().ok_or_else(
			|| "No file name"
		)?.to_string_lossy();

		let tags = process_into_tags(fname.as_ref(), &method);

		println!("Adding tags {:?} for file {}", &tags, entry.path().to_string_lossy());
		add_tags_to(st, entry.path(), &tags)?;
	}

	Ok(())
}

fn check_ignored(ignores: &HashSet<PathBuf>, e: &Path) -> IDFC<bool> {
	let rel: &Path = &path_relative_to_index(e)?;
	let should_ignore = ignores.iter().any(|ig| {
		rel.starts_with(ig)
	});

	Ok(should_ignore)
}

fn process_into_tags<'a>(path: &'a str, method: &ScribeMethod) -> Vec<&'a str> {
	match method {
		ScribeMethod::SplitBy(delim)	=> {
			path.split(delim).collect()
		}
	}
}
