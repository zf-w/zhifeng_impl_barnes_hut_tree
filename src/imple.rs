use crate::{boundbox::BoundBox, colvec::ColVec, nodes::Leaf, BHTree, Fnum, Udim};

mod add;

mod sub;

impl<const D: Udim> BHTree<D> {
    pub fn new_with_arr(root_bc: &[Fnum; D], root_br: &Fnum, vals: &[[Fnum; D]]) -> Self {
        let num = vals.len();
        let mut vs: Vec<ColVec<D>> = Vec::with_capacity(num);
        let mut to_leafs: Vec<Option<(*mut Leaf<D>, usize)>> = Vec::with_capacity(num);
        for val in vals {
            vs.push(ColVec::new_with_arr(val));
            to_leafs.push(None);
        }
        let mut temp_self = Self {
            vs,
            to_leafs,
            root: None,
            bb: BoundBox::new_with_arr(root_bc, root_br.clone()),
            count: 0,
            br_limit: 0.00000001,
        };
        for i in 0..num {
            temp_self.add(&i);
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
        let mut vs: Vec<ColVec<D>> = Vec::with_capacity(num);
        let mut to_leafs: Vec<Option<(*mut Leaf<D>, usize)>> = Vec::with_capacity(num);
        for val in vals {
            vs.push(ColVec::new_with_arr(val));
            to_leafs.push(None);
        }
        let mut temp_self = Self {
            vs,
            to_leafs,
            root: None,
            bb: BoundBox::new_with_arr(root_bc, root_br.clone()),
            count: 0,
            br_limit,
        };
        for i in 0..num {
            temp_self.add(&i);
        }
        temp_self
    }

    pub fn update(&mut self, node_i: &usize, val: &[Fnum; D]) {}

    pub fn calc_force_on_node_i(
        &self,
        node_i: &usize,
        f: impl Fn(&[Fnum; D], &[Fnum; D], usize) -> [Fnum; D],
    ) -> [Fnum; D] {
        todo!()
    }
}
