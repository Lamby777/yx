use std::{fs, ffi::{OsStr, OsString}, path::Path};
use crate::{ProgramState, classes::{YxRenderMethod, YxRenderOptions, IDFC}, get_closest_index};
use walkdir::WalkDir;

pub fn render(st: &ProgramState, options: YxRenderOptions) -> IDFC<()> {
	let files =
		st.index.clone().into_iter()
			.filter(|v| {
				let fpath = get_closest_index().unwrap().join(&v.0);
				fpath.exists()
			});

	for (path, data) in files {
		match &options.method {
			YxRenderMethod::Copy		=> {
				// Get old name
				let old_name = path.file_name().ok_or_else(
					|| "Error reading file names"
				)?.to_owned();

				// If not renaming, use old name, else change it
				let new_path = if !options.rename {
					old_name
				} else {
					todo!()
				};
				
				let res = fs::copy(path, new_path);

				if let Err(e) = res {
					println!("Failed to render ")
				}
			},

			YxRenderMethod::Hardlink	=> {
				todo!()
			},
		}
	}

	Ok(())
}

fn change_folder_name(path: &Path) -> OsString {
/*	let mut segments: Vec<&OsStr> = path.iter().collect();
	// index of second to last element
	let stli = segments.len() - 2;

	let old_folder_name =
		segments[stli].to_string_lossy() + ".yx-render";
	let new_folder_name = old_folder_name.as_ref();

	segments[stli] = OsStr::new(new_folder_name);

	segments.join(OsStr::new(""))*/
	todo!()
}

fn rename_to_tags() {
	//
}
