use std::collections::VecDeque;

use crate::{
    nodes::{Internal, Leaf, NodeBox},
    BHTree, Fnum, Udim,
};

impl<const D: Udim> BHTree<D> {
    pub(crate) fn calc_neighbour_internal<'o>(
        &'o self,
        curr_v_ref: &[Fnum; D],
        internal_ref: &'o Internal<D>,
        q: &mut VecDeque<&'o NodeBox<D>>,
        ans_v_mut_ref: &mut [Fnum; D],
        calc_this: impl Fn(&[Fnum; D], &[Fnum; D], Fnum) -> bool,
        calc_fn: impl Fn(&[Fnum; D], &[Fnum; D], usize, &mut [Fnum; D]),
    ) {
        if calc_this(curr_v_ref, &internal_ref.bb.bc.data, internal_ref.bb.br) {
            calc_fn(
                curr_v_ref,
                &internal_ref.vc.data,
                internal_ref.get_values_num_inside(),
                ans_v_mut_ref,
            );
        } else {
            for node_box_opt_ref in internal_ref.nexts.iter() {
                if let Some(node_box_ref) = node_box_opt_ref {
                    q.push_back(node_box_ref);
                }
            }
        }
    }
    pub(crate) fn calc_neighbour_leaf(
        &self,
        curr_v_ref: &[Fnum; D],
        leaf_ref: &Leaf<D>,
        ans_v_mut_ref: &mut [Fnum; D],
        calc_this: impl Fn(&[Fnum; D], &[Fnum; D], Fnum) -> bool,
        calc_fn: impl Fn(&[Fnum; D], &[Fnum; D], usize, &mut [Fnum; D]),
    ) {
        if calc_this(curr_v_ref, &leaf_ref.bb.bc.data, leaf_ref.bb.br) {
            calc_fn(
                curr_v_ref,
                &leaf_ref.vc.data,
                leaf_ref.get_values_num_inside(),
                ans_v_mut_ref,
            );
        } else {
            for value_i_ref in leaf_ref.vs.iter() {
                calc_fn(
                    curr_v_ref,
                    &self.vs[*value_i_ref].0.data,
                    leaf_ref.get_values_num_inside(),
                    ans_v_mut_ref,
                );
            }
        }
    }

    pub(crate) fn calc_leaf_siblings_and_get_parent(
        &self,
        value_i: usize,
        calc_fn: impl Fn(&[Fnum; D], &[Fnum; D], usize, &mut [Fnum; D]),
        ans_v_mut_ref: &mut [Fnum; D],
    ) -> Option<(*mut Internal<D>, usize)> {
        let (curr_leaf_ptr, curr_in_leaf_i) = self.vs[value_i].1.expect("Should always have");
        let curr_leaf_ref = unsafe { curr_leaf_ptr.as_ref().expect("Should be valid") };
        let curr_v_ref = &self.vs[value_i].0.data;
        for (in_leaf_i, other_value_i) in curr_leaf_ref.vs.iter().enumerate() {
            if in_leaf_i == curr_in_leaf_i {
                continue;
            }
            calc_fn(
                curr_v_ref,
                &self.vs[*other_value_i].0.data,
                1,
                ans_v_mut_ref,
            )
        }
        curr_leaf_ref.parent
    }
}
