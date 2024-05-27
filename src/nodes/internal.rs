use std::{fmt::Display, ptr};

use crate::{boundbox::BoundBox, colvec::ColVec, Udim};

use super::{Leaf, NodePtr};

pub struct Internal<const D: Udim> {
    parent: Option<(*mut Self, usize)>,
    nexts: Vec<Option<NodePtr<D>>>,

    count: usize,
    vc: ColVec<D>,
    bb: BoundBox<D>,
}

impl<const D: Udim> Internal<D> {
    // const DIM: usize = D;
    const DIM_LEN: usize = 2_usize.pow(D as u32);

    pub fn get_bb(&self) -> &BoundBox<D> {
        &self.bb
    }

    pub fn get_count(&self) -> usize {
        self.count.clone()
    }

    pub fn get_vc(&self) -> &ColVec<D> {
        &self.vc
    }

    pub fn calc_next_dir(&self, vc: &ColVec<D>) -> usize {
        self.bb.calc_next_dir(vc)
    }

    pub fn calc_child_bb(&self, dir: &usize) -> BoundBox<D> {
        self.bb.calc_child_bb(dir)
    }

    pub fn add_vc(&mut self, vc: &ColVec<D>) {
        self.vc.add_vec_to_self(vc);
        self.count += 1;
    }

    pub fn get_child_star_mut(&mut self, dir: &usize) -> *mut Option<NodePtr<D>> {
        ptr::addr_of_mut!(self.nexts[*dir])
    }

    pub fn get_nexts(&self) -> &[Option<NodePtr<D>>] {
        &self.nexts
    }

    pub fn new_with_leaf_replacement(leaf: *mut Leaf<D>) -> Box<Self> {
        let leaf_ref = unsafe { leaf.as_mut().expect("The leaf node to replace") };
        let bb = leaf_ref.get_bb().clone();
        let parent = leaf_ref.get_parent();
        let next_dir = bb.calc_next_dir(leaf_ref.get_vc());
        let mut curr = Internal::new_root(bb);
        let next_ptr = curr.get_child_star_mut(&next_dir);
        let next_ref = unsafe { next_ptr.as_mut().expect("The leaf's next position") };
        *next_ref = Some(NodePtr::Le(leaf));
        curr.add_vc(leaf_ref.get_vc());
        curr.parent = parent;
        let mut curr_box = Box::new(curr);
        leaf_ref.set_parent(ptr::addr_of_mut!(*curr_box), &next_dir);

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
            vc: ColVec::new(),
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
