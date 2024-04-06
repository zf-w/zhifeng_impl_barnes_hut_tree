mod leaf;

mod internal;

pub use internal::Internal;
pub use leaf::Leaf;

use crate::Udim;

pub enum NodePtr<const D: Udim> {
    In(Box<Internal<D>>),
    Le(*mut Leaf<D>),
}

// struct InternalData<const D: Udim> {
//     nexts: Vec<Option<NodePtr<D>>>,
//     count: usize,
// }

// enum Data<const D: Udim> {
//     Leaf,
//     In(InternalData<D>),
// }

// pub struct Node<const D: Udim> {
//     parent: Option<(*mut Self, usize)>,
//     data: Data<D>,

//     r: Fnum,
//     vc: ColVec<D>,
//     bc: ColVec<D>,
// }
