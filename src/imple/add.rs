use std::ptr;

use crate::{nodes::NodeBox::Le, BHTree, Udim};

mod expand_root;
mod find_pointer_to_add;

impl<const D: Udim> BHTree<D> {
    /// # Add a node into the tree
    pub fn add(&mut self, i: &usize) {
        self.expand_root(i);

        let leaf_ptr = self.find_leaf_to_add_value(i);

        let leaf_mut_ref = unsafe { leaf_ptr.as_mut().expect("The pointer position to add") };

        let id = leaf_mut_ref.add_value(&self.vs[*i]); // One bug here, storing pointer pointing to stack.

        self.to_leafs[*i] = Some((leaf_ptr, id));
    }
}
