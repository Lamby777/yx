/**
* Okay, I'm back to having no idea what I'm doing. <3
* - Dex
**/

use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Get command line args
	yx::start(args().collect::<Vec<String>>())
}
