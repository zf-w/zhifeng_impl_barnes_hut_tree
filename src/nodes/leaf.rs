use std::ptr;

use crate::{boundbox::BoundBox, colvec::ColVec, Udim};

use super::Internal;

pub struct Leaf<const D: Udim> {
    pub(crate) parent: Option<(*mut Internal<D>, usize)>,

    pub(crate) bb: BoundBox<D>,

    pub(crate) vc: ColVec<D>,
    pub(crate) vs: Vec<*const ColVec<D>>,
}

impl<const D: Udim> Leaf<D> {
    pub fn new_empty_from_bb(bb: BoundBox<D>) -> Box<Self> {
        let vc: ColVec<D> = ColVec::new_zeros();
        let vs: Vec<*const ColVec<D>> = Vec::with_capacity(1);
        let parent = None;
        Box::new(Self { parent, bb, vc, vs })
    }

    pub fn new_empty_from_parent_dir(parent: &mut Internal<D>, dir: usize) -> Box<Self> {
        let bb = parent.calc_child_bb(&dir);

        let vc: ColVec<D> = ColVec::new_zeros();
        let vs: Vec<*const ColVec<D>> = Vec::with_capacity(1);
        let parent = Some((ptr::addr_of_mut!(*parent), dir));
        Box::new(Self { parent, bb, vc, vs })
    }

    pub fn add_value(&mut self, v: &ColVec<D>) -> usize {
        let i = self.vs.len();
        self.vc.add_colvec_to_self(v);
        self.vs.push(ptr::addr_of!(*v));
        i
    }

    pub fn set_parent(&mut self, parent_ptr: *mut Internal<D>, from_dir: usize) {
        self.parent = Some((parent_ptr, from_dir));

        let parent_ref = unsafe {
            parent_ptr
                .as_ref()
                .expect("Dereferencing parent_ptr to update bounding box")
        };

        self.bb
            .set_self_from_parent_bb_and_dir(&parent_ref.bb, from_dir);
    }
}
