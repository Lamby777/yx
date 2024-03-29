#![feature(map_many_mut)]
#![feature(type_alias_impl_trait)]
#![feature(hash_drain_filter)]
// I have no idea why I have to do this in lib.rs :(

const INDEX_FILE_NAME: &str	= ".yx_index";

/// A shit ton of dashes to split up condensed data
const LINE_SEPARATOR: &str	= "--------------------------------------------------";

use std::{fs, env};
use std::collections::{HashMap, HashSet, hash_map::IntoIter};
use cli::c_scribe;
use itertools::Itertools;
use std::path::{PathBuf, Path};
use serde::{Serialize, Deserialize};
use indoc::indoc;
use text_io::read;

mod classes;
use classes::*;
pub use classes::IDFC;

mod sub;
mod constraints;
mod render;
mod scribe;

mod cli {
    use std::path::Path;
    use crate::classes::{ProgramState, ProgramStatePathed};
	use crate::sub::path_relative_to_index;
	use crate::{sub, IDFC, scribe};

	pub fn c_create(pathable: impl AsRef<Path>) -> IDFC<()> {
		let path = pathable.as_ref();

		if path.exists() {
			Err("An index already exists here! Consider deleting it.".into())
		} else {
			sub::write_to_index(path, &ProgramState::new())
		}
	}

	// Be careful!
	pub fn c_purge(st: &mut ProgramStatePathed) -> IDFC<()> {
		st.state = ProgramState::new();

		sub::write_to_index(&st.path, &st.state)
	}

	pub fn c_ignore(
		st: &mut ProgramStatePathed,
		target: impl AsRef<Path>,
	) -> IDFC<()> {
		let target = path_relative_to_index(target)?;
		
		let was_new_insert = st.state.ignores.insert(target);
		if !was_new_insert {
			println!("Already in ignores!");
		}

		sub::write_to_index(&st.path, &st.state)
	}

	pub fn c_unignore(
		st: &mut ProgramStatePathed,
		target: impl AsRef<Path>, // TODO: allow multiple tags to remove at once
	) -> IDFC<()> {
		let target = path_relative_to_index(target)?;

		st.state.ignores.drain_filter(
			|v| v == &target
		);

		sub::write_to_index(&st.path, &st.state)
	}

	pub fn c_add(
		st: &mut ProgramStatePathed,
		target: impl AsRef<Path>,
		tags: &[&str],
	) -> IDFC<()> {
		sub::add_tags_to(
			&mut st.state,
			target.as_ref(),
			tags,
		)?;

		sub::write_to_index(&st.path, &st.state)
	}

	pub fn c_remove(
		st: &mut ProgramStatePathed,
		target: impl AsRef<Path>,
		tags: &[&str],
	) -> IDFC<()> {
		sub::rm_tags_from(
			&mut st.state,
			target.as_ref(),
			tags
		)?;
		
		sub::write_to_index(&st.path, &st.state)
	}

	pub fn c_scribe(
		st: &mut ProgramState,
		target: impl AsRef<Path>
	) -> IDFC<()> {
		scribe::import_from_names(st, target, scribe::ScribeMethod::SplitBy(" "))
	}
}

