use crate::{
    boundbox::BoundBox,
    colvec::ColVec,
    nodes::{Internal, Leaf, NodeIndex},
    BarnesHutTree, Fnum, Udim,
};

#[inline]
#[cfg(feature = "unchecked")]
pub(crate) fn get_mut_ref_from_arr_mut_ref<'o, O, T: AsMut<O>>(
    arr_mut_ref: &'o mut [T],
    i: usize,
    _: &'static str,
) -> &'o mut O {
    unsafe { arr_mut_ref.get_unchecked_mut(i).as_mut() }
}
#[inline]
#[cfg(not(feature = "unchecked"))]
pub(crate) fn get_mut_ref_from_arr_mut_ref<'o, O, T: AsMut<O>>(
    arr_mut_ref: &'o mut [T],
    i: usize,
    expect_str: &'static str,
) -> &'o mut O {
    arr_mut_ref.get_mut(i).expect(expect_str).as_mut()
}

#[inline]
#[cfg(feature = "unchecked")]
pub(crate) fn get_ref_from_arr_ref<'o, O, T: AsRef<O>>(
    arr_ref: &'o [T],
    i: usize,
    _: &'static str,
) -> &'o O {
    unsafe { arr_ref.get_unchecked(i).as_ref() }
}
#[inline]
#[cfg(not(feature = "unchecked"))]
pub(crate) fn get_ref_from_arr_ref<'o, O, T: AsRef<O>>(
    arr_ref: &'o [T],
    i: usize,
    expect_str: &'static str,
) -> &'o O {
    arr_ref.get(i).expect(expect_str).as_ref()
}

mod add;

mod sub;

mod calc;

impl<const D: Udim> BarnesHutTree<D> {
    #[inline]
    pub(crate) fn new_without_add(
        root_bc: &[Fnum; D],
        root_br: Fnum,
        vals: &[[Fnum; D]],
        br_limit: Fnum,
    ) -> Self {
        let len = vals.len();
        let mut vs: Vec<Box<(ColVec<D>, Option<(usize, usize)>)>> = Vec::with_capacity(len);
        let leaf_vec = Vec::with_capacity(len);
        let internal_vec = Vec::with_capacity(len);
        for val in vals {
            vs.push(Box::new((ColVec::new_with_arr(val), None)));
        }
        Self {
            vs,
            leaf_vec,
            internal_vec,
            root: None,
            bb: BoundBox::new_with_arr(root_bc, root_br),
            br_limit,
        }
    }

    #[inline]
    pub(crate) fn new_leaf(&mut self, leaf_box: Box<Leaf<D>>) -> usize {
        let ans_i = self.leaf_vec.len();
        self.leaf_vec.push(leaf_box);
        ans_i
    }
    #[inline]
    pub(crate) fn drop_leaf(&mut self, leaf_i: usize) {
        let curr_len = self.leaf_vec.len();
        if leaf_i >= curr_len {
            return;
        }
        let curr = &self.leaf_vec[leaf_i];
        if let Some((parent_i, dir_i)) = curr.parent {
            self.internal_vec[parent_i].nexts[dir_i] = None;
        }
        for value_i in curr.vs.iter() {
            self.vs[*value_i].1 = None;
        }

        let last = self.leaf_vec.pop().expect("Checked length");
        if leaf_i < curr_len - 1 {
            if let Some((parent_i, dir_i)) = last.parent {
                self.internal_vec[parent_i].nexts[dir_i] = Some(NodeIndex::Le(leaf_i));
            }
            for (in_leaf_i, value_i) in last.vs.iter().enumerate() {
                self.vs[*value_i].1 = Some((leaf_i, in_leaf_i));
            }
            self.leaf_vec[leaf_i] = last;
        }
    }

    #[inline]
    pub(crate) fn drop_child(&mut self, internal_i: usize, dir: usize) {
        let internal_mut_ref = self.internal_vec.get_mut(internal_i).unwrap().as_mut();
        let to_drop = internal_mut_ref.drop_child(dir);
        if let Some(node_i) = to_drop {
            match node_i {
                NodeIndex::In(next_internal_i) => {
                    self.drop_internal(next_internal_i);
                }
                NodeIndex::Le(leaf_i) => {
                    self.drop_leaf(leaf_i);
                }
            }
        }
    }

    #[inline]
    pub(crate) fn new_internal(&mut self, internal_box: Box<Internal<D>>) -> usize {
        let ans_i = self.internal_vec.len();
        self.internal_vec.push(internal_box);
        ans_i
    }

    #[inline]
    pub(crate) fn drop_internal(&mut self, internal_i: usize) -> Option<(usize, usize)> {
        let curr_len = self.internal_vec.len();
        if internal_i >= curr_len {
            return None;
        }

        let last = self.internal_vec.pop().expect("Checked length");
        if internal_i < curr_len - 1 {
            if let Some((parent_i, dir_i)) = last.parent {
                #[cfg(feature = "unchecked")]
                unsafe {
                    self.internal_vec.get_unchecked_mut(parent_i).nexts[dir_i] =
                        Some(NodeIndex::In(internal_i))
                };
                #[cfg(not(feature = "unchecked"))]
                {
                    self.internal_vec
                        .get_mut(parent_i)
                        .expect("Update the replacing internal node's parent")
                        .nexts[dir_i] = Some(NodeIndex::In(internal_i));
                }
            } else {
                self.root = Some(NodeIndex::In(internal_i));
            }
            for (dir_i, node_opt) in last.nexts.iter().enumerate() {
                if let Some(node_i) = node_opt {
                    match node_i {
                        NodeIndex::In(next_internal_i) => {
                            self.internal_vec[*next_internal_i].parent = Some((internal_i, dir_i));
                        }
                        NodeIndex::Le(next_leaf_i) => {
                            self.leaf_vec[*next_leaf_i].parent = Some((internal_i, dir_i));
                        }
                    }
                }
            }
            self.internal_vec[internal_i] = last;
            Some((curr_len - 1, internal_i))
        } else {
            None
        }
    }

    //#[cfg(feature = "unchecked")]
    //#[cfg(not(feature = "unchecked"))]
    #[inline]
    pub(crate) fn set_root_leaf(&mut self, leaf_i: usize) {
        #[cfg(feature = "unchecked")]
        let leaf_mut_ref = unsafe { self.leaf_vec.get_unchecked_mut(leaf_i).as_mut() };
        #[cfg(not(feature = "unchecked"))]
        let leaf_mut_ref = self
            .leaf_vec
            .get_mut(leaf_i)
            .expect("Getting leaf for updating struct bounding box")
            .as_mut();

        leaf_mut_ref.parent = None;
        leaf_mut_ref.bb.clone_from(&self.bb);
        self.root = Some(NodeIndex::Le(leaf_i));
    }
}

#[cfg(feature = "serialize")]
mod test;
