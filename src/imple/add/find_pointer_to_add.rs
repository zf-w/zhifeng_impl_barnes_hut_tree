use crate::{
    nodes::{
        Internal, Leaf,
        NodeIndex::{self, In, Le},
    },
    BarnesHutTree, Udim,
};

impl<const D: Udim> BarnesHutTree<D> {
    /// # Find the pointer to add the leaf node
    ///
    /// We need to find the correct position to add a leaf position.
    /// First, we need find the correct direction to continue.
    /// If the final position is a leaf node, we need to insert an internal node in the middle and reinsert the two leaf nodes.
    ///
    #[inline]
    pub(super) fn find_leaf_to_add_value(&mut self, leaf_i: usize) -> usize {
        let mut prev_internal: Option<(usize, usize)> = None;

        while let Some(curr) = if let Some((prev_i, prev_dir)) = prev_internal {
            self.internal_vec.get_mut(prev_i).unwrap().nexts[prev_dir].take()
        } else {
            self.root.take()
        } {
            let target_internal_i = match curr {
                Le(curr_leaf_i) => {
                    let curr_leaf_mut_ref = self.leaf_vec[curr_leaf_i].as_mut();
                    if curr_leaf_mut_ref.bb.br <= self.br_limit {
                        *if let Some((prev_i, prev_dir)) = prev_internal {
                            &mut self.internal_vec.get_mut(prev_i).unwrap().nexts[prev_dir]
                        } else {
                            &mut self.root
                        } = Some(NodeIndex::Le(curr_leaf_i));

                        return curr_leaf_i;
                    } else {
                        let next_i = self.internal_vec.len();
                        let internal_box = Internal::new_with_leaf_replacement(
                            next_i,
                            curr_leaf_i,
                            curr_leaf_mut_ref,
                        );

                        self.new_internal(internal_box)
                    }
                }
                In(internal_i) => internal_i,
            };

            let target_internal = self
                .internal_vec
                .get_mut(target_internal_i)
                .expect("Dereferencing for add values along path")
                .as_mut();
            let leaf_vc = &self.vs[leaf_i].0;
            target_internal.add_value(leaf_vc);

            let next_dir = target_internal.calc_next_dir(leaf_vc);

            *if let Some((prev_i, prev_dir)) = prev_internal {
                &mut self.internal_vec.get_mut(prev_i).unwrap().nexts[prev_dir]
            } else {
                &mut self.root
            } = Some(NodeIndex::In(target_internal_i));
            prev_internal = Some((target_internal_i, next_dir.clone()));
        }

        if let Some((parent_internal_i, from_dir)) = prev_internal {
            let parent_internal_mut_ref = self
                .internal_vec
                .get_mut(parent_internal_i)
                .expect("For attaching node")
                .as_mut();
            let mut leaf_box = Leaf::new_empty_from_parent_dir(
                parent_internal_mut_ref,
                parent_internal_i,
                from_dir,
            );
            let ans_leaf_i = self.leaf_vec.len();
            parent_internal_mut_ref.link_leaf_to_dir(
                from_dir,
                parent_internal_i,
                ans_leaf_i,
                leaf_box.as_mut(),
            );

            self.new_leaf(leaf_box);
            // self.nodes_num += 1;

            ans_leaf_i
        } else {
            let ans_leaf_i = self.new_leaf(Leaf::new_empty_from_bb(self.bb.clone()));

            self.root = Some(NodeIndex::Le(ans_leaf_i));
            // self.nodes_num += 1;

            ans_leaf_i
        }
    }
}
