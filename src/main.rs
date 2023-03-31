/**
* Okay, I'm back to having no idea what I'm doing. <3
* - Dex
**/

use std::env::args;
use yx::IDFC;

fn main() -> IDFC<()> {
	// Get command line args
	yx::start(args().collect::<Vec<String>>())
}
