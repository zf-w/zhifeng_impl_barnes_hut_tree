use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
};

use crate::{
    nodes::{Leaf, NodeIndex},
    BarnesHutTree, ColVec, Fnum, Udim,
};

/// # The serialized form of Barnes Hut Tree
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BarnesHutTreeSer<const D: Udim> {
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

impl<const D: Udim> BarnesHutTreeSer<D> {
    pub(crate) fn with_num_of_nodes(
        num: usize,
        vs: &Vec<(ColVec<D>, Option<(usize, usize)>)>,
    ) -> BarnesHutTreeSer<D> {
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
                ans_vs.push(v.0.data[d]);
            }
        }

        BarnesHutTreeSer {
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

    pub(crate) fn add_node(
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

/// # Zhifeng's BHT Serialization Implementation
///
/// To make the serialization process simpler, I design to let BHT first serialize into the intermediate form `BarnesHutTreeSer` for `serde_json`'s auto "derive" and testing. I guess the `BarnesHutTreeSer` also makes the serialized `JSON` form relatively smaller in size due to less "struct" with "Strings" to represent fields.
impl<const D: Udim> BarnesHutTree<D> {
    pub fn calc_serialized(&self) -> BarnesHutTreeSer<D> {
        let nodes_num = self.get_total_nodes_num();
        let mut ans = BarnesHutTreeSer::<D>::with_num_of_nodes(nodes_num, &self.vs);
        let mut dq: VecDeque<(usize, Option<(usize, usize)>)> = VecDeque::with_capacity(nodes_num);

        fn add_leaf<const D: Udim>(
            parent_opt: Option<usize>,
            from_dir: Option<usize>,
            leaf_ref: &Leaf<D>,
            ans: &mut BarnesHutTreeSer<D>,
        ) {
            let curr_i = ans.add_node(
                parent_opt,
                from_dir,
                &leaf_ref.vc.data,
                &leaf_ref.bb.bc.data,
                leaf_ref.bb.br.clone(),
                leaf_ref.vs.len(),
            );

            for (i, leaf_i) in leaf_ref.vs.iter().enumerate() {
                ans.to_leafs[*leaf_i] = Some(curr_i);
                ans.idxs[*leaf_i] = Some(i);
            }
        }

        match &self.root {
            Some(NodeIndex::In(next_internal_i)) => {
                dq.push_back((next_internal_i.clone(), None));
            }
            Some(NodeIndex::Le(next_leaf_i)) => {
                add_leaf(
                    None,
                    None,
                    self.leaf_vec.get(*next_leaf_i).unwrap(),
                    &mut ans,
                );
            }
            _ => (),
        }

        while !dq.is_empty() {
            let (curr_i, parent_info) = dq.pop_front().expect("Just checked unempty");
            let curr_ref = self.internal_vec.get(curr_i).unwrap().as_ref();
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
                    Some(NodeIndex::In(next_internal_i)) => {
                        dq.push_back((*next_internal_i, Some((curr_i, from_dir))));
                    }
                    Some(NodeIndex::Le(next_leaf_i)) => {
                        add_leaf(
                            Some(curr_i),
                            Some(from_dir),
                            self.leaf_vec.get(*next_leaf_i).unwrap(),
                            &mut ans,
                        );
                    }
                    _ => (),
                }
            }
        }
        ans
    }
}

impl<const D: Udim> serde::Serialize for BarnesHutTree<D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bht_ser = self.calc_serialized();
        bht_ser.serialize(serializer)
    }
}

impl<const D: Udim> Debug for BarnesHutTree<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match serde_json::to_string(&self.calc_serialized()) {
                Ok(s) => s,
                Err(_) => return std::fmt::Result::Err(std::fmt::Error),
            }
        ))?;
        Ok(())
    }
}

impl<const D: Udim> Display for BarnesHutTree<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match serde_json::to_string(&self.calc_serialized()) {
                Ok(s) => s,
                Err(_) => return std::fmt::Result::Err(std::fmt::Error),
            }
        ))?;
        Ok(())
    }
}
