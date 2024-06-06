mod leaf;
pub use leaf::Leaf;

mod internal;
pub use internal::Internal;

pub enum NodeIndex {
    In(usize),
    Le(usize),
}
