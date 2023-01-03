/*
* Structs are stored here to save space in main
*/

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

pub struct YxConstraints {
	//
}
