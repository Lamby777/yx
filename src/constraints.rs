// Code for implementing `yx constraint` and `yx free` functionality

use crate::{YxIndexIter, YxFileRecord, PathBuf, HashMap};

pub fn filter_by_constraints<C>(it: &mut YxIndexIter, cons: Option<Vec<C>>)
	where C: Fn(&(PathBuf, YxFileRecord)) -> bool {

	if let Some(cons) = cons {
		for constraint in cons {
			let filtered = it.filter(constraint).collect::<HashMap<_, _>>();
			*it = filtered.into_iter();
		}
	}
}
