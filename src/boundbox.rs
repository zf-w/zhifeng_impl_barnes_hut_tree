use std::fmt::Display;

use crate::{colvec::ColVec, Fnum, Udim};

#[derive(Clone, PartialEq, Debug)]
pub struct BoundBox<const D: Udim> {
    pub(crate) bc: ColVec<D>,
    pub(crate) br: Fnum,
}

impl<const D: Udim> BoundBox<D> {
    const DIM: usize = D;

    pub fn new_zeros() -> Self {
        Self {
            bc: ColVec::new_zeros(),
            br: 0.0,
        }
    }

    pub fn new_with_arr(bc: &[Fnum; D], br: Fnum) -> Self {
        Self {
            bc: ColVec::new_with_arr(bc),
            br,
        }
    }

    pub fn calc_next_dir(&self, vc: &ColVec<D>) -> usize {
        let m: usize = 1 << (Self::DIM - 1);
        let mut ans = 0;
        for d in 0..D {
            if vc.data[d] >= self.bc.data[d] {
                ans |= m >> d;
            }
        }
        ans
    }

    pub fn calc_child_bb(&self, i: &usize) -> Self {
        let mut ans: ColVec<D> = ColVec::clone(&self.bc);
        let ans_r = self.br * 0.5;

        let mask: usize = 1 << (Self::DIM - 1);
        for d in 0..D {
            ans.data[d] += if (i & mask >> d) > 0 { ans_r } else { -ans_r };
        }
        BoundBox { bc: ans, br: ans_r }
    }

    pub fn calc_reverse_expand_bb(&self, vc: &ColVec<D>) -> (Self, usize) {
        let mut ans_bc = self.bc.clone();
        let mut ans_br = self.br.clone();
        let mask: usize = 1 << (Self::DIM - 1);

        let mut dir = 0;
        for d in 0..D {
            let curr_v = &mut ans_bc.data[d];
            if vc.data[d] >= *curr_v {
                *curr_v += ans_br;
            } else {
                *curr_v -= ans_br;
                dir |= mask >> d;
            }
        }
        ans_br *= 2.0;
        (
            Self {
                bc: ans_bc,
                br: ans_br,
            },
            dir,
        )
    }

    pub fn is_containing(&self, vc: &ColVec<D>) -> bool {
        let r = self.br;
        for d in 0..D {
            let curr_c = self.bc.data[d];
            let other_c = vc.data[d];
            if other_c < curr_c - r || other_c >= curr_c + r {
                return false;
            }
        }
        true
    }

    pub fn set_self_from_parent_bb_and_dir(&mut self, parent_bb: &Self, dir: usize) {
        self.bc = parent_bb.bc.clone();
        let ans_r = parent_bb.br * 0.5;

        let mask: usize = 1 << (Self::DIM - 1);
        for d in 0..D {
            let curr_mask = mask >> d;
            self.bc.data[d] += if (dir & curr_mask) > 0 { ans_r } else { -ans_r };
        }
        self.br = ans_r;
    }
}

impl<const D: Udim> Display for BoundBox<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{ bc: [")?;
        for i in 0..(D - 1) {
            f.write_fmt(format_args!("{},", self.bc.data[i]))?;
        }
        f.write_fmt(format_args!("{}", self.bc.data[D - 1]))?;
        f.write_fmt(format_args!("], r: {}}}", self.br))?;
        Ok(())
    }
}

#[test]
fn check_calc_next_dir() {
    let bc_arr = [0.0, 0.0, 0.0];
    let bb = BoundBox::new_with_arr(&bc_arr, 2.0);
    let vc_7 = ColVec::new_with_arr(&[1.0, 1.0, 1.0]);
    assert_eq!(7, bb.calc_next_dir(&vc_7));
    let vc_3 = ColVec::new_with_arr(&[-1.0, 1.0, 1.0]);
    assert_eq!(3, bb.calc_next_dir(&vc_3));
}

#[test]
fn check_calc_child_bb() {
    let bb = BoundBox::new_with_arr(&[0.0, 0.0, 0.0], 2.0);
    let expected_bb_7 = BoundBox::new_with_arr(&[1.0, 1.0, 1.0], 1.0);
    assert_eq!(expected_bb_7, bb.calc_child_bb(&7));
    let expected_bb_3 = BoundBox::new_with_arr(&[-1.0, 1.0, 1.0], 1.0);
    assert_eq!(expected_bb_3, bb.calc_child_bb(&3));
}

#[test]
fn check_is_containing_vc() {
    let bb = BoundBox::new_with_arr(&[0.0, 0.0, 0.0], 2.0);
    let vc0 = ColVec::new_with_arr(&[3.0, 0.0, 0.0]);
    let vc1 = ColVec::new_with_arr(&[0.0, 1.0, 0.0]);
    let vc2 = ColVec::new_with_arr(&[0.0, 3.0, 0.0]);
    assert_eq!(bb.is_containing(&vc0), false);
    assert_eq!(bb.is_containing(&vc1), true);
    assert_eq!(bb.is_containing(&vc2), false);
}

#[test]
fn check_calc_reverse_bc_0() {
    let bb = BoundBox::new_with_arr(&[0.0, 0.0, 0.0], 2.0);
    let vc = ColVec::new_with_arr(&[3.0, 0.0, 0.0]);

    let (new_bb, dir) = bb.calc_reverse_expand_bb(&vc);

    assert_eq!(new_bb.bc.data, [2.0, 2.0, 2.0]);
    assert_eq!(new_bb.br, 4.0);
    assert_eq!(dir, 0);
    assert_eq!(new_bb.calc_child_bb(&dir).bc, bb.bc);
}

#[test]
fn check_calc_reverse_bc_1() {
    let bb = BoundBox::new_with_arr(&[0.0, 0.0, 0.0], 2.0);
    let vc = ColVec::new_with_arr(&[3.0, -1.0, -1.0]);

    let (new_bb, dir) = bb.calc_reverse_expand_bb(&vc);

    assert_eq!(new_bb.bc.data, [2.0, -2.0, -2.0]);
    assert_eq!(new_bb.br, 4.0);
    assert_eq!(dir, 3);
    assert_eq!(new_bb.calc_child_bb(&dir).bc, bb.bc);
}
