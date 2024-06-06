use crate::{boundbox::BoundBox, colvec::ColVec, Udim};

use super::Internal;

pub struct Leaf<const D: Udim> {
    pub(crate) parent: Option<(usize, usize)>,

    pub(crate) bb: BoundBox<D>,

    pub(crate) vc: ColVec<D>,
    pub(crate) vs: Vec<usize>,
}

impl<const D: Udim> Leaf<D> {
    pub fn new_empty_from_bb(bb: BoundBox<D>) -> Box<Self> {
        let vc: ColVec<D> = ColVec::new_zeros();
        let vs: Vec<usize> = Vec::with_capacity(1);
        let parent = None;
        Box::new(Self { parent, bb, vc, vs })
    }

    pub fn new_empty_from_parent_dir(
        parent: &mut Internal<D>,
        parent_i: usize,
        dir: usize,
    ) -> Box<Self> {
        let bb = parent.calc_child_bb(&dir);

        let vc: ColVec<D> = ColVec::new_zeros();
        let vs: Vec<usize> = Vec::with_capacity(1);
        let parent = Some((parent_i, dir));
        Box::new(Self { parent, bb, vc, vs })
    }

    pub fn add_value(&mut self, leaf_i: usize, v: &ColVec<D>) -> usize {
        let i = self.vs.len();
        self.vc.update_online_average_with_one_new_data(i, &v.data);
        self.vs.push(leaf_i);
        i
    }

    pub fn sub_value(&mut self, child_i: usize, v: &ColVec<D>) -> usize {
        let len = self.vs.len();
        self.vc
            .update_online_average_with_one_data_removal(len, &v.data);

        if child_i + 1 < len {
            let last_v = self.vs.last().expect("Check length before").clone();
            self.vs[child_i].clone_from(&last_v);
            self.vs.pop(); // Forgot to pop
            last_v
        } else {
            self.vs.pop();
            0
        }
    }

    pub fn get_values_num_inside(&self) -> usize {
        self.vs.len()
    }

    pub fn set_parent(&mut self, parent_i: usize, parent_ref: &Internal<D>, from_dir: usize) {
        self.parent = Some((parent_i, from_dir));

        self.bb
            .set_self_from_parent_bb_and_dir(&parent_ref.bb, from_dir);
    }
}
