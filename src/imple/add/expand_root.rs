use crate::{
    nodes::{
        Internal,
        NodeIndex::{self, In, Le},
    },
    BarnesHutTree, ColVec, Udim,
};

impl<const D: Udim> BarnesHutTree<D> {
    #[inline]
    fn expand_struct_bb(&mut self, i: usize) {
        let vc = &self.vs[i].0;

        while !self.bb.is_containing(vc) {
            self.bb.self_expand(vc);
        }
    }

    #[inline]
    fn expand_root_internal(&mut self, mut root_i: usize, vc: &ColVec<D>) -> usize {
        while !{
            #[cfg(not(feature = "unchecked"))]
            {
                self.internal_vec
                    .get_mut(root_i) // One bug here, should be new_i instead of root_i
                    .unwrap()
                    .as_mut()
                    .bb
                    .is_containing(vc)
            }
            #[cfg(feature = "unchecked")]
            {
                unsafe {
                    self.internal_vec
                        .get_unchecked_mut(root_i) // One bug here, should be new_i instead of root_i
                        .as_mut()
                        .bb
                        .is_containing(vc)
                }
            }
        } {
            let next_i = self.internal_vec.len();
            #[cfg(not(feature = "unchecked"))]
            let root_mut_ref = self.internal_vec.get_mut(root_i).unwrap().as_mut();
            #[cfg(feature = "unchecked")]
            let root_mut_ref = unsafe { self.internal_vec.get_unchecked_mut(root_i).as_mut() };

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

                    #[cfg(not(feature = "unchecked"))]
                    let leaf_mut_ref = self
                        .leaf_vec
                        .get_mut(leaf_i)
                        .expect("Getting root leaf node to expand its bounding box")
                        .as_mut();
                    #[cfg(feature = "unchecked")]
                    let leaf_mut_ref = unsafe { self.leaf_vec.get_unchecked_mut(leaf_i).as_mut() };

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

                        #[cfg(feature = "unchecked")]
                        let new_root_mut_ref =
                            unsafe { self.internal_vec.get_unchecked_mut(new_root_i).as_mut() };
                        #[cfg(not(feature = "unchecked"))]
                        let new_root_mut_ref =
                            self.internal_vec.get_mut(new_root_i).unwrap().as_mut();

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

                    #[cfg(feature = "unchecked")]
                    let new_root_mut_ref =
                        unsafe { self.internal_vec.get_unchecked_mut(new_root_i).as_mut() };

                    #[cfg(not(feature = "unchecked"))]
                    let new_root_mut_ref = self
                        .internal_vec
                        .get_mut(new_root_i)
                        .expect("Preparing for update root internal node's bounding box")
                        .as_mut();

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
