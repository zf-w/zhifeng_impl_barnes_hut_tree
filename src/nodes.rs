mod leaf;
pub use leaf::Leaf;

mod internal;
pub use internal::Internal;

use crate::Udim;

pub enum NodeBox<const D: Udim> {
    In(Box<Internal<D>>),
    Le(Box<Leaf<D>>),
}
