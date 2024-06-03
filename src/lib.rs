//! # Barnes-Hut Tree for accelerated N-body force calculation
//!
//!

const DEFAULT_BR_LIMIT: Fnum = 1e-8;

type Fnum = f64;
type Udim = usize;

mod colvec;

use std::collections::VecDeque;

use colvec::ColVec;

mod boundbox;

use boundbox::BoundBox;

mod nodes;
use nodes::{Leaf, NodeBox};

/// # Barnes-Hut Tree
///
/// This is Zhifeng's implementation of Barnes-Hut Tree for accelerated N-body force calculation.
pub struct BarnesHutTree<const D: Udim> {
    vs: Vec<(ColVec<D>, Option<(*mut Leaf<D>, usize)>)>,

    root: Option<NodeBox<D>>,

    nodes_num: usize,
    bb: BoundBox<D>,

    br_limit: Fnum,
}

mod imple;

impl<const D: Udim> BarnesHutTree<D> {
    pub fn new() -> Self {
        Self {
            vs: Vec::new(),
            root: None,
            bb: BoundBox::new_with_arr(&[0.0; D], 1.0),
            nodes_num: 0,
            br_limit: DEFAULT_BR_LIMIT,
        }
    }

    pub fn with_bounding_and_capacity(root_bc: &[Fnum; D], root_br: Fnum, len: usize) -> Self {
        Self::with_bounding_capacity_and_limit(root_bc, root_br, len, DEFAULT_BR_LIMIT)
    }

    pub fn with_bounding_capacity_and_limit(
        root_bc: &[Fnum; D],
        root_br: Fnum,
        len: usize,
        br_limit: Fnum,
    ) -> Self {
        let vs: Vec<(ColVec<D>, Option<(*mut Leaf<D>, usize)>)> = Vec::with_capacity(len);
        Self {
            vs,
            root: None,
            bb: BoundBox::new_with_arr(root_bc, root_br),
            nodes_num: 0,
            br_limit,
        }
    }

    pub fn with_bounding_and_values(
        root_bc: &[Fnum; D],
        root_br: Fnum,
        vals: &[[Fnum; D]],
    ) -> Self {
        let mut temp_self = Self::new_without_add(root_bc, root_br, vals, DEFAULT_BR_LIMIT);
        for i in 0..vals.len() {
            temp_self.add(i);
        }
        temp_self
    }

    pub fn with_bounding_and_values_and_limit(
        root_bc: &[Fnum; D],
        root_br: Fnum,
        vals: &[[Fnum; D]],
        br_limit: Fnum,
    ) -> Self {
        let num = vals.len();
        let mut temp_self = Self::new_without_add(root_bc, root_br, vals, br_limit);
        for i in 0..num {
            temp_self.add(i);
        }
        temp_self
    }

    pub fn calc_force_on_value<T>(
        &self,
        value_i: usize,
        is_super_node: impl Fn(&[Fnum; D], &[Fnum; D], Fnum) -> bool,
        calc_fn: impl Fn(&[Fnum; D], &[Fnum; D], usize, &mut T),
        write_to_value: &mut T,
    ) -> bool {
        if value_i >= self.vs.len() {
            return false;
        }

        let mut curr_info =
            self.calc_leaf_siblings_and_get_parent(value_i, &calc_fn, write_to_value);

        let mut q: VecDeque<&NodeBox<D>> = VecDeque::with_capacity(self.nodes_num / 2);
        let curr_v_ref = &self.vs[value_i].0.data;

        while let Some((curr_internal_ptr, curr_in_leaf_i)) = curr_info {
            let curr_internal_ref = unsafe {
                curr_internal_ptr
                    .as_ref()
                    .expect("Should work if the structure is correct")
            };
            for (in_leaf_i, node_opt) in curr_internal_ref.nexts.iter().enumerate() {
                if in_leaf_i == curr_in_leaf_i {
                    continue;
                }
                if let Some(curr_node_box_ref) = node_opt.as_ref() {
                    self.calc_node(
                        curr_v_ref,
                        curr_node_box_ref,
                        &mut q,
                        write_to_value,
                        &is_super_node,
                        &calc_fn,
                    )
                }
            }
            curr_info = curr_internal_ref.parent;
        }

        while let Some(curr_node_box_ref) = q.pop_front() {
            self.calc_node(
                curr_v_ref,
                curr_node_box_ref,
                &mut q,
                write_to_value,
                &is_super_node,
                &calc_fn,
            )
        }
        true
    }

    pub fn get(&self, value_i: usize) -> Option<&[Fnum; D]> {
        if value_i >= self.vs.len() {
            return None;
        }
        Some(&self.vs[value_i].0.data)
    }

    pub fn push(&mut self, value: &[Fnum; D]) -> usize {
        let value_i = self.vs.len();
        self.vs.push((ColVec::new_with_arr(value), None));

        self.add(value_i);
        value_i
    }

    pub fn update(&mut self, value_i: usize, value: &[Fnum; D]) -> bool {
        let len = self.vs.len();
        if value_i >= len {
            return false;
        }
        self.sub(value_i);
        self.vs[value_i].0.clone_from_arr_ref(value);
        self.add(value_i);
        true
    }

    pub fn remove(&mut self, value_i: usize) -> Option<(usize, usize)> {
        let len = self.vs.len();
        if value_i >= len {
            return None;
        }
        self.sub(value_i);
        if value_i + 1 < len {
            let last_v_opt = self.vs.pop().expect("Should have a last");
            self.vs[value_i] = last_v_opt;
            Some((len - 1, value_i))
        } else {
            None
        }
    }

    pub fn nodes_num(&self) -> usize {
        self.nodes_num
    }
}

pub mod utils;

#[cfg(feature = "serialize")]
mod serialize;

#[cfg(feature = "serialize")]
pub use serialize::BarnesHutTreeSer;