pub fn start(args: Vec<String>) -> IDFC<()> {
	if args.len() < 2 {
		show_help();
		return Ok(())
	}

	let cmd	= &args[1].to_lowercase();
	let args: &[&str]	= &args[2..].iter()
							.map(|s| s.as_str())
							.collect::<Vec<&str>>();


	// shadow the String with a &str slice into itself
	// OR with an alias's full form, if possible
	let cmd = cmd_replace_aliases(cmd);

	match cmd {
		"create"	=> {
			assert_argc(args, &[0, 1]);

			let location = if args.len() < 1 {
				// current working dir
				get_cwd().join(INDEX_FILE_NAME)
			} else {
				PathBuf::from(&args[0])
			};

			cli::c_create(location)?;
		},

		"purge"		=> {
			assert_argc(args, &[0, 1]);

			// short circuit check if "yes" is the next arg
			let confirmed = args.len() >= 1 && (args[0].to_lowercase() == "yes");
			let closest = get_closest_index()?;
			let st =
				parse_index_at(&closest).unwrap_or_else(
					|_| ProgramState::new()
				);

			if !confirmed {
				if args.len() <= 0 {
					// Prompt for confirmation
					println!( indoc! {"

						{}
						Are you sure? This will clear out every tag from the index!
						Just to be clear, you'll be clearing this index:
						{}
						(found closest to the current working directory)
						{}

						[Y/N]"
					}, LINE_SEPARATOR, closest.display(), LINE_SEPARATOR);
					
					if !repeat_prompt_yn() {
						return Ok(())
					}
				} else {
					show_help();
					panic!("Invalid use of yx purge!");
				}
			}

			// are they gone yet?
			// ok cool, they're gone.

			// make a ProgramStatePathed
			let mut st_pathed = ProgramStatePathed {
				path:	closest,
				state:	st,
			};

			cli::c_purge(&mut st_pathed)?;

			// at long last, we purge the tags, because
			// no one with any regrets would get this far.
			// that wasn't so hard, was it?
		},

		"ignore"	=> {
			assert_argc(args, &[1]);

			cli::c_ignore(
				&mut load_state_and_path()?,
				&args[0],
			)?;
		},

		"unignore"	=> {
			assert_argc(args, &[1]);

			cli::c_unignore(
				&mut load_state_and_path()?,
				&args[0],
			)?;
		},

		"ilist"		=> {
			assert_argc(args, &[0]);

			let st = load_state_only()?;

			println!("Ignored: {}", st.ignores.iter().map(|ref v| v.display()).join(", "));
		},

		"add"		=> {
			assert_argc(args, &[2]);

			cli::c_add(
				&mut load_state_and_path()?,
				&args[0],
				&args[1..],
			)?;
		},

		"rm"		=> {
			assert_argc(args, &[2]);

			let mut st = load_state_and_path()?;

			// If file doesn't have tag, yell at the user :P
			let has_tag = sub::file_has_tag(
				&st.state,
				(&args[0]).into(),
				&args[1]
			)?;

			if !has_tag {
				return Err(
					"File doesn't have this tag!".into()
				);
			}

			cli::c_remove(
				&mut st,
				&args[0],
				&args[1..],
			)?;
		},

		"mv"	| "mvt"	|
		"cp"	| "cpt"	|
		"apt"	| "mapt" => {
			assert_argc(args, &[2]);

			let mut st = load_state_only()?;

			let cmd_action_fn = match cmd {
				"mv"		=> sub::fedit::move_file_and_tags,
				"mvt"		=> sub::fedit::move_tags,
				"cp"		=> sub::fedit::copy_file_and_tags,
				"cpt"		=> sub::fedit::copy_tags,
				"apt"		=> sub::fedit::append_tags,
				"mapt"		=> sub::fedit::append_tags_rm_old,
				_			=> unreachable!(),
			};

			cmd_action_fn(
				&mut st,
				(&args[0]).into(),
				(&args[1]).into()
			);

			sub::write_to_index(&get_closest_index().unwrap(), &st)?
		},

		"scribe"	=> {
			assert_argc(args, &[0, 1]);

			let st = &mut load_state_only()?;
			let target = args.get(0).unwrap_or(&".");

			c_scribe(
				st,
				target,
			)?;

			sub::write_to_index(&get_closest_index().unwrap(), &st)?
		},

		"render"	=> {
			assert_argc(args, &[0, 1, 2]);

			let st = load_state_only()?;

			// Get modes from args
			let (m_copy, m_rename, m_iall) = match args.len() {
				0	=> (false, false, false),
				_	=> {
					(
						args.contains(&"copy"),
						args.contains(&"named"),
						args.contains(&"iall"), // include all, even outside index
					)
				}
			};

			let render_method = if m_copy {
				YxRenderMethod::Copy
			} else {
				YxRenderMethod::Hardlink
			};

			let res = render::render(&st, YxRenderOptions {
				method:	render_method,
				rename: m_rename,
				iall:	m_iall,
			});

			if let Err(e) = res {
				println!("There was an error while rendering.");
				return Err(e)
			} else {
				println!("Rendered successfully!");
			}
		},

		"la"		=> {
			assert_argc(args, &[0, 1]);

			let st = load_state_only()?;
			let tag_output_separator = ",\n";

			let out =
				if args.len() != 0 {
					let order_mode: &str = &args[0].to_lowercase();
					if matches!(order_mode, "l->h" | "h->l") {
						let counts = sub::get_tag_counts_from(
							&st,
						)?;
						
						let sorted = counts.keys().sorted_by(
							|a, b| {
								match order_mode {
									"l->h"	=> Ord::cmp(&counts[*a], &counts[*b]),
									"h->l"	=> Ord::cmp(&counts[*b], &counts[*a]),
									_		=> unreachable!(),
								}
							}
						);

						sorted.map(
							// Add counts in parentheses next to tag
							|v| format!("({}) \t>> {v}", counts[v])
						).join(tag_output_separator)
					} else {
						return Err("Invalid `yx la` mode!".into());
					}
				} else {
					sub::get_all_tags_from(
						&st,
					)?.join(tag_output_separator)
				};

			println!("Tags: {}", out);
		}

		"list"		=> {
			assert_argc(args, &[0, 1, 2]);
			let argc = args.len();

			let st = load_state_only()?;

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
				println!( indoc! {"
					{}
					Index has no entries!
					Use `yx add <file> <tag>` to get started.
					{}
				"}, LINE_SEPARATOR, LINE_SEPARATOR);
				return Ok(())
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

	Ok(())
}

pub fn show_help() {
	println!("{}\n{}{}\n", LINE_SEPARATOR, include_str!("help.txt"), LINE_SEPARATOR);
}

pub fn load_state_and_path() -> IDFC<ProgramStatePathed> {
	Ok(load_state_and_path_from(&get_closest_index()?)?)
}

pub fn load_state_only() -> IDFC<ProgramState> {
	Ok(parse_index_at(get_closest_index()?)?)
}

pub fn load_state_and_path_from(index: &Path) -> IDFC<ProgramStatePathed> {
	Ok(ProgramStatePathed::from_path(index.to_path_buf())?)
}

pub fn parse_index_at(index_path: impl AsRef<Path>) -> IDFC<ProgramState> {
	let read_data = fs::read_to_string(index_path.as_ref())?;
	let mut res = serde_json::from_str::<ProgramState>(&read_data).map_err(
		|e| {
			println!("Failed to parse index... Did you recently do an update?");
			e
		}
	)?;

	res.ignores.insert(format!("./{}", INDEX_FILE_NAME).into());

	Ok(res)
}

pub fn get_closest_index() -> IDFC<PathBuf> {
	let v = get_all_current_indexes();
	let closest = v.get(0);
	
	closest.cloned().ok_or_else(
		|| format!("{} not found in current path!", INDEX_FILE_NAME).as_str().into()
	)
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

/// this just builds a string for convenience.
/// use get_closest_index() for finding what to write to
pub fn cwd_index_path() -> PathBuf {
	get_cwd().join(INDEX_FILE_NAME)
}

pub fn get_cwd() -> PathBuf {
	env::current_dir().expect("Error getting current directory")
}

pub fn assert_argc(args: &[&str], lens: &[usize]) {
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
		"listall"			=> "la",
		"ignorelist"		=> "ilist",

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

pub fn repeat_prompt_yn() -> bool {
	let res: char;

	loop {
		let res_attempt: String = read!();
		let res_attempt = res_attempt.to_lowercase().chars().nth(0);

		if let Some(res_n) = res_attempt {
			res = res_n;
			break
		} else {
			println!("Really? Come on! Type something!");
		}
	}

	res == 'y'
}
