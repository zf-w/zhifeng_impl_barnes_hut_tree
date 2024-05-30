use crate::{nodes::Internal, BHTree, Udim};

mod remove_from_direct;

mod drop_one_child_nodes;

impl<const D: Udim> BHTree<D> {
    fn sub_value_util_root(&mut self, internal_ptr: *mut Internal<D>, value_i: usize) {
        let mut curr_internal_mut_ref_opt = Some(unsafe {
            internal_ptr
                .as_mut()
                .expect("Dereferencing the previous found more-than-one-child internal;")
        });
        let to_sub_v_ref = &self.vs[value_i].0;
        while let Some(curr_internal_mut_ref) = curr_internal_mut_ref_opt {
            curr_internal_mut_ref.sub_value(to_sub_v_ref);
            if let Some((parent_ptr, _)) = curr_internal_mut_ref.parent {
                curr_internal_mut_ref_opt =
                    Some(unsafe { parent_ptr.as_mut().expect("Tracing to the parent") })
            } else {
                curr_internal_mut_ref_opt = None;
            }
        }
    }

    /// # Remove a node from the tree
    pub fn sub(&mut self, value_i: usize) {
        let remove_direct_res = self.remove_from_direct_leaf(value_i);

        let internal_ptr = if let Some(v) = remove_direct_res {
            v
        } else {
            #[cfg(test)]
            println!("Exit sub after removing direct leaf.");
            return;
        };

        let internal_ptr = if let Some(v) = self.drop_one_child_internals(internal_ptr) {
            v
        } else {
            #[cfg(test)]
            println!("Exit sub after dropping one-child internals.");
            return;
        };

        self.sub_value_util_root(internal_ptr, value_i);
    }
}
