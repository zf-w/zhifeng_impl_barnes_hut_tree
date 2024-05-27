use std::ptr;

use crate::{
    colvec::ColVec,
    nodes::{
        Internal,
        NodePtr::{self, In, Le},
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
    pub(super) fn find_pointer_to_add_with_prev_internal(
        &mut self,
        leaf_vc: &ColVec<D>,
    ) -> (*mut Option<NodePtr<D>>, Option<(*mut Internal<D>, usize)>) {
        let mut curr_ptr = ptr::addr_of_mut!(self.root);
        let mut prev_internal: Option<(*mut Internal<D>, usize)> = None;

        while let Some(curr_ref) = unsafe {
            curr_ptr.as_mut().expect(
                "Checking the next pointer, see if that's an avaliable place to put the leaf.",
            )
        } {
            let target_internal = match curr_ref {
                Le(curr_leaf_ptr) => {
                    let mut inserting_internal_box =
                        Internal::new_with_leaf_replacement(*curr_leaf_ptr);
                    self.count += 1;
                    // println!("{}", *inserting_internal_box);
                    let inserting_internal_star_mut = ptr::addr_of_mut!(*inserting_internal_box);
                    *curr_ref = NodePtr::In(inserting_internal_box);

                    unsafe {
                        inserting_internal_star_mut
                            .as_mut()
                            .expect("The Internal Node Just inserted")
                    }
                }
                In(internal_box) => &mut *internal_box,
            };

            target_internal.add_vc(leaf_vc);

            let next_dir = target_internal.calc_next_dir(leaf_vc);
            let next_ptr = target_internal.get_child_star_mut(&next_dir);

            curr_ptr = next_ptr;

            prev_internal = Some((ptr::addr_of_mut!(*target_internal), next_dir.clone()));
        }

        (curr_ptr, prev_internal)
    }
}
