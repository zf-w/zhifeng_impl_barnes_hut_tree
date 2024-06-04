//! # Barnes-Hut Tree for accelerated N-body force calculation
//!
//!

const DEFAULT_BR_LIMIT: Fnum = 1e-8;

type Fnum = f64;
type Udim = usize;

mod colvec;

use std::collections::VecDeque;

use colvec::ColVec;

mod boundbox;

use boundbox::BoundBox;

mod nodes;
use nodes::{Internal, Leaf, NodeIndex};

/// # Barnes-Hut Tree
///
/// This is Zhifeng's implementation of Barnes-Hut Tree for accelerated N-body force calculation.
pub struct BarnesHutTree<const D: Udim> {
    vs: Vec<(ColVec<D>, Option<(usize, usize)>)>,

    leaf_vec: Vec<Box<Leaf<D>>>,
    internal_vec: Vec<Box<Internal<D>>>,

    root: Option<NodeIndex>,

    bb: BoundBox<D>,

    br_limit: Fnum,
}

mod imple;

/// # Constructors
impl<const D: Udim> BarnesHutTree<D> {
    ///
    /// Construct a new, empty Barnes Hut Tree.
    ///
    /// ## Example:
    ///
    /// ```
    /// use zhifeng_impl_barnes_hut_tree as zbht;
    ///
    /// use zbht::BarnesHutTree as BHTree;
    ///
    /// let mut bht: BHTree<2> = BHTree::new();
    /// bht.push(&[-1.0,1.0]);
    /// bht.push(&[1.0,1.0]);
    ///
    /// let mut ans_displacement = [0.0; 2];
    ///
    /// bht.calc_force_on_value(0, |_,_,_| -> bool {false}, zbht::utils::factory_of_repulsive_displacement_calc_fn::<2>(1.0, 0.2), &mut ans_displacement);
    ///
    /// assert_eq!(ans_displacement, [(-2.0 * 0.2) / (2.0 * 2.0), 0.0],"The calculated displacement should be the same.");
    /// ```
    pub fn new() -> Self {
        let leaf_vec = Vec::new();
        let internal_vec = Vec::new();
        Self {
            vs: Vec::new(),
            leaf_vec,
            internal_vec,
            root: None,
            bb: BoundBox::new_with_arr(&[0.0; D], 1.0),
            br_limit: DEFAULT_BR_LIMIT,
        }
    }

    ///
    ///  Construct a new Barnes Hut Tree with specified:
    /// - the initial bounding center and radius (half width),
    /// - the estimation of number of values ("bodies") it's going to contain,
    ///
    /// ## Example:
    ///
    /// ```
    /// use zhifeng_impl_barnes_hut_tree as zbht;
    ///
    /// use zbht::BarnesHutTree as BHTree;
    ///
    /// let mut bht: BHTree<2> = BHTree::with_bounding_and_capacity(&[0.0,0.0],2.0, 100);
    ///
    /// bht.push(&[-1.0,1.0]);
    /// bht.push(&[1.0,1.0]);
    ///
    /// let mut ans_displacement = [0.0; 2];
    ///
    /// bht.calc_force_on_value(0, |_,_,_| -> bool {false}, zbht::utils::factory_of_repulsive_displacement_calc_fn::<2>(1.0, 0.2), &mut ans_displacement);
    ///
    /// assert_eq!(ans_displacement, [(-2.0 * 0.2) / (2.0 * 2.0), 0.0],"The calculated displacement should be the same.");
    /// ```
    pub fn with_bounding_and_capacity(root_bc: &[Fnum; D], root_br: Fnum, len: usize) -> Self {
        Self::with_bounding_and_capacity_and_limit(root_bc, root_br, len, DEFAULT_BR_LIMIT)
    }

