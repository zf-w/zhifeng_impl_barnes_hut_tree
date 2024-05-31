use std::ptr;

use crate::{
    nodes::{Internal, Leaf},
    BarnesHutTree,
    NodeBox::{In, Le},
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
    pub(super) fn drop_one_child_internals(
        &mut self,
        start_internal_ptr: *mut Internal<D>,
    ) -> Option<*mut Internal<D>> {
        let mut internal_ref = unsafe {
            start_internal_ptr
                .as_mut()
                .expect("Dereferencing the start internal to check")
        };

        if internal_ref.count > 2 {
            // Even an internal node only has one leaf node, we shouldn't shrink this tree because it would have more leaves without the bounding box limit.
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

            self.nodes_num -= 1;

            if curr_node_ref.count > 2 {
                internal_ref = curr_node_ref;
                prev_dir_opt = Some(dir);
                break;
            } else {
                internal_ref = curr_node_ref;
            }
        }

        if let Some(prev_dir) = prev_dir_opt {
            internal_ref.link_leaf_to_dir(prev_dir, sibling_box);
            Some(ptr::addr_of_mut!(*internal_ref))
        } else {
            self.set_root_leaf(sibling_box);
            None
        }
    }
}
