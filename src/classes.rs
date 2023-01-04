/*
* Structs are stored here to save space in main
*/

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
pub type YxIndexIter = IntoIter<PathBuf, YxFileRecord>;
pub type YxConstraintFilterClosure<'a> = impl Fn(&'a (PathBuf, YxFileRecord)) -> bool;

pub struct YxConstraints {
	cons: Vec<String>,
}

impl YxConstraints {
	pub fn to_filter_closures<'a>(&'a self)
	-> Vec<YxConstraintFilterClosure> {

		self.cons.iter().map(|constraint| {
			let filter_closure = YxConstraints::to_filter_closure(constraint);

			filter_closure
		}).collect::<Vec<_>>()
	}

	pub fn to_filter_closure(con: &str) -> YxConstraintFilterClosure {
		|v| {true}
	}
}
