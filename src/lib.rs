#![feature(map_many_mut)]
#![feature(type_alias_impl_trait)]
// I have no idea why I have to do this in lib.rs :(

const INDEX_FILE_NAME: &str	= ".yx_index";

// a shit ton of dashes to split up condensed data
const LINE_SEPARATOR: &str	= "--------------------------------------------------";

use std::{fs, env};
use std::collections::{HashMap, HashSet, hash_map::IntoIter};
use itertools::Itertools;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use indoc::indoc;
use text_io::read;

mod sub;

mod classes;
use classes::*;

mod constraints;

pub fn start(args: Vec<String>) {
	if args.len() < 2 { return show_help(); }

	let cmd = &args[1].to_lowercase();		// give the cmd its own binding
	let args = &args[2..];	// shadow first vec

	// shadow the String with a &str slice into itself
	// OR with an alias's full form, if possible
	let cmd = cmd_replace_aliases(cmd);

	match cmd {
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

		"purge"		=> {
			// short circuit check if "yes" is the next arg
			let confirmed = args.len() >= 1 && (args[0] == "yes");
			let closest = get_closest_index();

			if closest.is_none() {
				panic!("No index files in this directory or any of its parent directories!");
			}

			let closest = closest.unwrap();

			if !confirmed {
				if args.len() <= 0 {
					if !sub::confirm_purge(&closest) { return }
				} else {
					show_help();
					panic!("Invalid use of yx purge!");
				}
			}

			// are they gone yet?

			// ok cool, they're gone.

			// at long last, we purge the tags, because
			// no one with any regrets would get this far.

			let mut st = load_state();
			st.index = HashMap::new();

			sub::write_to_index(closest, st);

			// that wasn't so hard, was it?
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

		"mv"	| "mvt"	|
		"cp"	| "cpt"	|
		"apt"	| "mapt" => {
			assert_argc(args, &[2]);

			let mut st = load_state();

			let f = match cmd {
				"mv"		=> sub::fedit::move_file_and_tags,
				"mvt"		=> sub::fedit::move_tags,
				"cp"		=> sub::fedit::copy_file_and_tags,
				"cpt"		=> sub::fedit::copy_tags,
				"apt"		=> sub::fedit::append_tags,
				"mapt"		=> sub::fedit::append_tags_rm_old,

				_ => panic!("bruh"),
			};

			f(
				&mut st,
				(&args[0]).into(),
				(&args[1]).into()
			);

			sub::write_to_index(get_closest_index().unwrap(), st)
		},

		"render"	=> {
			assert_argc(args, &[0, 1, 2]);

			let st = load_state();

			// Get modes from args
			let (m_copy, m_rename) = match args.len() {
				0	=> (false, false),
				_	=> {
					let args_sl = args.iter().map(|v| v.as_str()).collect::<Vec<&str>>();
					(
						args_sl.contains(&"copy"),
						args_sl.contains(&"named"),
					)
				}
			};

			(if m_copy {
				// Copy files
				sub::render::copied
			} else {
				// Create hard links to files
				sub::render::hardlinked
			})(&st, m_rename);
		},

		"list"		=> {
			assert_argc(args, &[0, 1, 2]);
			let argc = args.len();

			let st = load_state();

			let it = st.index.into_iter();

			let it = match argc {
				1 => {
					// yx list <missing>
					let mode = args[0].to_lowercase();
					let mode_str = mode.as_str();
					match mode_str {
						"valid" | "missing" => {
							sub::retrieve_where(it, |v| {
								// if it exists			+ the user wants to find valid
								// if it doesn't exist	+ the user wants to find missing
								v.0.exists() == (mode_str == "valid")
							})
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
				let tags = record.tags.iter().join(", ");

				println!("{} >> {tags}", path.display());
				println!("{}", LINE_SEPARATOR);
			}

			println!();
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

fn cmd_replace_aliases<'a>(cmd: &'a String) -> &'a str {
	match cmd.as_str() {
		"ls"				=> "list",
		"move"				=> "mv",
		"copy"				=> "cp",

		"mvtags"			|
		"movetags"			=> "mvt",
		
		"cptags"			|
		"owtags"			|
		"overwritetags"		|
		"overwrite"			|
		"cpoverwrite"		|
		"cpo"				|
		"copytags"			=> "cpt",

		"merge"				|
		"mergetags"			|
		"mt"				|
		"met"				|
		"append"			|
		"appendtags"		=> "apt",

		"moveapt"			|
		"moveappend"		|
		"moveappendtags"	=> "mapt",

		_		=> &cmd
	}
}
