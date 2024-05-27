use std::fmt::Display;

use crate::{colvec::ColVec, Fnum, Udim};

#[derive(Clone, PartialEq, Debug)]
pub struct BoundBox<const D: Udim> {
    bc: ColVec<D>,
    br: Fnum,
}

impl<const D: Udim> BoundBox<D> {
    const DIM: usize = D;

    pub fn new_zeros() -> Self {
        Self {
            bc: ColVec::new(),
            br: 0.0,
        }
    }

    pub fn new_with_arr(bc: &[Fnum; D], br: Fnum) -> Self {
        Self {
            bc: ColVec::new_with_arr(bc),
            br,
        }
    }

    pub fn get_bc(&self) -> &ColVec<D> {
        &self.bc
    }

    pub fn get_br(&self) -> &Fnum {
        &self.br
    }

    pub fn calc_next_dir(&self, vc: &ColVec<D>) -> usize {
        let m: usize = 1 << (Self::DIM - 1);
        let mut ans = 0;
        for d in 0..D {
            if vc.data[d] > self.bc.data[d] {
                ans |= m >> d;
            }
        }
        ans
    }

    pub fn calc_child_bb(&self, i: &usize) -> Self {
        let mut ans: ColVec<D> = ColVec::clone(&self.bc);
        let ans_r = self.br * 0.5;

        let m: usize = 1 << (Self::DIM - 1);
        for d in 0..D {
            ans.data[d] += if (i & m >> d) > 0 { ans_r } else { -ans_r };
        }
        BoundBox { bc: ans, br: ans_r }
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
