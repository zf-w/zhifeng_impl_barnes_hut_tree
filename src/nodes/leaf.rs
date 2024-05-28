use std::fmt::Display;

use crate::{boundbox::BoundBox, colvec::ColVec, Udim};

use super::Internal;

pub struct Leaf<const D: Udim> {
    parent: Option<(*mut Internal<D>, usize)>,

    pub(crate) bb: BoundBox<D>,
    pub(crate) vc: ColVec<D>,
}

impl<const D: Udim> Leaf<D> {
    // const DIM: usize = D;
    // const DIM_LEN: usize = 2_usize.pow(D as u32);

    pub fn new_leaf(vc: ColVec<D>) -> Self {
        Self {
            parent: None,
            vc,
            bb: BoundBox::new_zeros(),
        }
    }

    pub fn set_parent(&mut self, parent: *mut Internal<D>, i: &usize) {
        let parent_ref = unsafe { parent.as_ref().expect("Parent node of the leaf") };
        self.parent = Some((parent, i.clone()));
        self.bb.clone_from(&parent_ref.calc_child_bb(i));
    }

    // pub fn break_parent(&mut self) {
    //     self.parent = None;
    // }

    // pub fn has_parent(&self) -> bool {
    //     self.parent.is_some()
    // }

    pub fn get_parent(&self) -> Option<(*mut Internal<D>, usize)> {
        self.parent
    }

    // pub fn cal_self_box(&self) -> Option<BoundBox<D>> {
    //     if let Some((parent, dir)) = self.parent {
    //         Some(unsafe {
    //             parent
    //                 .as_ref()
    //                 .expect("Parent should be able to deref")
    //                 .calc_child_bb(&dir)
    //         })
    //     } else {
    //         None
    //     }
    // }
}

impl<const D: Udim> Display for Leaf<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("vc: {}, bb: {}", self.vc, self.bb))?;
        Ok(())
    }
}
