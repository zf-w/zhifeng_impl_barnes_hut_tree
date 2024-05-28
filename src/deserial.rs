use std::{collections::VecDeque, fmt::Display, ptr};

use crate::{
    nodes::{Internal, Leaf, NodeBox},
    BHTree, ColVec, Fnum, Udim,
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BHTreeSer<const D: Udim> {
    dim: usize,
    num: usize,
    vcs: Vec<Fnum>,
    bcs: Vec<Fnum>,
    brs: Vec<Fnum>,
    ns: Vec<usize>,
    parents: Vec<Option<usize>>,
    from_dirs: Vec<Option<usize>>,
    vs: Vec<Fnum>,
    to_leafs: Vec<Option<usize>>,
    idxs: Vec<Option<usize>>,
}

impl<const D: Udim> BHTreeSer<D> {
    pub fn with_num_of_nodes(num: usize, vs: &Vec<ColVec<D>>) -> BHTreeSer<D> {
        let vcs: Vec<Fnum> = Vec::with_capacity(num * D);
        let bcs: Vec<Fnum> = Vec::with_capacity(num * D);
        let brs: Vec<Fnum> = Vec::with_capacity(num);
        let ns: Vec<usize> = Vec::with_capacity(num);
        let from_dirs: Vec<Option<usize>> = Vec::with_capacity(num);
        let parents: Vec<Option<usize>> = Vec::with_capacity(num);

        let to_leafs: Vec<Option<usize>> = vec![None; vs.len()];
        let idxs: Vec<Option<usize>> = vec![None; vs.len()];
        let mut ans_vs: Vec<Fnum> = Vec::with_capacity(num * D);
        for v in vs.iter() {
            for d in 0..D {
                ans_vs.push(v.data[d]);
            }
        }

        BHTreeSer {
            num,
            dim: D,
            vcs,
            bcs,
            brs,
            ns,
            parents,
            from_dirs,

            vs: ans_vs,
            to_leafs,
            idxs,
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

        self.parents.push(parent_opt);

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

    pub fn get_num(&self) -> &usize {
        &self.num
    }
    pub fn get_vcs(&self) -> &Vec<Fnum> {
        &self.vcs
    }
    pub fn get_bcs(&self) -> &Vec<Fnum> {
        &self.bcs
    }
    pub fn get_brs(&self) -> &Vec<Fnum> {
        &self.brs
    }
    pub fn get_ns(&self) -> &Vec<usize> {
        &self.ns
    }
    pub fn get_parents(&self) -> &Vec<Option<usize>> {
        &self.parents
    }
    pub fn get_from_dirs(&self) -> &Vec<Option<usize>> {
        &self.from_dirs
    }
    pub fn get_vs(&self) -> &Vec<Fnum> {
        &self.vs
    }
    pub fn get_to_leafs(&self) -> &Vec<Option<usize>> {
        &self.to_leafs
    }
    pub fn get_idxs(&self) -> &Vec<Option<usize>> {
        &self.idxs
    }
}

impl<const D: Udim> BHTree<D> {
    pub fn calc_serde_bhtree(&self) -> BHTreeSer<D> {
        let mut ans = BHTreeSer::<D>::with_num_of_nodes(self.count, &self.vs);
        let mut dq: VecDeque<(*const Internal<D>, Option<(usize, usize)>)> =
            VecDeque::with_capacity(self.count);
        let vs_start_ptr = self.vs.as_ptr_range().start;

        fn add_leaf<const D: Udim>(
            parent_opt: Option<usize>,
            from_dir: Option<usize>,
            leaf_ref: &Leaf<D>,
            ans: &mut BHTreeSer<D>,
            vs_start_ptr: *const ColVec<D>,
        ) {
            let curr_i = ans.add_node(
                parent_opt,
                from_dir,
                &leaf_ref.vc.data,
                &leaf_ref.bb.bc.data,
                leaf_ref.bb.br.clone(),
                leaf_ref.vs.len(),
            );

            for (i, v_ptr) in leaf_ref.vs.iter().enumerate() {
                let leaf_i = unsafe { (*v_ptr).offset_from(vs_start_ptr) };
                ans.to_leafs[leaf_i as usize] = Some(curr_i);
                ans.idxs[leaf_i as usize] = Some(i);
            }
        }

        match &self.root {
            Some(NodeBox::In(next_box_ref)) => {
                dq.push_back((ptr::addr_of!(**next_box_ref), None));
            }
            Some(NodeBox::Le(next_leaf_ptr_ref)) => {
                add_leaf(None, None, next_leaf_ptr_ref, &mut ans, vs_start_ptr);
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
                &curr_ref.vc.data,
                &curr_ref.bb.bc.data,
                curr_ref.bb.br.clone(),
                curr_ref.count,
            );
            for (from_dir, next) in curr_ref.nexts.iter().enumerate() {
                match next {
                    Some(NodeBox::In(next_box_ref)) => {
                        dq.push_back((ptr::addr_of!(**next_box_ref), Some((curr_i, from_dir))));
                    }
                    Some(NodeBox::Le(next_leaf_ptr_ref)) => {
                        add_leaf(
                            Some(curr_i),
                            Some(from_dir),
                            &next_leaf_ptr_ref,
                            &mut ans,
                            vs_start_ptr,
                        );
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
