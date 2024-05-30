use crate::{
    boundbox::BoundBox,
    colvec::ColVec,
    nodes::{Leaf, NodeBox},
    BHTree, Fnum, Udim,
};

mod add;

mod sub;

mod calc;

impl<const D: Udim> BHTree<D> {
    #[inline]
    pub(crate) fn new_without_add(
        root_bc: &[Fnum; D],
        root_br: Fnum,
        vals: &[[Fnum; D]],
        br_limit: Fnum,
    ) -> Self {
        let num = vals.len();
        let mut vs: Vec<(ColVec<D>, Option<(*mut Leaf<D>, usize)>)> = Vec::with_capacity(num);

        for val in vals {
            vs.push((ColVec::new_with_arr(val), None));
        }
        Self {
            vs,
            root: None,
            bb: BoundBox::new_with_arr(root_bc, root_br),
            nodes_num: 0,
            br_limit,
        }
    }

    #[inline]
    pub(crate) fn set_root_leaf(&mut self, mut leaf_box: Box<Leaf<D>>) {
        leaf_box.bb.clone_from(&self.bb);
        self.nodes_num = leaf_box.get_values_num_inside();
        self.root = Some(NodeBox::Le(leaf_box));
    }
}

// #[cfg(all(test, feature = "deserial"))]
mod test;
