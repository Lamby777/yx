/**
* Okay, I'm back to having no idea what I'm doing. <3
* - Dex
**/

use std::{fs, env};
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
//use text_io::read;

mod sub;

mod classes;
use classes::*;

/*
* Exit Codes
* 3 - (maybe add a prompt to create one later)
* 4 - Invalid command (help shown)
* 5 - Attempt to create an existing index
*/

const INDEX_FILE_NAME: &str = ".yx_index";

fn main() -> Result<(), ()> {
	// Get command line args
	let args: Vec<String> = env::args().collect();
	
	if args.len() < 2 { return Err(show_help()); }

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

			let mut st = load_state_unwrap();

			sub::add_tag_to(
				&mut st,
				(&args[0]).into(),
				&args[1]
			);

			sub::write_to_index(get_closest_index(), st)
		},

		"rm"		=> {
			assert_argc(args, &[2]);

			let mut st = load_state_unwrap();

			// If file doesn't have tag, yell at the user :P
			if !(sub::file_has_tag(&st, (&args[0]).into(), &args[1])) {
				panic!("File already has this tag!");
			}

			sub::rm_tag_from(
				&mut st,
				(&args[0]).into(),
				&args[1]
			);

			sub::write_to_index(get_closest_index(), st)
		},
		
		// dude has no clue what they're doing 💀
		_			=> show_help()
	}

	Ok(())
}

pub fn show_help() {
	println!(include_str!("help.txt"));
}

pub fn load_state_unwrap() -> ProgramState {
	let state = load_state();

	if let Err(_) = state {
		panic!("No {} in the current file structure!", INDEX_FILE_NAME);
	}

	state.unwrap()
}

pub fn load_state() -> Result<ProgramState, std::io::Error> {
	let res = fs::read_to_string(get_closest_index());

	match res {
		Ok(content) => Ok(serde_json::from_str(&content).unwrap()),

		Err(e) => Err(e),
	}
}

pub fn get_closest_index() -> PathBuf {
	get_all_current_indexes()[0].clone()
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
