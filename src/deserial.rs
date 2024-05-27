use std::{collections::VecDeque, fmt::Display, ptr};

use crate::{
    nodes::{Internal, Leaf, NodePtr},
    BHTree, Fnum, Udim,
};

#[derive(serde::Serialize)]
pub struct BHTreeSerde<const D: Udim> {
    dim: usize,
    num: usize,
    vcs: Vec<Fnum>,
    bcs: Vec<Fnum>,
    brs: Vec<Fnum>,
    ns: Vec<usize>,
    from_dirs: Vec<Option<usize>>,
}

impl<const D: Udim> BHTreeSerde<D> {
    pub fn with_num_of_nodes(num: usize) -> BHTreeSerde<D> {
        let vcs: Vec<Fnum> = Vec::with_capacity(num * D);
        let bcs: Vec<Fnum> = Vec::with_capacity(num * D);
        let brs: Vec<Fnum> = Vec::with_capacity(num);
        let ns: Vec<usize> = Vec::with_capacity(num);
        let from_dirs: Vec<Option<usize>> = Vec::with_capacity(num);
        BHTreeSerde {
            num,
            dim: D,
            vcs,
            bcs,
            brs,
            ns,
            from_dirs,
        }
    }

    pub fn add_node(
        &mut self,
        parent_opt: Option<usize>,
        from_dir: Option<usize>,
        vc: &[Fnum; D],
        bc: &[Fnum; D],
        br: Fnum,
        n: usize,
    ) -> usize {
        let curr_i = self.ns.len();

        self.from_dirs.push(from_dir);

        for v in vc.iter() {
            self.vcs.push(*v);
        }
        for v in bc.iter() {
            self.bcs.push(*v);
        }

        self.brs.push(br);

        self.ns.push(n);
        curr_i
    }
}

impl<const D: Udim> BHTree<D> {
    pub fn calc_serde_bhtree(&self) -> BHTreeSerde<D> {
        let mut ans = BHTreeSerde::<D>::with_num_of_nodes(self.count + self.leaf_refs.len());
        let mut dq: VecDeque<(*const Internal<D>, Option<(usize, usize)>)> =
            VecDeque::with_capacity(self.count);

        fn add_leaf<const D: Udim>(
            parent_opt: Option<usize>,
            from_dir: Option<usize>,
            leaf_ptr_ref: &*mut Leaf<D>,
            ans: &mut BHTreeSerde<D>,
        ) {
            let leaf_ref = unsafe {
                leaf_ptr_ref
                    .as_ref()
                    .expect("Should be able to dereference")
            };
            ans.add_node(
                parent_opt,
                from_dir,
                leaf_ref.get_vc().inside(),
                leaf_ref.get_bb().get_bc().inside(),
                leaf_ref.get_bb().get_br().clone(),
                1,
            );
        }

        match &self.root {
            Some(NodePtr::In(next_box_ref)) => {
                dq.push_back((ptr::addr_of!(**next_box_ref), None));
            }
            Some(NodePtr::Le(next_leaf_ptr_ref)) => {
                add_leaf(None, None, next_leaf_ptr_ref, &mut ans);
            }
            _ => (),
        }

        while !dq.is_empty() {
            let (curr, parent_info) = dq.pop_front().expect("Just checked unempty");
            let curr_ref = unsafe { curr.as_ref().expect("Dereferencing a next") };
            let (parent_opt, from_dir) = if let Some((parent_i, from_dir)) = parent_info {
                (Some(parent_i), Some(from_dir))
            } else {
                (None, None)
            };
            let curr_i = ans.add_node(
                parent_opt,
                from_dir,
                curr_ref.get_vc().inside(),
                curr_ref.get_bb().get_bc().inside(),
                curr_ref.get_bb().get_br().clone(),
                curr_ref.get_count(),
            );
            for (from_dir, next) in curr_ref.get_nexts().iter().enumerate() {
                match next {
                    Some(NodePtr::In(next_box_ref)) => {
                        dq.push_back((ptr::addr_of!(**next_box_ref), Some((curr_i, from_dir))));
                    }
                    Some(NodePtr::Le(next_leaf_ptr_ref)) => {
                        add_leaf(Some(curr_i), Some(from_dir), next_leaf_ptr_ref, &mut ans);
                    }
                    _ => (),
                }
            }
        }
        ans
    }
}

impl<const D: Udim> Display for BHTree<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match serde_json::to_string(&self.calc_serde_bhtree()) {
                Ok(s) => s,
                Err(_) => return std::fmt::Result::Err(std::fmt::Error),
            }
        ))?;
        Ok(())
    }
}
