use crate::{BarnesHutTree, Udim};

use super::{get_mut_ref_from_arr_mut_ref, get_ref_from_arr_ref};

mod remove_from_direct;

mod drop_one_child_nodes;

impl<const D: Udim> BarnesHutTree<D> {
    fn sub_value_util_root(&mut self, internal_i: usize, value_i: usize) {
        let mut curr_internal_mut_ref_opt = Some(get_mut_ref_from_arr_mut_ref(
            &mut self.internal_vec,
            internal_i,
            "Getting the first more-than-1-child internal to start",
        ));

        let to_sub_v_ref = &get_ref_from_arr_ref(&self.vs, value_i, "Getting the to-sub value").0;

        while let Some(curr_internal_mut_ref) = curr_internal_mut_ref_opt {
            curr_internal_mut_ref.sub_value(to_sub_v_ref);

            if let Some((parent_i, _)) = curr_internal_mut_ref.parent {
                curr_internal_mut_ref_opt = Some(get_mut_ref_from_arr_mut_ref(
                    &mut self.internal_vec,
                    parent_i,
                    "Tracking back to sub value from parents",
                ))
            } else {
                curr_internal_mut_ref_opt = None;
            }
        }
    }

    /// # Remove a node from the tree
    pub(crate) fn sub(&mut self, value_i: usize) {
        let remove_direct_res = self.remove_from_direct_leaf(value_i);

        let internal_i = if let Some(v) = remove_direct_res {
            v
        } else {
            #[cfg(test)]
            println!("Exit sub after removing direct leaf.");
            return;
        };

        let internal_i = if let Some(v) = self.drop_one_child_internals(internal_i) {
            v
        } else {
            #[cfg(test)]
            println!("Exit sub after dropping one-child internals.");
            return;
        };

        self.sub_value_util_root(internal_i, value_i);
    }
}
