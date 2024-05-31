use std::ptr;

use crate::{
    nodes::{
        Internal,
        NodeBox::{self, In, Le},
    },
    BarnesHutTree, ColVec, Udim,
};

impl<const D: Udim> BarnesHutTree<D> {
    fn expand_struct_bb(&mut self, i: usize) {
        let vc = &self.vs[i].0;

        while !self.bb.is_containing(vc) {
            self.bb.self_expand(vc);
        }
    }

    #[inline]
    fn expand_root_internal(
        mut root_box: Box<Internal<D>>,
        vc: &ColVec<D>,
    ) -> (Box<Internal<D>>, usize) {
        let mut new_created_count: usize = 0;
        while !root_box.bb.is_containing(vc) {
            let (mut new_root_box, dir) = root_box.calc_new_internal_with_new_vc(vc);
            root_box.parent = Some((ptr::addr_of_mut!(*new_root_box), dir)); // One Bug here before, creating a self loop hahaha
            new_root_box.nexts[dir] = Some(NodeBox::In(root_box));
            root_box = new_root_box;
            new_created_count += 1;
        }
        (root_box, new_created_count)
    }

    /// # Expand root bounding box according to new leaf value
    #[inline]
    pub(super) fn expand_root(&mut self, i: usize) {
        let vc = &self.vs[i].0;

        if self.bb.is_containing(vc) {
            return;
        }
        if let Some(root_ptr) = self.root.take() {
            match root_ptr {
                Le(mut leaf_box) => {
                    while !leaf_box.bb.is_containing(vc)
                        && (leaf_box.get_values_num_inside() == 1
                            || leaf_box.bb.br <= self.br_limit)
                    {
                        leaf_box.bb.self_expand(vc);
                    }
                    if leaf_box.bb.br > self.br_limit {
                        let new_root_box = Internal::new_with_leaf_replacement(leaf_box);

                        let (new_root_box, new_created_nodes) =
                            Self::expand_root_internal(new_root_box, vc);
                        self.nodes_num += new_created_nodes + 1;
                        self.bb.clone_from(&new_root_box.bb);
                        self.root = Some(NodeBox::In(new_root_box));
                    } else {
                        self.bb.clone_from(&leaf_box.bb);
                        self.root = Some(NodeBox::Le(leaf_box));
                    }
                }
                In(internal_box) => {
                    let (new_root_box, new_created_nodes) =
                        Self::expand_root_internal(internal_box, vc);
                    self.nodes_num += new_created_nodes;
                    self.bb.clone_from(&new_root_box.bb);
                    self.root = Some(NodeBox::In(new_root_box));
                }
            }
        } else {
            self.expand_struct_bb(i);
        }
    }
}
