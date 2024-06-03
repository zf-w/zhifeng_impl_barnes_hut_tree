use crate::{BarnesHutTree, Udim};

mod expand_root;
mod find_pointer_to_add;

impl<const D: Udim> BarnesHutTree<D> {
    /// # Add a node into the tree
    pub(crate) fn add(&mut self, value_i: usize) {
        self.expand_root(value_i);

        let leaf_ptr = self.find_leaf_to_add_value(value_i);

        let leaf_mut_ref = unsafe { leaf_ptr.as_mut().expect("The pointer position to add") };

        let id = leaf_mut_ref.add_value(value_i, &self.vs[value_i].0); // One bug here, storing pointer pointing to stack.

        self.vs[value_i].1 = Some((leaf_ptr, id));
    }
}
