use std::ptr;

use crate::{
    colvec::ColVec,
    nodes::NodePtr::{self, In, Le},
    BHTree, Udim,
};

impl<const D: Udim> BHTree<D> {
    pub fn expand_struct_bb(&mut self, vc: &ColVec<D>) {
        let mut curr_bb = self.bb.clone();
        while !curr_bb.is_containing(vc) {
            curr_bb = curr_bb.calc_reverse_expand_bb(vc).0;
        }
        self.bb.clone_from(&curr_bb);
    }

    /// # Expand root bounding box according to new leaf value
    #[inline]
    pub fn expand_root(&mut self, vc: &ColVec<D>) {
        if self.bb.is_containing(vc) {
            return;
        }
        if let Some(root_ptr) = self.root.take() {
            match root_ptr {
                Le(leaf_ptr) => {
                    let leaf_mut_ref = unsafe {
                        leaf_ptr
                            .as_mut()
                            .expect("Dereferencing leaf to update its box")
                    };

                    self.expand_struct_bb(vc);

                    leaf_mut_ref.bb.clone_from(&self.bb);
                    self.root = Some(NodePtr::Le(leaf_ptr));
                }
                In(internal_box) => {
                    let mut root_box = internal_box;
                    while !root_box.bb.is_containing(vc) {
                        let (mut new_root, dir) = root_box.calc_new_internal_with_new_vc(vc);
                        root_box.parent = Some((ptr::addr_of_mut!(*root_box), dir));
                        new_root.nexts[dir] = Some(NodePtr::In(root_box));
                        root_box = new_root;
                        self.count += 1;
                    }
                    self.bb.clone_from(&root_box.bb);
                    self.root = Some(NodePtr::In(root_box));
                }
            }
        } else {
            self.expand_struct_bb(vc);
        }
    }
}
