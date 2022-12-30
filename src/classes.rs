/*
* Structs are stored here to save space in main
*/

use crate::{HashMap, PathBuf};

pub struct ProgramState {
	index: HashMap<PathBuf, YxFileRecord>
}

pub struct YxFileRecord {
	tags: Vec<YxTag>
}

pub type YxTag = String;
