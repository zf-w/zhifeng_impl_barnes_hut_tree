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

    pub fn update_online_average_with_one_new_data(
        &mut self,
        curr_self_count: usize,
        other: &[Fnum; D],
    ) {
        let curr_self_count = curr_self_count as Fnum;
        let next_self_count = curr_self_count + 1.0;
        for i in 0..D {
            self.data[i] =
                self.data[i] * (curr_self_count / next_self_count) + other[i] / next_self_count;
            debug_assert!(self.data[i].is_finite());
        }
    }

    pub fn update_online_average_with_one_data_removal(
        &mut self,
        curr_self_count: usize,
        other: &[Fnum; D],
    ) {
        let curr_self_count = curr_self_count as Fnum;
        let prev_self_count = curr_self_count - 1.0;
        if prev_self_count == 0.0 {
            for i in 0..D {
                self.data[i] = 0.0;
            }
        } else {
            for i in 0..D {
                self.data[i] = (self.data[i] - other[i] / curr_self_count)
                    * (curr_self_count / prev_self_count);
                debug_assert!(self.data[i].is_finite());
            }
        }
    }

    pub fn add_from_arr(&mut self, other: &[Fnum; D]) {
        for i in 0..D {
            debug_assert!(other[i].is_finite());
            self.data[i] += other[i];
        }
    }

    pub fn add_colvec_to_self(&mut self, other: &Self) {
        for i in 0..D {
            debug_assert!(other.data[i].is_finite());
            self.data[i] += other.data[i];
        }
    }

    pub fn sub_colvec_from_self(&mut self, other: &Self) {
        for i in 0..D {
            self.data[i] -= other.data[i];
            debug_assert!(self.data[i].is_finite())
        }
    }

    pub fn clone_from_arr_ref(&mut self, arr_ref: &[Fnum; D]) {
        for d in 0..D {
            self.data[d].clone_from(&arr_ref[d]);
            debug_assert!(self.data[d].is_finite());
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
}

impl<const D: Udim> Clone for ColVec<D> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}
