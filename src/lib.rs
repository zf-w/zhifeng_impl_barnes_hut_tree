pub type Fnum = f64;
pub type Udim = usize;

mod colvec;
pub use colvec::ColVec;

mod boundbox;

use boundbox::BoundBox;

mod nodes;
use nodes::{Leaf, NodeBox};

/// # Zhifeng's implementation of Barnes-Hut Tree
pub struct BHTree<const D: Udim> {
    vs: Vec<ColVec<D>>,
    to_leafs: Vec<Option<(*mut Leaf<D>, usize)>>,

    root: Option<NodeBox<D>>,

    count: usize,
    bb: BoundBox<D>,

    br_limit: Fnum,
}

mod imple;

#[cfg(feature = "deserial")]
mod deserial;

#[cfg(feature = "deserial")]
pub use deserial::BHTreeSer;
