use std::ptr;

use crate::{
    nodes::NodeBox::{self, In, Le},
    BHTree, Udim,
};

impl<const D: Udim> BHTree<D> {
    fn expand_struct_bb(&mut self, i: usize) {
        let vc = &self.vs[i];
        let mut curr_bb = self.bb.clone();
        while !curr_bb.is_containing(vc) {
            curr_bb = curr_bb.calc_reverse_expand_bb(vc).0;
        }
        self.bb.clone_from(&curr_bb);
    }

    /// # Expand root bounding box according to new leaf value
    #[inline]
    pub(super) fn expand_root(&mut self, i: usize) {
        let vc = &self.vs[i];

        if self.bb.is_containing(vc) {
            return;
        }
        if let Some(root_ptr) = self.root.take() {
            match root_ptr {
                Le(mut leaf_box) => {
                    self.expand_struct_bb(i);

                    leaf_box.bb.clone_from(&self.bb);
                    self.root = Some(NodeBox::Le(leaf_box));
                }
                In(internal_box) => {
                    let mut root_box = internal_box;
                    while !root_box.bb.is_containing(vc) {
                        let (mut new_root, dir) = root_box.calc_new_internal_with_new_vc(vc);
                        root_box.parent = Some((ptr::addr_of_mut!(*root_box), dir));
                        new_root.nexts[dir] = Some(NodeBox::In(root_box));
                        root_box = new_root;
                        self.count += 1;
                    }
                    self.bb.clone_from(&root_box.bb);
                    self.root = Some(NodeBox::In(root_box));
                }
            }
        } else {
            self.expand_struct_bb(i);
        }
    }
}
