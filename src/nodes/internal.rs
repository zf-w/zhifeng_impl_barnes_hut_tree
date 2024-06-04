use crate::{boundbox::BoundBox, colvec::ColVec, Udim};

use super::{Leaf, NodeIndex};

pub struct Internal<const D: Udim> {
    pub(crate) parent: Option<(usize, usize)>,
    pub(crate) nexts: Vec<Option<NodeIndex>>,

    count: usize,
    pub(crate) vc: ColVec<D>,
    pub(crate) bb: BoundBox<D>,
}

impl<const D: Udim> Internal<D> {
    const DIM_LEN: usize = 2_usize.pow(D as u32);

    pub fn calc_next_dir(&self, vc: &ColVec<D>) -> usize {
        self.bb.calc_next_dir(vc)
    }

    pub fn calc_child_bb(&self, dir: &usize) -> BoundBox<D> {
        self.bb.calc_child_bb(dir)
    }

    pub fn add_value(&mut self, vc: &ColVec<D>) {
        self.vc
            .update_online_average_with_one_new_data(self.count, &vc.data);
        self.count += 1;
    }

    pub fn sub_value(&mut self, vc: &ColVec<D>) {
        self.vc
            .update_online_average_with_one_data_removal(self.count, &vc.data);
        self.count -= 1;
    }

    pub fn calc_new_internal_with_new_vc(&self, vc: &ColVec<D>) -> (Box<Self>, usize) {
        let (new_bb, dir) = self.bb.calc_reverse_expand_bb(vc);

        (
            Box::new(Self::new_empty_with_vc_and_bb(
                new_bb,
                self.vc.clone(),
                self.count,
            )),
            dir,
        )
    }

    pub fn new_empty_with_vc_and_bb(bb: BoundBox<D>, vc: ColVec<D>, count: usize) -> Self {
        let parent: Option<(usize, usize)> = None;
        let mut nexts = Vec::with_capacity(Self::DIM_LEN);
        for _ in 0..Self::DIM_LEN {
            nexts.push(None);
        }
        Self {
            bb,
            parent,
            nexts,
            count,
            vc,
        }
    }

    pub fn new_with_leaf_replacement(
        my_i: usize,
        leaf_i: usize,
        leaf_mut_ref: &mut Leaf<D>,
    ) -> Box<Self> {
        let bb = leaf_mut_ref.bb.clone();
        let parent = leaf_mut_ref.parent;
        let next_dir = bb.calc_next_dir(&leaf_mut_ref.vc);

        let mut curr_box = Internal::new_root(bb);

        curr_box.parent = parent;
        curr_box.vc.clone_from(&leaf_mut_ref.vc);
        curr_box.count = leaf_mut_ref.get_values_num_inside();

        curr_box.link_leaf_to_dir(next_dir, my_i, leaf_i, leaf_mut_ref);

        curr_box
    }

    pub fn new_root(root_bb: BoundBox<D>) -> Box<Self> {
        let mut nexts = Vec::with_capacity(Self::DIM_LEN);
        for _ in 0..Self::DIM_LEN {
            nexts.push(None);
        }
        Box::new(Self {
            parent: None,
            nexts,
            count: 0,
            vc: ColVec::new_zeros(),
            bb: root_bb,
        })
    }

    #[inline]
    pub fn link_leaf_to_dir(
        &mut self,
        dir: usize,
        my_i: usize,
        leaf_i: usize,
        leaf_mut_ref: &mut Leaf<D>,
    ) {
        leaf_mut_ref.set_parent(my_i, &self, dir);
        self.nexts[dir] = Some(NodeIndex::Le(leaf_i));
    }

    #[inline]
    pub fn get_values_num_inside(&self) -> usize {
        self.count
    }

    #[inline]
    pub fn drop_child(&mut self, dir: usize) -> Option<NodeIndex> {
        self.nexts[dir].take()
    }
}
