// Code for implementing `yx constraint` and `yx free` functionality

use crate::{YxIndexIter, ProgramState, sub::retrieve_where};

/// Return all record pairs where constraints are followed
pub fn retrieve_where_constraint(st: &mut ProgramState) -> YxIndexIter {
	let cl_vec = st.constraints.to_filter_closures();//.clone();
		
	retrieve_where(st.index.clone().into_iter(),
		move |kv| {
			for filter_fn in &cl_vec {
				let passes = true;//filter_fn(kv);

				if !passes {
					return false
				}
			}

			true
		}
	)
}
