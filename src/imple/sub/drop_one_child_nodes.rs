use crate::{
    imple::get_mut_ref_from_arr_mut_ref,
    BarnesHutTree,
    NodeIndex::{In, Le},
    Udim,
};

impl<const D: Udim> BarnesHutTree<D> {
    /// # Drop One-Child Internals
    ///
    /// After we have cut the to-remove value from the leaf and the leaf from its parent internal node, the parent internal node might only holds one child, and we need to cut these nodes off until an internal node with more than one leaves.
    ///
    /// ## Picking up the sibling
    ///
    /// After we have cut off the to-remove-value-holding leaf, we can pickup the single sibling and cut it off from the internal node.
    ///
    /// ## Tracing back
    ///
    /// We need to drop any 1-value-child internal nodes until a more-than-2-value-holding internal node or reaching root.
    ///
    /// ## Relink Sibling node
    ///
    /// We can then re-attach the sibling to that internal node or root.
    #[inline]
    pub(super) fn drop_one_child_internals(&mut self, start_internal_i: usize) -> Option<usize> {
        let internal_mut_ref = get_mut_ref_from_arr_mut_ref(
            &mut self.internal_vec,
            start_internal_i,
            "To check from the start",
        );
        let mut internal_i = start_internal_i;
        if internal_mut_ref.get_values_num_inside() > 2 {
            // Even an internal node has only one leaf node, if it contains more than two nodes, we shouldn't shrink this path because it would have more leaves without the bounding box limit.
            return Some(start_internal_i);
        }
        let mut sibling_opt: Option<usize> = None;

        for child_opt in internal_mut_ref.nexts.iter_mut() {
            if child_opt.is_some() {
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
        while let Some((curr_node_i, dir)) = get_mut_ref_from_arr_mut_ref(
            &mut self.internal_vec,
            internal_i,
            "Tracing back to cut 1-child internal nodes",
        )
        .parent
        {
            let curr_node_mut_ref = get_mut_ref_from_arr_mut_ref(
                &mut self.internal_vec,
                curr_node_i,
                "Tracking to the parent",
            );

            let curr_count = curr_node_mut_ref.get_values_num_inside();

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
            let internal_mut_ref = get_mut_ref_from_arr_mut_ref(
                &mut self.internal_vec,
                internal_i,
                "To relink the previous sibling",
            );

            let sibling_mut_ref = get_mut_ref_from_arr_mut_ref(
                &mut self.leaf_vec,
                sibling_i,
                "Getting the sibling leaf",
            );
            internal_mut_ref.link_leaf_to_dir(prev_dir, internal_i, sibling_i, sibling_mut_ref);
            Some(internal_i)
        } else {
            self.drop_internal(internal_i);
            self.set_root_leaf(sibling_i);
            None
        }
    }
}
