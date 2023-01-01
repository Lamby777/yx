/**
* Okay, I'm back to having no idea what I'm doing. <3
* - Dex
**/

use std::{fs, env};
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use indoc::indoc;
//use text_io::read;

mod sub;

mod classes;
use classes::*;

const INDEX_FILE_NAME: &str	= ".yx_index";

// a shit ton of dashes to split up condensed data
const LINE_SEPARATOR: &str	= "--------------------------------------------------";

fn main() {
	// Get command line args
	let args: Vec<String> = env::args().collect();
	
	if args.len() < 2 { return show_help(); }

	let cmd = &args[1];		// give the cmd its own binding
	let args = &args[2..];	// shadow first vec

	match cmd.as_str() {
		"create"	=> {
			assert_argc(args, &[0, 1]);

			let path: PathBuf;
			
			if args.len() < 1 {
				// current working dir
				path = get_cwd().join(INDEX_FILE_NAME);
			} else {
				path = PathBuf::from(&args[0]);
			}

			if path.exists() {
				panic!("An index already exists here! Consider deleting it.");
			}

			sub::write_to_index(path, ProgramState::new());
		},

		"add"		=> {
			assert_argc(args, &[2]);

			let mut st = load_state();

			sub::add_tag_to(
				&mut st,
				(&args[0]).into(),
				&args[1]
			);

			sub::write_to_index(get_closest_index().unwrap(), st)
		},

		"rm"		=> {
			assert_argc(args, &[2]);

			let mut st = load_state();

			// If file doesn't have tag, yell at the user :P
			if !(sub::file_has_tag(&st, (&args[0]).into(), &args[1])) {
				panic!("File already has this tag!");
			}

			sub::rm_tag_from(
				&mut st,
				(&args[0]).into(),
				&args[1]
			);

			sub::write_to_index(get_closest_index().unwrap(), st)
		},

		"list"		=> {
			assert_argc(args, &[0, 1, 2]);
			let argc = args.len();

			let st = load_state();

			let it = st.index.into_iter();

			let it = match argc {
				1 => {
					// yx list <missing>
					match args[0].to_lowercase().as_str() {
						"missing" => {
							let it = it.filter(|v| v.0.exists());
							let temp = it.collect::<HashMap<_, _>>();
							temp.into_iter()
						},

						_ => {
							show_help();
							panic!("Invalid use of `yx list <arg>`");
						}
					}
				},

				2 => {
					// Q: Why not combine this and the _ case?
					// A: This is just a placeholder for the `yx list by <tag>` case.
					it
				},

				_ => {
					it
				}
			};

			if it.len() <= 0 {
				return println!( indoc! {"
					{}
					Index has no entries!
					Use `yx add <file> <tag>` to get started.
					{}
				"}, LINE_SEPARATOR, LINE_SEPARATOR);
			}

			
			println!("{}", LINE_SEPARATOR);

			// if there are records, print 'em out.
			for (path, record) in it {
				let tags = record.tags.join(", ");

				println!("{} >> {tags}", path.display());
				println!("{}\n", LINE_SEPARATOR);
			}
		},
		
		// dude has no clue what they're doing 💀
		_			=> show_help()
	}
}

pub fn show_help() {
	println!("{}\n{}{}\n", LINE_SEPARATOR, include_str!("help.txt"), LINE_SEPARATOR);
}

pub fn load_state() -> ProgramState {
	let index = get_closest_index();
	
	if index.is_none() {
		panic!("{} not found in current path!", INDEX_FILE_NAME);
	}

	let index = index.unwrap();

	let res = fs::read_to_string(index);

	match res {
		Ok(content) => serde_json::from_str(&content).unwrap(),

		Err(_) => {
			panic!("Error deserializing .yx_index!");
		},
	}
}

pub fn get_closest_index() -> Option<PathBuf> {
	let v = get_all_current_indexes();

	if v.len() <= 0 {
		None
	} else {
		Some(v[0].clone())
	}
}

pub fn get_all_current_indexes() -> Vec<PathBuf> {
	let mut dir = get_cwd();

	let mut paths: Vec<PathBuf> = vec![];

	// Loop goes backwards, so the result vec is in order
	// of inner to outer directories.
	loop {
		// check if index exists here
		let index_path = dir.join(INDEX_FILE_NAME);

		if index_path.exists() {
			// if it does, add it to res
			paths.push(index_path);
		}

		let repeat = dir.pop();

		if !repeat { break };
	}

	paths
}

pub fn cwd_index_path() -> PathBuf {
	// this just builds a string for convenience.
	// use get_closest_index() for finding what to write to
	get_cwd().join(INDEX_FILE_NAME)
}

pub fn get_cwd() -> PathBuf {
	env::current_dir().expect("Error getting current directory")
}

pub fn assert_argc(args: &[String], lens: &[usize]) {
	let len = args.len();

	let mapped: Vec<String> = lens.iter().map(|&id| id.to_string()).collect();
	let joined = mapped.join("|");

	if !lens.contains(&len) {
		panic!("This subcommand requires {} arguments, but you only gave {}!", joined, len);
	}
}
