type Fnum = f64;
type Udim = usize;

mod colvec;

use std::collections::VecDeque;

use colvec::ColVec;

mod boundbox;

use boundbox::BoundBox;

mod nodes;
use nodes::{Leaf, NodeBox};

/// # Zhifeng's implementation of Barnes-Hut Tree
pub struct BHTree<const D: Udim> {
    vs: Vec<(ColVec<D>, Option<(*mut Leaf<D>, usize)>)>,

    root: Option<NodeBox<D>>,

    nodes_num: usize,
    bb: BoundBox<D>,

    br_limit: Fnum,
}

mod imple;

const DEFAULT_BR_LIMIT: Fnum = 0.00000001;

impl<const D: Udim> BHTree<D> {
    pub fn with_capacity(root_bc: &[Fnum; D], root_br: Fnum, len: usize) -> Self {
        Self::with_capacity_and_limit(root_bc, root_br, len, DEFAULT_BR_LIMIT)
    }

    pub fn with_capacity_and_limit(
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

    pub fn new_with_values(root_bc: &[Fnum; D], root_br: Fnum, vals: &[[Fnum; D]]) -> Self {
        let mut temp_self = Self::new_without_add(root_bc, root_br, vals, 0.00000001);
        for i in 0..vals.len() {
            temp_self.add(i);
        }
        temp_self
    }

    pub fn new_with_values_and_limit(
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

    pub fn calc_force_on_value(
        &self,
        value_i: usize,
        is_super_node: impl Fn(&[Fnum; D], &[Fnum; D], Fnum) -> bool,
        calc_fn: impl Fn(&[Fnum; D], &[Fnum; D], usize, &mut [Fnum; D]),
        mut write_to_value: &mut [Fnum; D],
    ) -> bool {
        if value_i >= self.vs.len() {
            return false;
        }

        let mut curr_info =
            self.calc_leaf_siblings_and_get_parent(value_i, &calc_fn, &mut write_to_value);

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
                        &mut write_to_value,
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
                &mut write_to_value,
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

const DEFAULT_EPSILON: Fnum = 0.000000001;

fn calc_v0_to_v1_diff<const D: Udim>(v0: &[Fnum; D], v1: &[Fnum; D]) -> [Fnum; D] {
    let mut ans = [0.0; D];
    for d in 0..D {
        ans[d] = v1[d] - v0[d];
    }
    ans
}

fn calc_l2_norm<const D: Udim>(v: &[Fnum; D]) -> Fnum {
    let mut ans_f64 = 0.0;
    for curr_v in v.iter() {
        ans_f64 += curr_v * curr_v;
    }
    ans_f64
}

pub fn factory_of_repulsive_displacement_calc_fn<const D: Udim>(
    k: Fnum,
    c: Fnum,
) -> impl Fn(&[Fnum; D], &[Fnum; D], usize, &mut [Fnum; D]) {
    move |curr_v_ref: &[Fnum; D],
          v_center: &[Fnum; D],
          num: usize,
          ans_v_mut_ref: &mut [Fnum; D]| {
        let diff = calc_v0_to_v1_diff(v_center, curr_v_ref);
        let dis_pow2 = calc_l2_norm(&diff);
        let dis_pow2 = if dis_pow2.is_finite() && dis_pow2 > DEFAULT_EPSILON {
            dis_pow2
        } else {
            DEFAULT_EPSILON
        };
        let scalar = num as Fnum * k * k * c / dis_pow2;
        for d in 0..D {
            ans_v_mut_ref[d] += diff[d] * scalar;
        }
    }
}

pub fn factory_of_is_super_node_fn<const D: Udim>(
    theta: Fnum,
) -> impl Fn(&[Fnum; D], &[Fnum; D], Fnum) -> bool {
    move |curr_v_ref: &[Fnum; D], super_bc_ref: &[Fnum; D], super_half_w: Fnum| -> bool {
        let diff = calc_v0_to_v1_diff(curr_v_ref, super_bc_ref);
        let dis_pow2 = calc_l2_norm(&diff);
        ((super_half_w * 2.0)
            / if dis_pow2.is_finite() && dis_pow2 > DEFAULT_EPSILON {
                dis_pow2.sqrt()
            } else {
                DEFAULT_EPSILON.sqrt()
            })
            <= theta
    }
}

// #[cfg(feature = "deserial")]
mod deserial;

// #[cfg(feature = "deserial")]
pub use deserial::{assert_bht_serde_eq, BHTreeSer};
