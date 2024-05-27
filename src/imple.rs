use crate::{boundbox::BoundBox, colvec::ColVec, nodes::Leaf, BHTree, Fnum, Udim};

mod add;

mod sub;

impl<const D: Udim> BHTree<D> {
    pub fn new_with_vec(root_bc: &[Fnum; D], root_br: &Fnum, vals: &[[Fnum; D]]) -> Self {
        let num = vals.len();
        let mut leaf_refs: Vec<Box<Leaf<D>>> = Vec::with_capacity(num);
        for val in vals {
            let curr = Leaf::new_leaf(ColVec::new_with_arr(val));
            leaf_refs.push(Box::new(curr));
        }
        let mut temp_self = Self {
            leaf_refs,
            root: None,
            bb: BoundBox::new_with_arr(root_bc, root_br.clone()),
            count: 0,
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
