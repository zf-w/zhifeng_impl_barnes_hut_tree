use std::ptr;

use crate::{
    nodes::{
        Internal, Leaf,
        NodeBox::{self, In, Le},
    },
    BHTree, Udim,
};

impl<const D: Udim> BHTree<D> {
    /// # Find the pointer to add the leaf node
    ///
    /// We need to find the correct position to add a leaf position.
    /// First, we need find the correct direction to continue.
    /// If the final position is a leaf node, we need to insert an internal node in the middle and reinsert the two leaf nodes.
    ///
    pub(super) fn find_leaf_to_add_value(&mut self, i: usize) -> *mut Leaf<D> {
        let leaf_vc = &self.vs[i];

        let mut curr_ptr = ptr::addr_of_mut!(self.root);
        let mut prev_internal: Option<(*mut Internal<D>, usize)> = None;

        fn relink_node<const D: Udim>(curr_ptr: *mut Option<NodeBox<D>>, node: NodeBox<D>) {
            let curr_ptr_mut_ref = unsafe {
                curr_ptr
                    .as_mut()
                    .expect("Should be a valid None Option now")
            };

            *curr_ptr_mut_ref = Some(node);
        }

        while let Some(curr) =
            unsafe {
                curr_ptr.as_mut().expect(
                "Checking the next pointer, see if that's an avaliable place to put the leaf.",
            ).take()
            }
        {
            let mut target_internal = match curr {
                Le(mut curr_leaf_box) => {
                    if curr_leaf_box.bb.br <= self.br_limit {
                        let curr_leaf_ptr = ptr::addr_of_mut!(*curr_leaf_box);

                        relink_node(curr_ptr, NodeBox::Le(curr_leaf_box));

                        return curr_leaf_ptr;
                    } else {
                        self.count += 1;

                        Internal::new_with_leaf_replacement(curr_leaf_box)
                    }
                }
                In(internal_box) => internal_box,
            };

            target_internal.add_value(leaf_vc);

            let next_dir = target_internal.calc_next_dir(leaf_vc);
            let next_ptr = target_internal.get_child_star_mut(&next_dir);

            prev_internal = Some((ptr::addr_of_mut!(*target_internal), next_dir.clone()));

            relink_node(curr_ptr, NodeBox::In(target_internal));

            curr_ptr = next_ptr;
        }

        if let Some((parent_internal_ptr, from_dir)) = prev_internal {
            let parent_internal_mut_ref =
                unsafe { parent_internal_ptr.as_mut().expect("Should work") };
            let mut ans_leaf_box =
                Leaf::new_empty_from_parent_dir(parent_internal_mut_ref, from_dir);

            let ans_leaf_ptr = ptr::addr_of_mut!(*ans_leaf_box);

            parent_internal_mut_ref.attach_leaf_to_dir(from_dir, ans_leaf_box);

            self.count += 1;

            return ans_leaf_ptr;
        } else {
            let mut ans_leaf = Leaf::new_empty_from_bb(self.bb.clone());
            let ans_leaf_ptr = ptr::addr_of_mut!(*ans_leaf);

            self.root = Some(NodeBox::Le(ans_leaf));
            self.count += 1;

            return ans_leaf_ptr;
        }
    }
}
