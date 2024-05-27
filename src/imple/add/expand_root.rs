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
    /// # Check if the root bounding box contains the leaf value center
    pub fn expand_root(&mut self, leaf_vc: &ColVec<D>) {
        todo!()
    }
}
