/*
* Structs are stored here to save space in main
*/

use crate::{HashMap, PathBuf, Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ProgramState {
	index: HashMap<PathBuf, YxFileRecord>
}

#[derive(Serialize, Deserialize)]
pub struct YxFileRecord {
	tags: Vec<YxTag>
}

pub type YxTag = String;
