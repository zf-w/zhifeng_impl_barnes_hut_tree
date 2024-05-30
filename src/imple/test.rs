#[cfg(test)]
use crate::assert_bht_serde_eq;
#[cfg(test)]
use crate::{BHTree, BHTreeSer};

#[test]
fn check_sub_from_leaf_tree() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0]];

    let mut bht: BHTree<2> = BHTree::new_with_values(&[0.0, 0.0], 4.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":1,
            \"vcs\":[1.0,1.0],
            \"bcs\":[0.0,0.0],
            \"brs\":[4.0],
            \"ns\":[1],
            \"leaf_ns\":[1],
            \"parents\":[null],
            \"from_dirs\":[null],

            \"vs\":[1.0,1.0],
            \"to_leafs\":[0],
            \"idxs\":[0]
        }",
    )
    .unwrap();
    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);

    bht.sub(0);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":0,
            \"vcs\":[],
            \"bcs\":[],
            \"brs\":[],
            \"ns\":[],
            \"leaf_ns\":[],
            \"parents\":[],
            \"from_dirs\":[],

            \"vs\":[1.0,1.0],
            \"to_leafs\":[null],
            \"idxs\":[null]
        }",
    )
    .unwrap();
    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);

    Ok(())
}

#[test]
fn check_sub_from_tree_with_one_internal_insertion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 3.0], [3.0, 1.0]];

    let mut bht: BHTree<2> = BHTree::new_with_values(&[0.0, 0.0], 4.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":4,
            \"vcs\":[4.0,4.0,4.0,4.0,1.0,3.0,3.0,1.0],
            \"bcs\":[0.0,0.0,2.0,2.0,1.0,3.0,3.0,1.0],
            \"brs\":[4.0,2.0,1.0,1.0],
            \"ns\":[2,2,1,1],
            \"leaf_ns\":[2,2,1,1],
            \"parents\":[null,0,1,1],
            \"from_dirs\":[null,3,1,2],
            \"vs\":[1.0,3.0,3.0,1.0],
            \"to_leafs\":[2,3],
            \"idxs\":[0,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);

    bht.sub(0);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":1,
            \"vcs\":[3.0,1.0],
            \"bcs\":[0.0,0.0],
            \"brs\":[4.0],
            \"ns\":[1],
            \"leaf_ns\":[1],
            \"parents\":[null],
            \"from_dirs\":[null],
            \"vs\":[1.0,3.0,3.0,1.0],
            \"to_leafs\":[null,0],
            \"idxs\":[null,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);

    Ok(())
}

#[test]
fn check_sub_from_tree_with_two_internal_insertion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 3.0], [3.0, 1.0]];

    let mut bht: BHTree<2> = BHTree::new_with_values(&[0.0, 0.0], 8.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":5,
            \"vcs\":[4.0,4.0,4.0,4.0,4.0,4.0,1.0,3.0,3.0,1.0],
            \"bcs\":[0.0,0.0,4.0,4.0,2.0,2.0,1.0,3.0,3.0,1.0],
            \"brs\":[8.0,4.0,2.0,1.0,1.0],
            \"ns\":[2,2,2,1,1],
            \"leaf_ns\":[2,2,2,1,1],
            \"parents\":[null,0,1,2,2],
            \"from_dirs\":[null,3,0,1,2],
            \"vs\":[1.0,3.0,3.0,1.0],
            \"to_leafs\":[3,4],
            \"idxs\":[0,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);

    bht.sub(0);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":1,
            \"vcs\":[3.0,1.0],
            \"bcs\":[0.0,0.0],
            \"brs\":[8.0],
            \"ns\":[1],
            \"leaf_ns\":[1],
            \"parents\":[null],
            \"from_dirs\":[null],
            \"vs\":[1.0,3.0,3.0,1.0],
            \"to_leafs\":[null,0],
            \"idxs\":[null,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);

    Ok(())
}

#[test]
fn check_sub_from_tree_with_adding_to_same_leaf() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [-1.0, -1.0], [7.0, 7.0]];

    let mut bht: BHTree<2> = BHTree::new_with_values_and_limit(&[0.0, 0.0], 2.0, &vals, 10.0);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":1,
            \"vcs\":[7.0,7.0],
            \"bcs\":[6.0,6.0],
            \"brs\":[8.0],
            \"ns\":[3],
            \"leaf_ns\":[1],
            \"parents\":[null],
            \"from_dirs\":[null],
            \"vs\":[1.0,1.0,-1.0,-1.0,7.0,7.0],
            \"to_leafs\":[0,0,0],
            \"idxs\":[0,1,2]
        }",
    )?;

    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);

    bht.sub(0);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":1,
            \"vcs\":[6.0,6.0],
            \"bcs\":[6.0,6.0],
            \"brs\":[8.0],
            \"ns\":[2],
            \"leaf_ns\":[1],
            \"parents\":[null],
            \"from_dirs\":[null],
            \"vs\":[1.0,1.0,-1.0,-1.0,7.0,7.0],
            \"to_leafs\":[null,0,0],
            \"idxs\":[null,1,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}
