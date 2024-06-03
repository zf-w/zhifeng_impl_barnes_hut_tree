use crate::{Fnum, Udim};

#[derive(Debug, PartialEq)]
pub struct ColVec<const D: Udim> {
    pub data: [Fnum; D],
}

impl<const D: Udim> ColVec<D> {
    #[inline]
    pub fn new_zeros() -> Self {
        let data: [Fnum; D] = [0.0; D];
        Self { data }
    }

    #[inline]
    pub fn new_with_arr(arr: &[Fnum; D]) -> Self {
        let data: [Fnum; D] = arr.clone();
        for d in 0..D {
            assert!(data[d].is_finite(), "A numeric error occurred...");
        }
        Self { data }
    }

    #[inline]
    pub fn clone_from_arr_ref(&mut self, arr_ref: &[Fnum; D]) {
        for d in 0..D {
            self.data[d].clone_from(&arr_ref[d]);
            assert!(
                self.data[d].is_finite(),
                "A numeric error occurred when updating..."
            );
        }
    }

    #[inline]
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
            assert!(
                self.data[i].is_finite(),
                "A numeric error occurred when calculating the new average value after node adding..."
            );
        }
    }

    #[inline]
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
                assert!(self.data[i].is_finite(), "A numeric error occurred when calculating the new average value after node removal...");
            }
        }
    }
}

impl<const D: Udim> Clone for ColVec<D> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}
