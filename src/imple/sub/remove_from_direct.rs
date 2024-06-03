use crate::{BarnesHutTree, Udim};

impl<const D: Udim> BarnesHutTree<D> {
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
    pub(super) fn remove_from_direct_leaf(&mut self, i: usize) -> Option<usize> {
        let (parent_leaf_i, idx) = self.vs[i].1.expect("We should only remove an added value");

        self.vs[i].1 = None;

        #[cfg(feature = "unchecked")]
        let parent_leaf_mut_ref =
            unsafe { self.leaf_vec.get_unchecked_mut(parent_leaf_i).as_mut() };

        #[cfg(not(feature = "unchecked"))]
        let parent_leaf_mut_ref = self.leaf_vec.get_mut(parent_leaf_i).unwrap().as_mut();

        if parent_leaf_mut_ref.get_values_num_inside() > 1 {
            let replaced_leaf_i = parent_leaf_mut_ref.sub_value(idx, &self.vs[i].0);

            if replaced_leaf_i > 0 {
                #[cfg(feature = "unchecked")]
                {
                    unsafe {
                        self.vs
                            .get_unchecked_mut(replaced_leaf_i)
                            .1
                            .as_mut()
                            .unwrap_unchecked()
                            .1 = idx
                    };
                }
                #[cfg(not(feature = "unchecked"))]
                {
                    self.vs
                        .get_mut(replaced_leaf_i)
                        .expect("Should be pointing to this leaf")
                        .1
                        .as_mut()
                        .expect("Using another leaf node to replace the removed")
                        .1 = idx;
                }
            }

            if let Some((parent1_internal_i, _)) = parent_leaf_mut_ref.parent {
                Some(parent1_internal_i)
            } else {
                None
            }
        } else {
            if let Some((parent1_internal_i, dir)) = parent_leaf_mut_ref.parent {
                self.drop_child(parent1_internal_i, dir);

                Some(parent1_internal_i)
            } else {
                #[cfg(feature = "unchecked")]
                {
                    self.root.take();
                    self.drop_leaf(parent_leaf_i);
                }
                #[cfg(not(feature = "unchecked"))]
                {
                    use crate::nodes::NodeIndex;
                    let to_drop = self.root.take();
                    if let Some(node_i) = to_drop {
                        match node_i {
                            NodeIndex::In(_) => unreachable!(),
                            NodeIndex::Le(leaf_i) => {
                                assert!(leaf_i == parent_leaf_i);
                                self.drop_leaf(leaf_i);
                            }
                        }
                    }
                }

                None
            }
        }
    }
}
