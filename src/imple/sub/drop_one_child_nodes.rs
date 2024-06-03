use crate::{
    BarnesHutTree,
    NodeIndex::{In, Le},
    Udim,
};

impl<const D: Udim> BarnesHutTree<D> {
    /// # Drop One-Child Internals
    ///
    /// After we have cut the to-remove value from the leaf and the leaf from its parent internal node, the parent internal node might only holds one child, and we need to cut these nodes out until an internal node with more than one leaves.
    ///
    /// ## Picking up the sibling
    ///
    /// After we have cut off the to-remove-value-holding leaf, we can pickup the single sibling and cut it off from the internal node.
    #[inline]
    pub(super) fn drop_one_child_internals(&mut self, start_internal_i: usize) -> Option<usize> {
        let internal_mut_ref = self
            .internal_vec
            .get_mut(start_internal_i)
            .unwrap()
            .as_mut();
        let mut internal_i = start_internal_i;
        if internal_mut_ref.count > 2 {
            // Even an internal node only has one leaf node, we shouldn't shrink this tree because it would have more leaves without the bounding box limit.
            return Some(start_internal_i);
        }
        let mut sibling_opt: Option<usize> = None;

        for child_opt in internal_mut_ref.nexts.iter_mut() {
            if child_opt.is_some() {
                // debug_assert!(child_count <= 1, "Should not have more than one child");
                if sibling_opt.is_none() {
                    sibling_opt = Some(
                        match child_opt.take().expect("The sibling, just check is some") {
                            Le(leaf_i) => leaf_i,
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

        let sibling_i = if let Some(sibling) = sibling_opt {
            sibling
        } else {
            debug_assert!(false, "A one-child internal having none children...");
            return None;
        };
        let mut prev_dir_opt: Option<usize> = None;
        while let Some((curr_node_i, dir)) = unsafe {
            self.internal_vec
                .get_unchecked_mut(internal_i)
                .as_ref()
                .parent
        } {
            let curr_node_mut_ref = if cfg!(feature = "unchecked") {
                unsafe { self.internal_vec.get_unchecked_mut(curr_node_i).as_mut() }
            } else {
                self.internal_vec.get_mut(curr_node_i).unwrap().as_mut()
            };

            let curr_count = curr_node_mut_ref.count;

            curr_node_mut_ref.nexts[dir] = None;

            // Sometimes, there might be a rare case that droping the internal changes the index of its parent.
            if let Some((old_i, new_i)) = self.drop_internal(internal_i) {
                if old_i == curr_node_i {
                    internal_i = new_i;
                } else {
                    internal_i = curr_node_i;
                }
            } else {
                internal_i = curr_node_i;
            }

            if curr_count > 2 {
                prev_dir_opt = Some(dir);
                break;
            }
        }

        if let Some(prev_dir) = prev_dir_opt {
            let internal_mut_ref =
                unsafe { self.internal_vec.get_unchecked_mut(internal_i).as_mut() };
            let sibling_mut_ref = self.leaf_vec.get_mut(sibling_i).unwrap().as_mut();
            internal_mut_ref.link_leaf_to_dir(prev_dir, internal_i, sibling_i, sibling_mut_ref);
            Some(internal_i)
        } else {
            self.drop_internal(internal_i);
            self.set_root_leaf(sibling_i);
            None
        }
    }
}
