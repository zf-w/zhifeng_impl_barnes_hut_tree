use std::collections::VecDeque;

use crate::{
    nodes::{Internal, Leaf, NodeIndex},
    BarnesHutTree, Fnum, Udim,
};

use super::get_ref_from_arr_ref;

impl<const D: Udim> BarnesHutTree<D> {
    #[inline]
    pub(crate) fn calc_node<'o, T>(
        &'o self,
        curr_v_ref: &[Fnum; D],
        node_box_ref: &'o NodeIndex,
        q: &mut VecDeque<&'o NodeIndex>,
        write_to: &mut T,
        calc_this: impl Fn(&[Fnum; D], &[Fnum; D], Fnum) -> bool,
        calc_fn: impl Fn(&[Fnum; D], &[Fnum; D], usize, &mut T),
    ) {
        match node_box_ref {
            NodeIndex::In(internal_i_ref) => self.calc_neighbour_internal(
                curr_v_ref,
                get_ref_from_arr_ref(&self.internal_vec, *internal_i_ref, "Calculate internal"),
                q,
                write_to,
                &calc_this,
                calc_fn,
            ),
            NodeIndex::Le(leaf_i_ref) => self.calc_neighbour_leaf(
                curr_v_ref,
                get_ref_from_arr_ref(&self.leaf_vec, *leaf_i_ref, "Calculate leaf"),
                write_to,
                &calc_this,
                calc_fn,
            ),
        }
    }

    pub(crate) fn calc_neighbour_internal<'o, T>(
        &'o self,
        curr_v_ref: &[Fnum; D],
        internal_ref: &'o Internal<D>,
        q: &mut VecDeque<&'o NodeIndex>,
        write_to: &mut T,
        calc_this: impl Fn(&[Fnum; D], &[Fnum; D], Fnum) -> bool,
        mut calc_fn: impl FnMut(&[Fnum; D], &[Fnum; D], usize, &mut T),
    ) {
        if calc_this(curr_v_ref, &internal_ref.vc.data, internal_ref.bb.br) {
            calc_fn(
                curr_v_ref,
                &internal_ref.vc.data,
                internal_ref.get_values_num_inside(),
                write_to,
            );
        } else {
            for node_box_opt_ref in internal_ref.nexts.iter() {
                if let Some(node_box_ref) = node_box_opt_ref {
                    q.push_back(node_box_ref);
                }
            }
        }
    }
    pub(crate) fn calc_neighbour_leaf<T>(
        &self,
        curr_v_ref: &[Fnum; D],
        leaf_ref: &Leaf<D>,
        write_to: &mut T,
        calc_this: impl Fn(&[Fnum; D], &[Fnum; D], Fnum) -> bool,
        calc_fn: impl Fn(&[Fnum; D], &[Fnum; D], usize, &mut T),
    ) {
        if calc_this(curr_v_ref, &leaf_ref.vc.data, leaf_ref.bb.br) {
            calc_fn(
                curr_v_ref,
                &leaf_ref.vc.data,
                leaf_ref.get_values_num_inside(),
                write_to,
            );
        } else {
            for value_i in leaf_ref.vs.iter().cloned() {
                calc_fn(
                    curr_v_ref,
                    &get_ref_from_arr_ref(&self.vs, value_i, "Calculating direct in-leaf values due to the current leaf is not far enough").0.data,
                    leaf_ref.get_values_num_inside(),
                    write_to,
                );
            }
        }
    }

    pub(crate) fn calc_leaf_siblings_and_get_parent<T>(
        &self,
        value_i: usize,
        mut calc_fn: impl FnMut(&[Fnum; D], &[Fnum; D], usize, &mut T),
        write_to: &mut T,
    ) -> Option<(usize, usize)> {
        let (curr_leaf_i, curr_in_leaf_i) =
            get_ref_from_arr_ref(&self.vs, value_i, "Getting value")
                .1
                .expect(
                    "A value should always have a parent except during the process of updating",
                );
        let curr_leaf_ref = get_ref_from_arr_ref(
            &self.leaf_vec,
            curr_leaf_i,
            "Getting the target value's parent leaf",
        );
        let curr_v_ref = &get_ref_from_arr_ref(&self.vs, value_i, "Getting target value")
            .0
            .data;
        for (in_leaf_i, other_value_i) in curr_leaf_ref.vs.iter().enumerate() {
            if in_leaf_i == curr_in_leaf_i {
                continue;
            }
            calc_fn(
                curr_v_ref,
                &get_ref_from_arr_ref(&self.vs, *other_value_i, "Getting same-leaf values")
                    .0
                    .data,
                1,
                write_to,
            )
        }
        curr_leaf_ref.parent
    }
}
