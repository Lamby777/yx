/**
* Okay, I'm back to having no idea what I'm doing. <3
* - Dex
**/

use std::{fs, env};
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::process::exit;
use text_io::read;

mod sub;

mod classes;
use classes::*;

/*
* Exit Codes
* 3 - .yx_index missing when required (maybe add a prompt to create one later)
* 4 - Invalid command (help shown)
* 5 - Attempt to create an existing index
*/

fn main() {
	// Get command line args
	let args: Vec<String> = env::args().collect();
	
	if args.len() < 2 { return show_help(); }

	let cmd = &args[1];		// give the cmd its own binding
	let args = &args[2..];	// shadow first vec

	match cmd.as_str() {
		"create"	=> {
			let path: PathBuf;
			
			if args.len() < 1 {
				// current working dir
				let cwd = env::current_dir();

				match cwd {
					Ok(val) => {
						path = PathBuf::from(val);
					},

					Err(_) => {
						return println!("Error getting current directory");
					}
				}
			} else {
				path = PathBuf::from(&args[0]);
			}

			// shadow old path with appended version
			let path = path.join(".yx_index");

			if path.exists() {
				println!("An index already exists here! Consider deleting it.");
				exit(5);
			}

			sub::create_index(path, ProgramState::new());
		},

		"help"		=> {
			show_help_no_exit();
			exit(0);
		},
		
		// dude has no clue what they're doing 💀
		_			=> show_help()
	}
}

pub fn show_help() {
	show_help_no_exit();
	exit(4);
}

pub fn show_help_no_exit() {
	println!(include_str!("help.txt"));
}

pub fn load_state() -> Result<ProgramState, std::io::Error> {
	let res = fs::read_to_string(".yx_index");

	// make this check for .yx_index in all parent dirs later on
	match res {
		Ok(content) => Ok(serde_json::from_str(&content).unwrap()),

		Err(e) => Err(e),
//			println!("No .yx_index in the current file structure!");
//			exit(3);
	}
}
