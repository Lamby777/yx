/**
* Okay, I'm back to having no idea what I'm doing. <3
* - Dex
**/

use std::env;
use serde::{Serialize, Deserialize};

fn main() {
	// Get command line args
	let args: Vec<String> = env::args().collect();
	
	if args.len() < 2 { return show_help(); }

	let cmd = &args[1];		// give the cmd its own binding
	let args = &args[2..];	// shadow first vec
}

fn show_help() -> () {
	println!(include_str!("help.txt"));
}
