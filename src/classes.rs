/*
* Structs are stored here to save space in main
*/

use crate::{HashMap, PathBuf, Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct YxFileRecord {
	pub tags: Vec<YxTag>
}

impl YxFileRecord {
	pub fn new() -> Self {
		YxFileRecord {
			tags: vec![],
		}
	}
}

pub type YxTag = String;
