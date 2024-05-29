use std::ptr;

use crate::{
    nodes::{Internal, Leaf, NodeBox},
    BHTree,
    NodeBox::{In, Le},
    Udim,
};

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
    fn remove_from_direct_leaf(&mut self, i: usize) -> Option<*mut Internal<D>> {
        let (parent_leaf, idx) = self.to_leafs[i].expect("We should only remove an added value");

        self.to_leafs[i] = None;

        let parent_leaf_mut_ref = unsafe {
            parent_leaf
                .as_mut()
                .expect("Dereferencing direct leaf parent")
        };
        parent_leaf_mut_ref.vc.sub_colvec_from_self(&self.vs[i]);

        if parent_leaf_mut_ref.get_num_nodes_inside() > 1 {
            let replaced_leaf_i = parent_leaf_mut_ref.sub_value(idx, &self.vs[i]);
            if replaced_leaf_i > 0 {
                self.to_leafs[replaced_leaf_i]
                    .as_mut()
                    .expect("Using another leaf node to replace the removed")
                    .1 = idx;
            }

            if let Some((parent1_internal_ptr, dir)) = parent_leaf_mut_ref.parent {
                let parent1_internal_mut_ref = unsafe {
                    parent1_internal_ptr
                        .as_mut()
                        .expect("Cutting the empty parent leaf")
                };
                parent1_internal_mut_ref.sub_value(&self.vs[i]);

                Some(parent1_internal_ptr)
            } else {
                None
            }
        } else {
            if let Some((parent1_internal_ptr, dir)) = parent_leaf_mut_ref.parent {
                let parent1_internal_mut_ref = unsafe {
                    parent1_internal_ptr
                        .as_mut()
                        .expect("Cutting the empty parent leaf")
                };
                parent1_internal_mut_ref.sub_value(&self.vs[i]);
                parent1_internal_mut_ref.nexts[dir] = None;
                Some(parent1_internal_ptr)
            } else {
                self.root = None;
                None
            }
        }
    }

    /// # Drop One-Child Internals
    ///
    /// After we have cut the to-remove value from the leaf and the leaf from its parent internal node, the parent internal node might only holds one child, and we need to cut these nodes out until an internal node with more than one leaves.
    ///
    /// ## Picking up the sibling
    ///
    /// After we have cut off the to-remove-value-holding leaf, we can pickup the single sibling and cut it off from the internal node.
    #[inline]
    fn drop_one_child_internals(
        &mut self,
        start_internal_ptr: *mut Internal<D>,
    ) -> Option<*mut Internal<D>> {
        let mut internal_ref = unsafe {
            start_internal_ptr
                .as_mut()
                .expect("Dereferencing the start internal to check")
        };

        if internal_ref.leaf_count > 1 {
            return Some(start_internal_ptr);
        }
        let mut sibling_opt: Option<Box<Leaf<D>>> = None;
        for child_opt in internal_ref.nexts.iter_mut() {
            if child_opt.is_some() {
                if sibling_opt.is_none() {
                    sibling_opt = Some(
                        match child_opt.take().expect("The sibling, just check is some") {
                            Le(leaf_box) => leaf_box,
                            In(_) => unreachable!(),
                        },
                    );
                } else {
                    debug_assert!(
                        false,
                        "Appears that a one-child internal having more than one children..."
                    );
                    return None;
                }
            }
        }

        let sibling_box = if let Some(sibling) = sibling_opt {
            sibling
        } else {
            debug_assert!(false, "A one-child internal having none children...");
            return None;
        };
        let mut prev_dir_opt: Option<usize> = None;
        while let Some((curr_node_ptr, dir)) = internal_ref.parent {
            let curr_node_ref = unsafe {
                curr_node_ptr
                    .as_mut()
                    .expect("Dereferencing parent nodes...")
            };

            curr_node_ref.drop_child(dir);

            self.count -= 1;

            if curr_node_ref.leaf_count > 1 {
                internal_ref = curr_node_ref;
                prev_dir_opt = Some(dir);
                break;
            } else {
                internal_ref = curr_node_ref;
            }
        }

        if let Some(prev_dir) = prev_dir_opt {
            internal_ref.relink_leaf(prev_dir, sibling_box);
            Some(ptr::addr_of_mut!(*internal_ref))
        } else {
            self.set_root_leaf(sibling_box);
            None
        }
    }

    /// # Remove a node from the tree
    pub fn sub(&mut self, i: usize) {
        let remove_direct_res = self.remove_from_direct_leaf(i);

        let internal_ptr = if let Some(v) = remove_direct_res {
            v
        } else {
            return;
        };
    }
}
