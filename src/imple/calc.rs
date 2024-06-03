use std::collections::VecDeque;

use crate::{
    nodes::{Internal, Leaf, NodeIndex},
    BarnesHutTree, Fnum, Udim,
};

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
                self.internal_vec[*internal_i_ref].as_ref(),
                q,
                write_to,
                &calc_this,
                calc_fn,
            ),
            NodeIndex::Le(leaf_i_ref) => self.calc_neighbour_leaf(
                curr_v_ref,
                self.leaf_vec[*leaf_i_ref].as_ref(),
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
        if calc_this(curr_v_ref, &internal_ref.bb.bc.data, internal_ref.bb.br) {
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
        if calc_this(curr_v_ref, &leaf_ref.bb.bc.data, leaf_ref.bb.br) {
            calc_fn(
                curr_v_ref,
                &leaf_ref.vc.data,
                leaf_ref.get_values_num_inside(),
                write_to,
            );
        } else {
            for value_i_ref in leaf_ref.vs.iter() {
                calc_fn(
                    curr_v_ref,
                    &self.vs[*value_i_ref].0.data,
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
        let (curr_leaf_i, curr_in_leaf_i) = self.vs[value_i].1.expect("Should always have");
        let curr_leaf_ref = self.leaf_vec[curr_leaf_i].as_ref();
        let curr_v_ref = &self.vs[value_i].0.data;
        for (in_leaf_i, other_value_i) in curr_leaf_ref.vs.iter().enumerate() {
            if in_leaf_i == curr_in_leaf_i {
                continue;
            }
            calc_fn(curr_v_ref, &self.vs[*other_value_i].0.data, 1, write_to)
        }
        curr_leaf_ref.parent
    }
}