    /// Construct a new Barnes Hut Tree with specified:
    /// - the initial bounding center and radius (half width),
    /// - the estimation of number of values ("bodies") it's going to contain,
    /// - the minimum "radius" (half width) of the hyper cube.
    ///
    /// ## Example:
    ///
    /// ```
    /// use zhifeng_impl_barnes_hut_tree as zbht;
    ///
    /// use zbht::BarnesHutTree as BHTree;
    ///
    /// let mut bht: BHTree<2> = BHTree::with_bounding_and_capacity_and_limit(&[0.0,0.0],2.0, 100, 100.0);
    ///
    /// bht.push(&[-1.0,1.0]);
    /// bht.push(&[1.0,1.0]);
    ///
    /// let mut ans_displacement = [0.0; 2];
    ///
    /// bht.calc_force_on_value(0, |_,_,_| -> bool {false}, zbht::utils::factory_of_repulsive_displacement_calc_fn::<2>(1.0, 0.2), &mut ans_displacement);
    ///
    /// assert_eq!(ans_displacement, [(-2.0 * 0.2) / (2.0 * 2.0), 0.0],"The calculated displacement should be the same.");
    /// assert_eq!(bht.get_total_nodes_num(), 1, "The total number of nodes inside the tree.")
    /// ```
    pub fn with_bounding_and_capacity_and_limit(
        root_bc: &[Fnum; D],
        root_br: Fnum,
        len: usize,
        br_limit: Fnum,
    ) -> Self {
        Self {
            vs: Vec::with_capacity(len),
            leaf_vec: Vec::with_capacity(len),
            internal_vec: Vec::with_capacity(len),
            root: None,
            bb: BoundBox::new_with_arr(root_bc, root_br),
            br_limit,
        }
    }
    /// Construct a new Barnes Hut Tree with specified:
    /// - the initial bounding center and radius (half width),
    /// - the to-insert values (bodies).
    ///
    /// ## Example:
    ///
    /// ```
    /// use zhifeng_impl_barnes_hut_tree as zbht;
    ///
    /// use zbht::BarnesHutTree as BHTree;
    ///
    /// let bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0,0.0],2.0,&[[-1.0,1.0],[1.0,1.0]]);
    ///
    /// let mut ans_displacement = [0.0; 2];
    ///
    /// bht.calc_force_on_value(0, |_,_,_| -> bool {false}, zbht::utils::factory_of_repulsive_displacement_calc_fn::<2>(1.0, 0.2), &mut ans_displacement);
    ///
    /// assert_eq!(ans_displacement, [(-2.0 * 0.2) / (2.0 * 2.0), 0.0],"The calculated displacement should be the same.");
    /// ```
    pub fn with_bounding_and_values(
        root_bc: &[Fnum; D],
        root_br: Fnum,
        vals: &[[Fnum; D]],
    ) -> Self {
        let mut temp_self = Self::new_without_add(root_bc, root_br, vals, DEFAULT_BR_LIMIT);
        for i in 0..vals.len() {
            temp_self.add(i);
        }
        temp_self
    }

    /// Construct a new Barnes Hut Tree with specified:
    /// - the initial bounding center and radius (half width),
    /// - the to-insert values (bodies),
    /// - the minimum "radius" (half width) of the hyper cube.
    ///
    /// ## Example:
    ///
    /// ```
    /// use zhifeng_impl_barnes_hut_tree as zbht;
    ///
    /// use zbht::BarnesHutTree as BHTree;
    ///
    /// let bht: BHTree<2> = BHTree::with_bounding_and_values_and_limit(&[0.0,0.0],2.0, &[[-1.0,1.0],[1.0,1.0]], 100.0);
    ///
    ///
    /// let mut ans_displacement = [0.0; 2];
    ///
    /// bht.calc_force_on_value(0, |_,_,_| -> bool {false}, zbht::utils::factory_of_repulsive_displacement_calc_fn::<2>(1.0, 0.2), &mut ans_displacement);
    ///
    /// assert_eq!(ans_displacement, [(-2.0 * 0.2) / (2.0 * 2.0), 0.0],"The calculated displacement should be the same.");
    /// assert_eq!(bht.get_total_nodes_num(), 1, "The total number of nodes inside the tree.")
    /// ```
    pub fn with_bounding_and_values_and_limit(
        root_bc: &[Fnum; D],
        root_br: Fnum,
        vals: &[[Fnum; D]],
        br_limit: Fnum,
    ) -> Self {
        let num = vals.len();
        let mut temp_self = Self::new_without_add(root_bc, root_br, vals, br_limit);
        for i in 0..num {
            temp_self.add(i);
        }
        temp_self
    }

