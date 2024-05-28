use std::{fmt::Display, ptr};

use crate::{boundbox::BoundBox, colvec::ColVec, Udim};

use super::{Leaf, NodeBox};

pub struct Internal<const D: Udim> {
    pub(crate) parent: Option<(*mut Self, usize)>,
    pub(crate) nexts: Vec<Option<NodeBox<D>>>,

    pub(crate) count: usize,
    pub(crate) vc: ColVec<D>,
    pub(crate) bb: BoundBox<D>,
}

impl<const D: Udim> Internal<D> {
    // const DIM: usize = D;
    const DIM_LEN: usize = 2_usize.pow(D as u32);

    pub fn calc_next_dir(&self, vc: &ColVec<D>) -> usize {
        self.bb.calc_next_dir(vc)
    }

    pub fn calc_child_bb(&self, dir: &usize) -> BoundBox<D> {
        self.bb.calc_child_bb(dir)
    }

    pub fn add_vc(&mut self, vc: &ColVec<D>) {
        self.vc.add_colvec_to_self(vc);
        self.count += 1;
    }

    pub fn get_child_star_mut(&mut self, dir: &usize) -> *mut Option<NodeBox<D>> {
        ptr::addr_of_mut!(self.nexts[*dir])
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
        let parent: Option<(*mut Internal<D>, usize)> = None;
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

    pub fn new_with_leaf_replacement(mut leaf_box: Box<Leaf<D>>) -> Box<Self> {
        let bb = leaf_box.bb.clone();
        let parent = leaf_box.parent;
        let next_dir = bb.calc_next_dir(&leaf_box.vc);
        let mut curr = Internal::new_root(bb);
        let next_ptr = curr.get_child_star_mut(&next_dir);
        let next_ref = unsafe { next_ptr.as_mut().expect("The leaf's next position") };

        curr.add_vc(&leaf_box.vc);
        curr.parent = parent;

        let mut curr_box = Box::new(curr);
        leaf_box.set_parent(ptr::addr_of_mut!(*curr_box), next_dir);
        *next_ref = Some(NodeBox::Le(leaf_box));

        curr_box
    }

    pub fn new_root(root_bb: BoundBox<D>) -> Self {
        let mut nexts = Vec::with_capacity(Self::DIM_LEN);
        for _ in 0..Self::DIM_LEN {
            nexts.push(None);
        }
        Self {
            parent: None,
            nexts,
            count: 0,
            vc: ColVec::new_zeros(),
            bb: root_bb,
        }
    }
}

impl<const D: Udim> Display for Internal<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "n: {}, vc: {}, bb: {}",
            self.count, self.vc, self.bb
        ))?;

        Ok(())
    }
}
