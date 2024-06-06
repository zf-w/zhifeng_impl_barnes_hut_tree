use crate::{
    imple::{get_mut_ref_from_arr_mut_ref, get_ref_from_arr_ref},
    nodes::{
        Internal,
        NodeIndex::{self, In, Le},
    },
    BarnesHutTree, ColVec, Udim,
};

impl<const D: Udim> BarnesHutTree<D> {
    #[inline]
    fn expand_struct_bb(&mut self, value_i: usize) {
        let vc = &get_ref_from_arr_ref(&self.vs, value_i, "For updating struct bb").0;

        while !self.bb.is_containing(vc) {
            self.bb.self_expand(vc);
        }
    }

    #[inline]
    fn expand_root_internal(&mut self, mut root_i: usize, vc: &ColVec<D>) -> usize {
        while !get_mut_ref_from_arr_mut_ref(
            self.internal_vec.as_mut(),
            root_i,
            "To expand the root internal node",
        )
        .bb
        .is_containing(vc)
        {
            let next_i = self.internal_vec.len();

            let root_mut_ref = get_mut_ref_from_arr_mut_ref(
                self.internal_vec.as_mut(),
                root_i,
                "Get root node internal mut ref.",
            );

            let (mut new_root_box, dir) = root_mut_ref.calc_new_internal_with_new_vc(vc);
            new_root_box.nexts[dir] = Some(NodeIndex::In(root_i));
            root_i = next_i;
            root_mut_ref.parent = Some((root_i, dir));
            self.new_internal(new_root_box);
            // One Bug here before, creating a self loop hahaha
        }
        root_i
    }

    /// # Expand root bounding box according to new leaf value
    #[inline]
    pub(super) fn expand_root(&mut self, i: usize) {
        let vc = self.vs[i].0.clone();
        if self.bb.is_containing(&vc) {
            return;
        }
        if let Some(root_ptr) = self.root.take() {
            match root_ptr {
                Le(leaf_i) => {
                    // If the current node is a leaf node.

                    // #[cfg(not(feature = "unchecked"))]
                    let leaf_mut_ref = get_mut_ref_from_arr_mut_ref(
                        self.leaf_vec.as_mut(),
                        leaf_i,
                        "Getting root leaf node to expand its bounding box",
                    );

                    while !leaf_mut_ref.bb.is_containing(&vc)
                        && (leaf_mut_ref.get_values_num_inside() == 1
                            || leaf_mut_ref.bb.br <= self.br_limit)
                    {
                        leaf_mut_ref.bb.self_expand(&vc);
                    }
                    if leaf_mut_ref.bb.br > self.br_limit {
                        let next_i = self.internal_vec.len();
                        let new_root_box =
                            Internal::new_with_leaf_replacement(next_i, leaf_i, leaf_mut_ref);

                        self.new_internal(new_root_box);

                        let new_root_i = self.expand_root_internal(next_i, &vc);

                        let new_root_mut_ref = get_mut_ref_from_arr_mut_ref(
                            self.internal_vec.as_mut(),
                            new_root_i,
                            "Get the new root mut ref to update struct bounding box",
                        );

                        self.bb.clone_from(&new_root_mut_ref.bb);

                        self.root = Some(NodeIndex::In(new_root_i));
                    } else {
                        self.bb.clone_from(&leaf_mut_ref.bb);
                        self.root = Some(NodeIndex::Le(leaf_i));
                    }
                }
                In(internal_i) => {
                    // If the current node is an internal node

                    let new_root_i = self.expand_root_internal(internal_i, &vc);

                    // #[cfg(feature = "unchecked")]
                    let new_root_mut_ref = get_mut_ref_from_arr_mut_ref(
                        &mut self.internal_vec,
                        new_root_i,
                        "Preparing for update root internal node's bounding box",
                    );

                    self.bb.clone_from(&new_root_mut_ref.bb);
                    self.root = Some(NodeIndex::In(new_root_i));
                }
            }
        } else {
            // If there is no nodes, we only need to expand the tree's general bounding box.
            self.expand_struct_bb(i);
        }
    }
}
