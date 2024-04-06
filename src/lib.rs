type Fnum = f64;
type Udim = usize;

mod colvec;

mod boundbox;

use std::{collections::VecDeque, fmt::Display, ptr};

use crate::NodePtr::*;
use boundbox::BoundBox;
use colvec::ColVec;
use nodes::{Internal, Leaf, NodePtr};
mod nodes;

pub struct BHTree<const D: Udim> {
    leaf_refs: Vec<Box<Leaf<D>>>,
    root: Option<NodePtr<D>>,

    count: usize,
    bb: BoundBox<D>,
}

impl<const D: Udim> BHTree<D> {
    // const DIM: usize = D;
    // const DIM_LEN: usize = 2_usize.pow(D as u32);

    fn add(&mut self, i: &usize) {
        let leaf_ptr = ptr::addr_of_mut!(*self.leaf_refs[*i]);
        let leaf_ref = unsafe {
            leaf_ptr
                .as_mut()
                .expect("Dereferencing the current leaf node to add")
        };
        let leaf_vc = leaf_ref.get_vc();

        let mut curr_ptr = ptr::addr_of_mut!(self.root);
        let mut prev_internal: Option<(*mut Internal<D>, usize)> = None;

        while let Some(curr_ref) = unsafe { curr_ptr.as_mut().expect("The Root Ptr") } {
            let target_internal = match curr_ref {
                Le(curr_leaf_ptr) => {
                    // let curr_leaf_ref = unsafe {
                    //     curr_leaf_ptr
                    //         .as_ref()
                    //         .expect("The leaf at the current position")
                    // };

                    let mut inserting_internal_box =
                        Internal::new_with_leaf_replacement(*curr_leaf_ptr);
                    self.count += 1;
                    // println!("{}", *inserting_internal_box);
                    let inserting_internal_star_mut = ptr::addr_of_mut!(*inserting_internal_box);
                    *curr_ref = NodePtr::In(inserting_internal_box);
                    unsafe {
                        inserting_internal_star_mut
                            .as_mut()
                            .expect("The Internal Node Just inserted")
                    }
                }
                In(internal_box) => &mut *internal_box,
            };

            target_internal.add_vc(leaf_vc);

            let next_dir = target_internal.calc_next_dir(leaf_vc);
            let next_ptr = target_internal.get_child_star_mut(&next_dir);

            curr_ptr = next_ptr;

            prev_internal = Some((ptr::addr_of_mut!(*target_internal), next_dir.clone()));
        }
        let curr_ptr_ref = unsafe { curr_ptr.as_mut().expect("The pointer position to add") };
        *curr_ptr_ref = Some(Le(leaf_ptr));

        if let Some((parent_ptr, dir)) = prev_internal {
            // let parent_ref = unsafe {
            //     parent_ptr
            //         .as_mut()
            //         .expect("The internal node the leaf is adding to")
            // };

            leaf_ref.set_parent(parent_ptr, &dir);
        } else {
            leaf_ref.set_bb(&self.bb);
        }
    }

    pub fn new_with_vec(root_bc: &[Fnum; D], root_br: &Fnum, vals: &[[Fnum; D]]) -> Self {
        let len = vals.len();
        let mut leaf_refs: Vec<Box<Leaf<D>>> = Vec::with_capacity(len);
        for val in vals {
            let curr = Leaf::new_leaf(ColVec::new_with_arr(val));
            leaf_refs.push(Box::new(curr));
        }
        let mut temp_self = Self {
            leaf_refs,
            root: None,
            bb: BoundBox::new_with_arr(root_bc, root_br.clone()),
            count: 0,
        };
        for i in 0..len {
            temp_self.add(&i);
        }
        temp_self
    }
}

impl<const D: Udim> Display for BHTree<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dq: VecDeque<*const Internal<D>> = VecDeque::with_capacity(self.count);
        match &self.root {
            Some(NodePtr::In(next_box_ref)) => {
                f.write_fmt(format_args!("{}\n", **next_box_ref))?;
                dq.push_back(ptr::addr_of!(**next_box_ref));
            }
            Some(NodePtr::Le(next_leaf_ptr_ref)) => unsafe {
                f.write_fmt(format_args!("{}\n", **next_leaf_ptr_ref))?;
            },
            _ => (),
        }

        while !dq.is_empty() {
            let curr = dq.pop_front().expect("Just checked unempty");
            let curr_ref = unsafe { curr.as_ref().expect("Dereferencing a next") };

            for next in curr_ref.get_nexts() {
                match next {
                    Some(NodePtr::In(next_box_ref)) => {
                        f.write_fmt(format_args!("{}\n", **next_box_ref))?;
                        dq.push_back(ptr::addr_of!(**next_box_ref));
                    }
                    Some(NodePtr::Le(next_leaf_ptr_ref)) => unsafe {
                        f.write_fmt(format_args!("{}\n", **next_leaf_ptr_ref))?;
                    },
                    _ => (),
                }
            }
        }
        Ok(())
    }
}
