pub type Fnum = f64;
pub type Udim = usize;

mod colvec;
pub use colvec::ColVec;

mod boundbox;

use boundbox::BoundBox;

mod nodes;
use nodes::{Leaf, NodePtr};

pub struct BHTree<const D: Udim> {
    leaf_refs: Vec<Box<Leaf<D>>>,
    root: Option<NodePtr<D>>,

    count: usize,
    bb: BoundBox<D>,
}

mod imple;

#[cfg(feature = "deserial")]
mod deserial;
