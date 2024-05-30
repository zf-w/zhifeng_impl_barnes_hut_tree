use crate::{
    boundbox::BoundBox,
    colvec::ColVec,
    nodes::{Leaf, NodeBox},
    BHTree, Fnum, Udim,
};

mod add;

mod sub;

impl<const D: Udim> BHTree<D> {
    #[inline]
    pub(crate) fn new(
        root_bc: &[Fnum; D],
        root_br: &Fnum,
        vals: &[[Fnum; D]],
        br_limit: Fnum,
    ) -> Self {
        let num = vals.len();
        let mut vs: Vec<ColVec<D>> = Vec::with_capacity(num);
        let mut to_leafs: Vec<Option<(*mut Leaf<D>, usize)>> = Vec::with_capacity(num);
        for val in vals {
            vs.push(ColVec::new_with_arr(val));
            to_leafs.push(None);
        }
        Self {
            vs,
            to_leafs,
            root: None,
            bb: BoundBox::new_with_arr(root_bc, root_br.clone()),
            nodes_num: 0,
            br_limit,
        }
    }
    pub fn new_with_arr(root_bc: &[Fnum; D], root_br: &Fnum, vals: &[[Fnum; D]]) -> Self {
        let mut temp_self = Self::new(root_bc, root_br, vals, 0.00000001);
        for i in 0..vals.len() {
            temp_self.add(i);
        }
        temp_self
    }

    pub fn new_with_arr_and_limit(
        root_bc: &[Fnum; D],
        root_br: &Fnum,
        vals: &[[Fnum; D]],
        br_limit: Fnum,
    ) -> Self {
        let num = vals.len();
        let mut temp_self = Self::new(root_bc, root_br, vals, br_limit);
        for i in 0..num {
            temp_self.add(i);
        }
        temp_self
    }

    pub(crate) fn set_root_leaf(&mut self, mut leaf_box: Box<Leaf<D>>) {
        leaf_box.bb.clone_from(&self.bb);
        self.nodes_num = leaf_box.get_values_num_inside();
        self.root = Some(NodeBox::Le(leaf_box));
    }

    pub fn update_value(&mut self, node_i: &usize, val: &[Fnum; D]) {}

    pub fn calc_force_on_leaf(
        &self,
        leaf_i: usize,
        f: impl Fn(&[Fnum; D], &[Fnum; D], usize) -> [Fnum; D],
    ) -> [Fnum; D] {
        todo!()
    }
}

// #[cfg(all(test, feature = "deserial"))]
mod test;