    /// Calculate force or other custom functions on a specific value (body).
    ///
    /// The function takes:
    /// - a index of a target value
    /// - a closure to determine whether a super node is "far" enough to be considered as a whole,
    /// - a closure to calculate force or other relations between the target value and another super node (or value if the size is one),
    /// - a custom struct to store and accumulate the results from the previous calculator function.
    ///
    /// ## Example:
    ///
    /// ```
    /// use zhifeng_impl_barnes_hut_tree as zbht;
    ///
    /// use zbht::BarnesHutTree as BHTree;
    ///
    /// let bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0,0.0],2.0, &[[-1.0,1.0],[1.0,1.0]]);
    ///
    ///
    /// let mut ans_displacement = [0.0; 2];
    ///
    /// let is_super_fn = |_: &[f64; 2],_: &[f64; 2],_: f64| -> bool {false}; // If every node is not far enough, the approximation will be the same (but slower due to tree traversal) as looping through all the nodes.
    /// let calc_fn = zbht::utils::factory_of_repulsive_displacement_calc_fn::<2>(1.0, 0.2);
    ///
    /// bht.calc_force_on_value(0, &is_super_fn, &calc_fn, &mut ans_displacement);
    ///
    /// assert_eq!(ans_displacement, [(-2.0 * 0.2) / (2.0 * 2.0), 0.0],"The calculated displacement should be the same.");
    /// ```
    pub fn calc_force_on_value<T>(
        &self,
        value_i: usize,
        is_super_node: impl Fn(&[Fnum; D], &[Fnum; D], Fnum) -> bool,
        calc_fn: impl Fn(&[Fnum; D], &[Fnum; D], usize, &mut T),
        write_to_value: &mut T,
    ) -> bool {
        if value_i >= self.vs.len() {
            return false;
        }

        let mut curr_info =
            self.calc_leaf_siblings_and_get_parent(value_i, &calc_fn, write_to_value);

        let mut q: VecDeque<&NodeIndex> = VecDeque::with_capacity(self.get_total_nodes_num() / 2);
        let curr_v_ref = &self.vs[value_i].0.data;

        while let Some((curr_internal_i, curr_in_leaf_i)) = curr_info {
            let curr_internal_ref = self.internal_vec[curr_internal_i].as_ref();
            for (in_leaf_i, node_opt) in curr_internal_ref.nexts.iter().enumerate() {
                if in_leaf_i == curr_in_leaf_i {
                    continue;
                }
                if let Some(curr_node_box_ref) = node_opt.as_ref() {
                    self.calc_node(
                        curr_v_ref,
                        curr_node_box_ref,
                        &mut q,
                        write_to_value,
                        &is_super_node,
                        &calc_fn,
                    )
                }
            }
            curr_info = curr_internal_ref.parent;
        }

        while let Some(curr_node_box_ref) = q.pop_front() {
            self.calc_node(
                curr_v_ref,
                curr_node_box_ref,
                &mut q,
                write_to_value,
                &is_super_node,
                &calc_fn,
            )
        }
        true
    }
}

/// # Utilities
///
/// These functions are about getting, pushing, removing, and updating values (bodies) inside the tree.
impl<const D: Udim> BarnesHutTree<D> {
    /// Getting a reference of the stored value's coordinates.
    ///
    ///  # Return
    ///
    /// Returns a reference of the current value (body)'s coordinates if the index is within-range.
    ///
    /// # Example
    /// ```
    /// use zhifeng_impl_barnes_hut_tree as zbht;
    ///
    /// use zbht::BarnesHutTree as BHTree;
    ///
    /// let bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0,0.0],2.0, &[[-1.0,1.0],[1.0,1.0]]);
    ///
    /// assert_eq!(bht.get(0).unwrap(), &[-1.0,1.0]);
    /// ```
    pub fn get(&self, value_i: usize) -> Option<&[Fnum; D]> {
        if value_i >= self.vs.len() {
            return None;
        }
        Some(&self.vs[value_i].0.data)
    }

    /// Pushes a value into the tree.
    ///
    /// # Return
    ///
    /// The function will return the value's corresponding value-index `usize` in the tree.
    /// This design is mainly for easier mapping of values in case we want to connect the values with other format of keys, for example, `String` or `usize` that is not filling an entire range from `0..len`.
    ///
    /// # Example:
    /// ```
    /// use zhifeng_impl_barnes_hut_tree as zbht;
    ///
    /// use zbht::BarnesHutTree as BHTree;
    ///
    /// let mut bht: BHTree<2> = BHTree::with_bounding_and_capacity(&[0.0,0.0],2.0, 100);
    ///
    /// bht.push(&[-1.0,1.0]);
    /// let idx = bht.push(&[1.0,1.0]);
    ///
    /// assert_eq!(bht.get(0).unwrap(), &[-1.0,1.0]);
    /// assert_eq!(bht.get(idx).unwrap(), &[1.0,1.0]);
    /// ```
    pub fn push(&mut self, value_ref: &[Fnum; D]) -> usize {
        let value_i = self.vs.len();
        self.vs.push((ColVec::new_with_arr(value_ref), None));

        self.add(value_i);
        value_i
    }

