use crate::{nodes::Leaf, BHTree, Udim};

mod expand_root;
mod find_pointer_to_add;

impl<const D: Udim> BHTree<D> {
    fn back_to_root_and_update_values(
        &mut self,
        value_i: usize,
        leaf_mut_ref: &mut Leaf<D>,
        new_leaf: bool,
    ) {
        let curr_value_ref = &self.vs[value_i];

        let mut parent_opt = leaf_mut_ref.parent;
        while let Some((parent_ptr, _)) = parent_opt {
            let parent_mut_ref = unsafe {
                parent_ptr
                    .as_mut()
                    .expect("Should be valid if the opt is Some")
            };

            parent_mut_ref.add_value(curr_value_ref);
            if new_leaf {
                parent_mut_ref.leaf_count += 1;
            }
            parent_opt = parent_mut_ref.parent;
        }
    }

    /// # Add a node into the tree
    pub fn add(&mut self, value_i: usize) {
        self.expand_root(value_i);

        let (leaf_ptr, new_leaf) = self.find_leaf_to_add_value(value_i);

        let leaf_mut_ref = unsafe { leaf_ptr.as_mut().expect("The pointer position to add") };

        let id = leaf_mut_ref.add_value(value_i, &self.vs[value_i]); // One bug here, storing pointer pointing to stack.

        self.to_leafs[value_i] = Some((leaf_ptr, id));

        self.back_to_root_and_update_values(value_i, leaf_mut_ref, new_leaf);
    }
}
