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

			sub::create_index(path.join(".yx_index"));
		},
		
		// dude has no clue what they're doing 💀
		_			=> show_help()
	}
}

fn show_help() {
	println!(include_str!("help.txt"));
}
