/**
* Okay, I'm back to having no idea what I'm doing. <3
* - Dex
**/

use std::env;
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
		"create"	=> sub::create(),

		// dude has no clue what they're doing 💀
		_			=> show_help()
	}
}

fn show_help() -> () {
	println!(include_str!("help.txt"));
}
