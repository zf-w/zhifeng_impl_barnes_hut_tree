use std::ptr;

use crate::{nodes::NodePtr::Le, BHTree, Udim};

mod expand_root;
mod find_pointer_to_add;

impl<const D: Udim> BHTree<D> {
    /// # Add a node into the tree
    pub fn add(&mut self, i: &usize) {
        let leaf_ptr = ptr::addr_of_mut!(*self.leaf_refs[*i]);

        let leaf_ref = unsafe { leaf_ptr.as_mut().expect("Should work") };

        let leaf_vc = leaf_ref.get_vc();

        // self.expand_root(leaf_vc);

        let (curr_ptr, prev_internal) = self.find_pointer_to_add_with_prev_internal(leaf_vc);

        let curr_ptr_ref = unsafe { curr_ptr.as_mut().expect("The pointer position to add") };
        *curr_ptr_ref = Some(Le(leaf_ptr));

        let leaf_ref = &mut *self.leaf_refs[*i];
        if let Some((parent_ptr, dir)) = prev_internal {
            leaf_ref.set_parent(parent_ptr, &dir);
        } else {
            leaf_ref.set_bb(&self.bb);
        }
    }
}
