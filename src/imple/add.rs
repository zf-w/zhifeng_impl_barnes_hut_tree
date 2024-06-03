use crate::{BarnesHutTree, Udim};

mod expand_root;
mod find_pointer_to_add;

impl<const D: Udim> BarnesHutTree<D> {
    /// # Add a node into the tree
    pub(crate) fn add(&mut self, value_i: usize) {
        self.expand_root(value_i);

        let leaf_i = self.find_leaf_to_add_value(value_i);

        let leaf_mut_ref = self.leaf_vec.get_mut(leaf_i).unwrap().as_mut();

        let id = leaf_mut_ref.add_value(value_i, &self.vs[value_i].0);

        self.vs[value_i].1 = Some((leaf_i, id));
    }
}
