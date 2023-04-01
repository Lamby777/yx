/*
* Structs are stored here to save space in main
*/

use crate::{HashMap, HashSet, PathBuf, Serialize, Deserialize, IntoIter, load_state};

#[derive(Debug)]
pub struct ProgramStatePathed {
	pub	path:	PathBuf,
	pub	state:	ProgramState,
}

impl ProgramStatePathed {
	fn new(path: PathBuf) -> Self {
		let state = todo!();
		
		Self {
			path:	path,
			state:	state,
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramState {
	pub index:			HashMap<PathBuf, YxFileRecord>,
	pub constraints:	YxConstraints,
}

impl ProgramState {
	pub fn new() -> Self {
		ProgramState {
			index:			HashMap::new(),
			constraints:	YxConstraints { cons: vec![] },
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct YxFileRecord {
	pub tags: HashSet<YxTag>
}

impl YxFileRecord {
	pub fn new(tag: YxTag) -> Self {
		YxFileRecord {
			tags: HashSet::from([tag]),
		}
	}
}

/// Result, but... more like an Option! :D
pub type IDFC<T>		= Result<T, Box<dyn std::error::Error>>;

// yx types
pub type YxTag			= String;
pub type YxIndexKV		= (PathBuf, YxFileRecord);
pub type YxIndexIter	= IntoIter<PathBuf, YxFileRecord>;

/// Methods of rendering with `yx render`
pub enum YxRenderMethod {
	Hardlink,
	Copy,
}

pub struct YxRenderOptions {
	pub	method:	YxRenderMethod,
	pub	rename:	bool,
	pub	iall:	bool,
}

// Closures are weird in Rust :/
pub type YxConstraintFilterClosureI<'a>	= impl (Fn(&'a YxIndexKV) -> bool);
pub type YxConstraintFilterClosure		= Box<dyn Fn(&YxIndexKV) -> bool>;

#[derive(Serialize, Deserialize, Debug)]
pub struct YxConstraints {
	pub cons: Vec<String>,
}

impl YxConstraints {
	// create a vec of to_filter_closure()'s result on each constraint
	pub fn to_filter_closures<'a>(&'a self)
	-> Vec<YxConstraintFilterClosureI> {

		self.cons.iter().map(|con| {
			YxConstraints::to_filter_closure(con)
		}).collect::<Vec<_>>()
	}

	// &str -> closure that can be used in a .filter()
	pub fn to_filter_closure(con: &str) -> YxConstraintFilterClosure {
		let mut split = con.split(" ");

		if split.clone().count() != 3 {
			panic!("Wrong number of arguments in constraint!");
		}

		let condition	= split.next().unwrap().to_lowercase();
		let matchtype	= split.next().unwrap().to_lowercase()
			.split_whitespace().collect::<String>();
		let tag			= split.next().unwrap().to_lowercase();

		match condition.as_str() {
			"tag"	=> Box::new(move |v: &YxIndexKV| {
				let (_, rec) = v;
				
				let mode = match matchtype.as_str() {
					"is"		=> true,

					// maybe more complex modes later?
					"isnot"	|
					"isnt"	| _	=> false,
				};

				// "It contains" XOR "User wants it to contain"
				rec.tags.contains(&tag) == mode
			}),

			// Don't filter anything if the condition is invalid
			_		=> Box::new(|_: &YxIndexKV| {
				true
			}),
		}
	}
}
