use crate::{nodes::Internal, BHTree, Udim};

impl<const D: Udim> BHTree<D> {
    /// # Removing the leaf value from the direct leaf node
    ///
    /// An added leaf value always have a direct leaf node parent containing that value.
    ///
    /// ## Removing from a leaf with one value or with multiple values
    ///
    /// For the leaf's containment status, there are two cases: the leaf node only contains the to-remove value or the leaf node contains more than the to-remove value.
    ///
    /// ### The leaf contains multiple values
    ///
    /// If the leaf node's bounding box radius is smaller than the user-defined limit, a leaf node might hold multiple values. Normally, a leaf node has only one value.
    ///
    /// If a leaf is holding multiple values, we would like to remove the to-remove value from is values list. If the to-remove value is the last one, we can simply pop it out. However, if the to-remove value is not the last one, we need to replace its value with another leaf index about which value is under the leaf's control and update the values' to leaf mapping accordingly.
    ///
    /// ### The leaf contains one value
    ///
    /// If a leaf is holding only one value, we need to cut that leaf from its parent or the root.
    ///
    #[inline]
    pub(super) fn remove_from_direct_leaf(&mut self, i: usize) -> Option<*mut Internal<D>> {
        let (parent_leaf, idx) = self.vs[i].1.expect("We should only remove an added value");

        self.vs[i].1 = None;

        let parent_leaf_mut_ref = unsafe {
            parent_leaf
                .as_mut()
                .expect("Dereferencing direct leaf parent")
        };

        if parent_leaf_mut_ref.get_values_num_inside() > 1 {
            let replaced_leaf_i = parent_leaf_mut_ref.sub_value(idx, &self.vs[i].0);
            if replaced_leaf_i > 0 {
                self.vs[replaced_leaf_i]
                    .1
                    .as_mut()
                    .expect("Using another leaf node to replace the removed")
                    .1 = idx;
            }

            if let Some((parent1_internal_ptr, _)) = parent_leaf_mut_ref.parent {
                let parent1_internal_mut_ref = unsafe {
                    parent1_internal_ptr
                        .as_mut()
                        .expect("Cutting the empty parent leaf")
                };
                parent1_internal_mut_ref.sub_value(&self.vs[i].0);

                Some(parent1_internal_ptr)
            } else {
                None
            }
        } else {
            self.nodes_num -= 1;
            if let Some((parent1_internal_ptr, dir)) = parent_leaf_mut_ref.parent {
                let parent1_internal_mut_ref = unsafe {
                    parent1_internal_ptr
                        .as_mut()
                        .expect("Cutting the empty parent leaf")
                };
                // parent1_internal_mut_ref.sub_value(&self.vs[i]);
                parent1_internal_mut_ref.drop_child(dir);
                Some(parent1_internal_ptr)
            } else {
                self.root = None;
                None
            }
        }
    }
}
