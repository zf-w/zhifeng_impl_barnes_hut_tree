//! # A module of helper calculation function factories
//!
//! This module provides some implementations of repulsive displacement and energy calculation function factories.
//!
//! The output closures from the factory functions are from Hu, Y. (2005). Efficient, high-quality force-directed graph drawing. _Mathematica journal, 10_(1), 37-71, mentioned on the main page of the crate. These functions are designed to calculate the repulsive forces and, via force simulation, find nice graph node positions.
//!
//! ## When a distance gets too close
//!
//! If a distance falls under `1e-8`, the closure will use `1e-8` to proceed with the calculations.

use crate::{Fnum, Udim};

const DEFAULT_MIN_DIS: Fnum = 1e-8;

fn calc_v0_to_v1_diff<const D: Udim>(v0: &[Fnum; D], v1: &[Fnum; D]) -> [Fnum; D] {
    let mut ans = [0.0; D];
    for d in 0..D {
        ans[d] = v1[d] - v0[d];
    }
    ans
}

fn calc_sum_of_squared<const D: Udim>(v: &[Fnum; D]) -> Fnum {
    let mut ans_f64 = 0.0;
    for curr_v in v.iter() {
        ans_f64 += curr_v * curr_v;
    }
    ans_f64
}

/// This function is the factory of the repulsive displacement calculation function.
///
/// The function returns a closure defined by parameters `k` and `c`.
///
/// The returned closure takes the position of the target value, the mean position of a group of values, the size of the group, and the to-calculate answer's mutable reference.
///
/// The returned closure updates the third argument representing the answer displacement when calling it with the position of the target value, the average position of the values, and the number of values of a "far" super node.
///
/// The repulsive force between the target value and the super node is the number of values contained in the super node times `c` and the square of `k` divided by the distance between the value and the average center of the values in the super node. The repulsive displacement in one update is, therefore, the force times the direction of the super node to the target value, which is the vector of target value minus super node value center times the number of values contained in the super node times `c` and square of `k` divided by the distance squared.
///
/// ## Example
///
/// ```rust
/// use zhifeng_impl_barnes_hut_tree as zbht;
///
/// let k = 1.0;
/// let c = 0.2;
///
/// let calc_fn = zbht::utils::factory_of_repulsive_displacement_calc_fn::<2>(k, c);
///
/// let mut ans_displacement = [0.0;2];
/// calc_fn(&[-1.0,0.0],&[1.0,0.0],1, &mut ans_displacement);
///
/// let diff = -2.0;
/// let dis = 2.0;
///
/// assert_eq!(ans_displacement, [(diff * k * k * c) / (dis * dis), 0.0]);
/// ```
///
pub fn factory_of_repulsive_displacement_calc_fn<const D: Udim>(
    k: Fnum,
    c: Fnum,
) -> impl Fn(&[Fnum; D], &[Fnum; D], usize, &mut [Fnum; D]) {
    move |curr_v_ref: &[Fnum; D],
          other_vc_ref: &[Fnum; D],
          num: usize,
          ans_mut_ref: &mut [Fnum; D]| {
        let diff = calc_v0_to_v1_diff(other_vc_ref, curr_v_ref);
        let dis_pow2 = calc_sum_of_squared(&diff);
        let dis_pow2 = if dis_pow2.is_finite() && dis_pow2 > DEFAULT_MIN_DIS {
            dis_pow2
        } else {
            DEFAULT_MIN_DIS
        };
        let scalar = num as Fnum * k * k * c / dis_pow2;
        for d in 0..D {
            ans_mut_ref[d] += diff[d] * scalar;
        }
    }
}

///
/// This function is the factory of the repulsive displacement and energy calculation function.
///
/// The function returns a closure defined by parameter `k` and `c`.
///
/// In addition to the previous function [factory_of_repulsive_displacement_calc_fn], this function's returned closure also updates the "energy", the sum of squared forces on the target value.
///
/// ## Example
///
/// ```rust
/// use zhifeng_impl_barnes_hut_tree as zbht;
///
/// let k = 1.0;
/// let c = 0.2;
///
/// let calc_fn = zbht::utils::factory_of_repulsive_displacement_with_energy_calc_fn::<2>(k, c);
///
/// let mut ans_displacement_and_energy = ([0.0;2],0.0);
///
/// let super_node_size = 1;
///
/// calc_fn(&[-1.0,0.0],&[1.0,0.0],super_node_size, &mut ans_displacement_and_energy);
///
/// let diff: f64 = -2.0;
/// let dis: f64 = 2.0;
///
/// assert_eq!(ans_displacement_and_energy, ([(diff * k * k * c) / (dis * dis), 0.0],(super_node_size as f64 * k * k * c).powi(2) / (dis.powi(2))));
/// ```
///
pub fn factory_of_repulsive_displacement_with_energy_calc_fn<const D: Udim>(
    k: Fnum,
    c: Fnum,
) -> impl Fn(&[Fnum; D], &[Fnum; D], usize, &mut ([Fnum; D], Fnum)) {
    move |curr_v_ref: &[Fnum; D],
          other_vc_ref: &[Fnum; D],
          num: usize,
          ans_mut_ref: &mut ([Fnum; D], Fnum)| {
        let diff = calc_v0_to_v1_diff(other_vc_ref, curr_v_ref);
        let dis_pow2 = calc_sum_of_squared(&diff);
        let dis_pow2 = if dis_pow2.is_finite() && dis_pow2 > DEFAULT_MIN_DIS {
            dis_pow2
        } else {
            DEFAULT_MIN_DIS
        };
        let num_fnum = num as Fnum;
        ans_mut_ref.1 += (num_fnum * k * k * c).powi(2) / dis_pow2;
        let scalar = num_fnum * k * k * c / dis_pow2;
        for d in 0..D {
            ans_mut_ref.0[d] += diff[d] * scalar;
        }
    }
}

///
/// This function is the factory of is-super-dode(is far enough) function.
///
/// This function returns a closure defined by parameter `theta`. The closure will return `true` if the width of a super cube divided by the distance between the current value and the average position of values in the super node is less than `theta.`
///
/// ## Example
///
/// ```rust
/// use zhifeng_impl_barnes_hut_tree as zbht;
///
/// let theta = 1.2;
///
/// let is_super_fn = zbht::utils::factory_of_is_super_node_fn::<2>(theta);
///
/// let half_width = 5.0;
/// let dis = 5.0;
/// assert_eq!(is_super_fn(&[0.0,0.0],&[4.0,3.0], half_width), (2.0 * half_width / dis) <= theta);
/// ```
///
pub fn factory_of_is_super_node_fn<const D: Udim>(
    theta: Fnum,
) -> impl Fn(&[Fnum; D], &[Fnum; D], Fnum) -> bool {
    move |curr_v_ref: &[Fnum; D], super_bc_ref: &[Fnum; D], super_half_w: Fnum| -> bool {
        let diff = calc_v0_to_v1_diff(curr_v_ref, super_bc_ref);
        let dis_pow2 = calc_sum_of_squared(&diff);
        let dis = if dis_pow2.is_finite() && dis_pow2 > DEFAULT_MIN_DIS {
            dis_pow2.sqrt()
        } else {
            DEFAULT_MIN_DIS.sqrt()
        };
        ((super_half_w * 2.0) / dis) <= theta
    }
}
