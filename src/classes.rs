/*
* Structs are stored here to save space in main
*/

use std::ops::Index;

use itertools::Itertools;

use crate::{HashMap, HashSet, PathBuf, Serialize, Deserialize, IntoIter};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramState {
	pub index: HashMap<PathBuf, YxFileRecord>
}

impl ProgramState {
	pub fn new() -> Self {
		ProgramState {
			index: HashMap::new(),
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

pub type YxTag = String;
pub type YxIndexKV = (PathBuf, YxFileRecord);
pub type YxIndexIter = IntoIter<PathBuf, YxFileRecord>;
pub type YxConstraintFilterClosure<'a> = impl Fn(&'a YxIndexKV) -> bool;

pub struct YxConstraints {
	cons: Vec<String>,
}

impl YxConstraints {
	pub fn to_filter_closures<'a>(&'a self)
	-> Vec<YxConstraintFilterClosure> {

		self.cons.iter().map(|con| {
			YxConstraints::to_filter_closure(con)
		}).collect::<Vec<_>>()
	}

	pub fn to_filter_closure(con: &str) -> YxConstraintFilterClosure {
		let mut split = con.split(" ");

		if split.clone().count() != 3 {
			panic!("Wrong number of arguments in constraint!");
		}

		let condition = split.next().unwrap().to_lowercase();
		let matchtype = split.next().unwrap().to_lowercase();

		match condition.as_str() {
			"tag"	=> |v: &YxIndexKV| {
				let (k,v) = v;

				true
			},

			_		=> |v: &YxIndexKV| true,
		}
	}
}
