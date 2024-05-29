use std::fmt::Display;

use crate::{Fnum, Udim};

#[derive(Debug, PartialEq)]
pub struct ColVec<const D: Udim> {
    pub data: [Fnum; D],
}

impl<const D: Udim> ColVec<D> {
    pub fn new_zeros() -> Self {
        let data: [Fnum; D] = [0.0; D];
        Self { data }
    }

    pub fn new_with_arr(arr: &[Fnum; D]) -> Self {
        let data: [Fnum; D] = arr.clone();
        Self { data }
    }

    pub fn new_with_runtime_arr(arr: &[Fnum]) -> Self {
        let mut data: [Fnum; D] = [0.0; D];
        for (val0, val1) in data.iter_mut().zip(arr.iter()) {
            *val0 = *val1;
        }
        Self { data }
    }

    // pub fn set_from_sub(&mut self, a: &Self, b: &Self) -> Fnum {
    //     let mut length: Fnum = 0.0;
    //     for i in 0..D {
    //         let diff = a.data[i] - b.data[i];
    //         self.data[i] = diff;
    //         length += diff * diff;
    //     }
    //     length
    // }

    pub fn add_from_arr(&mut self, other: &[Fnum; D]) {
        for i in 0..D {
            self.data[i] += other[i];
            if self.data[i].is_infinite() {
                panic!("!!!")
            }
        }
    }

    pub fn add_colvec_to_self(&mut self, other: &Self) {
        for i in 0..D {
            // println!(
            //     "{:?} {:?} {:?}",
            //     self.data[i],
            //     other.data[i],
            //     self.data[i] + other.data[i]
            // );
            self.data[i] += other.data[i];
            if self.data[i].is_infinite() {
                panic!("!!!")
            }
        }
    }

    pub fn sub_colvec_from_self(&mut self, other: &Self) {
        for i in 0..D {
            // println!(
            //     "{:?} {:?} {:?}",
            //     self.data[i],
            //     other.data[i],
            //     self.data[i] + other.data[i]
            // );
            self.data[i] -= other.data[i];
            if self.data[i].is_infinite() {
                panic!("!!!")
            }
        }
    }

    pub fn mul_scalar(&mut self, s: Fnum) {
        if s.is_infinite() || s.is_nan() {
            return;
        }
        for i in 0..D {
            self.data[i] *= s;
        }
    }

    pub fn norm2(&self, other: &[Fnum]) -> Fnum {
        let mut dis = 0.0;
        for (val0, val1) in self.data.iter().zip(other.iter()) {
            if val0 > val1 {
                dis += (val0 - val1).powi(2);
            } else if val0 < val1 {
                dis += (val1 - val0).powi(2);
            }
        }
        dis
    }

    // pub fn set_all(&mut self, val: Fnum) {
    //     for curr in self.data.iter_mut() {
    //         *curr = val;
    //     }
    // }

    // pub fn set_from(&mut self, other: &Self) {
    //     for (i, curr) in self.data.iter_mut().enumerate() {
    //         *curr = other.data[i];
    //     }
    // }

    // pub fn len2(&self) -> Fnum {
    //     let mut ret: Fnum = 0.0;
    //     for curr in self.data.iter() {
    //         ret += curr * curr;
    //     }
    //     ret
    // }
}

// mod nodeval;

impl<const D: Udim> Clone for ColVec<D> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<const D: Udim> Display for ColVec<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for i in 0..(D - 1) {
            f.write_fmt(format_args!("{},", self.data[i]))?;
        }
        f.write_fmt(format_args!("{}", self.data[D - 1]))?;
        f.write_str("]")?;
        Ok(())
    }
}

// impl<const D: usize> ColVec<D> {
//     const DIM: usize = D;
//     const DIM_LEN: usize = 2_usize.pow(D as u32);

//     pub fn merge(&mut self, other: &Self) {
//         self.add_vec_to_self(other);
//     }

//     pub fn contains(&self, r: &Fnum, other: &Self) -> bool {
//         for d in 0..D {
//             let x = self.data[d];
//             let y = other.data[d];
//             if y <= x - r || y >= x + r {
//                 return false;
//             }
//         }
//         true
//     }

//     pub fn which_child_i(&self, other: &Self) -> usize {
//         let mut i: usize = 0;
//         let m: usize = 1 << (Self::DIM - 1);
//         for d in 0..D {
//             if self.data[d] > self.data[d] {
//                 i |= m >> d;
//             }
//         }
//         i
//     }

//     pub fn set_peer_box_to_child(&self, r: &Fnum, i: &usize, other: &mut Self, other_r: &mut Fnum) {
//         todo!()
//     }

//     pub fn set_peer_box_to_parent(
//         &self,
//         r: &Fnum,
//         i: &usize,
//         other: &mut Self,
//         other_r: &mut Fnum,
//     ) {
//         todo!()
//     }

//     pub fn consider_group(&self, other: &Self, count: &usize, r: &Fnum) -> bool {
//         todo!()
//     }
// }