    /// Update the coordinates of a value.
    ///
    /// # Return
    ///
    /// A boolean indicating whether the update is successful. The update will usually be successful if the value-index is pointing to a valid value.
    ///
    /// # Example
    /// ```
    /// use zhifeng_impl_barnes_hut_tree as zbht;
    ///
    /// use zbht::BarnesHutTree as BHTree;
    ///
    /// let mut bht: BHTree<2> = BHTree::with_bounding_and_capacity(&[0.0,0.0],2.0, 100);
    ///
    /// bht.push(&[-0.5,1.0]);
    /// let idx = bht.push(&[1.0,1.0]);
    ///
    /// bht.update(0, &[-1.0,1.0]);
    ///
    /// assert_eq!(bht.get(0).unwrap(), &[-1.0,1.0]);
    /// assert_eq!(bht.get(idx).unwrap(), &[1.0,1.0]);
    /// ```
    pub fn update(&mut self, value_i: usize, value_ref: &[Fnum; D]) -> bool {
        let len = self.vs.len();
        if value_i >= len {
            return false;
        }
        self.sub(value_i);
        debug_assert!(
            self.vs[value_i].1.is_none(),
            "The to_leaf index should be `None`."
        );
        self.vs[value_i].0.clone_from_arr_ref(value_ref);
        self.add(value_i);
        true
    }

    /// Remove a value from the tree.
    ///
    /// # Return
    ///
    /// Since the underlying structure uses `vec` to store nodes, if a value that is not the last one in the `vec` needs to be removed, the last value from the `vec` will replace its position. The function returns an option of an index `usize` to indicate which value was moved to that position to replace the removed one (always the previous last one). If the to-remove value is the last one, or the index is out-of-range, the function will return a `None`.
    ///
    /// # Example
    /// ```
    /// use zhifeng_impl_barnes_hut_tree as zbht;
    ///
    /// use zbht::BarnesHutTree as BHTree;
    ///
    /// let mut bht: BHTree<2> = BHTree::with_bounding_and_capacity(&[0.0,0.0],2.0, 100);
    ///
    /// bht.push(&[-0.5,1.0]);
    /// bht.push(&[1.0,1.0]);
    ///
    /// let old_i = bht.remove(0).unwrap();
    ///
    /// assert_eq!(old_i, 1);
    /// assert_eq!(bht.get(0).unwrap(), &[1.0,1.0]);
    /// ```
    pub fn remove(&mut self, value_i: usize) -> Option<usize> {
        let last_i = self.vs.len() - 1;
        if value_i > last_i {
            return None;
        }
        self.sub(value_i);
        let last_v_opt = self.vs.pop().expect("Should have a last");
        if value_i < last_i {
            if let Some((leaf_i, in_leaf_i)) = last_v_opt.1 {
                #[cfg(not(feature = "unchecked"))]
                {
                    *self
                        .leaf_vec
                        .get_mut(leaf_i)
                        .expect("To update the leaf node's value pointing index to the new value location; The leaf should be valid")
                        .vs
                        .get_mut(in_leaf_i)
                        .expect("To update the leaf node's value pointing index to the new value location; The in-leaf position should be valid") = value_i;
                }

                #[cfg(feature = "unchecked")]
                {
                    unsafe {
                        *self
                            .leaf_vec
                            .get_unchecked_mut(leaf_i)
                            .vs
                            .get_unchecked_mut(in_leaf_i) = value_i;
                    }
                }
            }
            self.vs[value_i] = last_v_opt;
            Some(last_i)
        } else {
            None
        }
    }

    /// Get the total number of nodes
    ///
    /// # Return
    ///
    /// The total number of tree nodes.
    ///
    /// # Example
    /// ```
    /// use zhifeng_impl_barnes_hut_tree as zbht;
    ///
    /// use zbht::BarnesHutTree as BHTree;
    ///
    /// let mut bht: BHTree<2> = BHTree::with_bounding_and_capacity(&[0.0,0.0],2.0, 100);
    ///
    /// assert_eq!(bht.get_total_nodes_num(), 0);
    ///
    /// bht.push(&[-1.0,1.0]);
    /// assert_eq!(bht.get_total_nodes_num(), 1);
    ///
    /// bht.push(&[1.0,1.0]);
    /// assert_eq!(bht.get_total_nodes_num(), 3);
    ///
    /// bht.remove(0);
    /// assert_eq!(bht.get_total_nodes_num(), 1);
    ///
    /// bht.remove(0);
    /// assert_eq!(bht.get_total_nodes_num(), 0);
    /// ```
    #[inline]
    pub fn get_total_nodes_num(&self) -> usize {
        self.internal_vec.len() + self.leaf_vec.len()
    }
}

pub mod utils;

#[cfg(any(feature = "serialize"))]
mod serialize;
#[cfg(any(feature = "serialize"))]
pub use serialize::BarnesHutTreeSer;
