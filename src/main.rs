/**
* Okay, I'm back to having no idea what I'm doing. <3
* - Dex
**/

use std::{fs, env};
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

mod sub;

mod classes;
use classes::*;

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

			sub::create_index(path.join(".yx_index"), load_state());
		},
		
		// dude has no clue what they're doing 💀
		_			=> show_help()
	}
}

pub fn show_help() {
	println!(include_str!("help.txt"));
}

pub fn load_state() -> ProgramState {
	let res = fs::read_to_string(".yx_index");

	// make this check for .yx_index in all parent dirs later on
	match res {
		Ok(content) => {
			serde_json::from_str(&content).unwrap()
		},

		Err(_) => {
			panic!("No .yx_index in the current file structure!");
		}
	}
}
