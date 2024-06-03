use crate::{Fnum, Udim};

const DEFAULT_EPSILON: Fnum = 1e-9;

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
          other_vc_ref: &[Fnum; D],
          num: usize,
          ans_mut_ref: &mut [Fnum; D]| {
        let diff = calc_v0_to_v1_diff(other_vc_ref, curr_v_ref);
        let dis_pow2 = calc_l2_norm(&diff);
        let dis_pow2 = if dis_pow2.is_finite() && dis_pow2 > DEFAULT_EPSILON {
            dis_pow2
        } else {
            DEFAULT_EPSILON
        };
        let scalar = num as Fnum * k * k * c / dis_pow2;
        for d in 0..D {
            ans_mut_ref[d] += diff[d] * scalar;
        }
    }
}

pub fn factory_of_repulsive_displacement_with_energy_calc_fn<const D: Udim>(
    k: Fnum,
    c: Fnum,
) -> impl Fn(&[Fnum; D], &[Fnum; D], usize, &mut ([Fnum; D], Fnum)) {
    move |curr_v_ref: &[Fnum; D],
          other_vc_ref: &[Fnum; D],
          num: usize,
          ans_mut_ref: &mut ([Fnum; D], Fnum)| {
        let diff = calc_v0_to_v1_diff(other_vc_ref, curr_v_ref);
        let dis_pow2 = calc_l2_norm(&diff);
        let dis = dis_pow2.sqrt();
        let dis_pow2 = if dis_pow2.is_finite() && dis_pow2 > DEFAULT_EPSILON {
            dis_pow2
        } else {
            DEFAULT_EPSILON
        };
        let num_f = num as Fnum;
        ans_mut_ref.1 += num_f * k * k * c / dis;
        let scalar = num_f * k * k * c / dis_pow2;
        for d in 0..D {
            ans_mut_ref.0[d] += diff[d] * scalar;
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
