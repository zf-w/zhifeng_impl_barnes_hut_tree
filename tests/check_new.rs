use zhifeng_impl_barnes_hut_tree as zbht;

use zbht::{BarnesHutTree as BHTree, BarnesHutTreeSer as BHTreeSer};

mod utils;

use utils::assert_bht_serde_eq;

#[test]
fn check_new_to_empty() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0]];

    let bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0, 0.0], 4.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":1,
            \"vcs\":[1.0,1.0],
            \"bcs\":[0.0,0.0],
            \"brs\":[4.0],
            \"ns\":[1],
            \"parents\":[null],
            \"from_dirs\":[null],

            \"vs\":[1.0,1.0],
            \"to_leafs\":[0],
            \"idxs\":[0]
        }",
    )
    .unwrap();
    let calc_bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}

#[test]
fn check_new_with_one_internal_insertion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 3.0], [3.0, 1.0]];

    let bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0, 0.0], 4.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":4,
            \"vcs\":[2.0,2.0,2.0,2.0,1.0,3.0,3.0,1.0],
            \"bcs\":[0.0,0.0,2.0,2.0,1.0,3.0,3.0,1.0],
            \"brs\":[4.0,2.0,1.0,1.0],
            \"ns\":[2,2,1,1],
            \"parents\":[null,0,1,1],
            \"from_dirs\":[null,3,1,2],
            \"vs\":[1.0,3.0,3.0,1.0],
            \"to_leafs\":[2,3],
            \"idxs\":[0,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}

#[test]
fn check_new_with_two_internal_insertion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 3.0], [3.0, 1.0]];

    let bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0, 0.0], 8.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":5,
            \"vcs\":[2.0,2.0,2.0,2.0,2.0,2.0,1.0,3.0,3.0,1.0],
            \"bcs\":[0.0,0.0,4.0,4.0,2.0,2.0,1.0,3.0,3.0,1.0],
            \"brs\":[8.0,4.0,2.0,1.0,1.0],
            \"ns\":[2,2,2,1,1],
            \"parents\":[null,0,1,2,2],
            \"from_dirs\":[null,3,0,1,2],
            \"vs\":[1.0,3.0,3.0,1.0],
            \"to_leafs\":[3,4],
            \"idxs\":[0,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);

    Ok(())
}

#[test]
fn check_new_with_root_expansion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 3.0]];

    let bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0, 0.0], 2.0, &vals);

    // assert_eq!(bht.to_string(), "{\"dim\":2,\"num\":1,\"vcs\":[1.0,3.0],\"bcs\":[2.0,2.0],\"brs\":[4.0],\"ns\":[1],\"parents\":[null],\"from_dirs\":[null]}");

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":1,
            \"vcs\":[1.0,3.0],
            \"bcs\":[2.0,2.0],
            \"brs\":[4.0],
            \"ns\":[1],
            \"parents\":[null],
            \"from_dirs\":[null],
            \"vs\":[1.0,3.0],
            \"to_leafs\":[0],
            \"idxs\":[0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);

    Ok(())
}

#[test]
fn check_new_with_leaf_expansion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [1.0, 3.0]];

    let bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0, 0.0], 2.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":3,
            \"vcs\":[1.0,2.0,1.0,1.0,1.0,3.0],
            \"bcs\":[2.0,2.0,0.0,0.0,0.0,4.0],
            \"brs\":[4.0,2.0,2.0],
            \"ns\":[2,1,1],
            \"parents\":[null,0,0],
            \"from_dirs\":[null,0,1],
            \"vs\":[1.0,1.0,1.0,3.0],
            \"to_leafs\":[1,2],
            \"idxs\":[0,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);

    Ok(())
}

#[test]
fn check_new_with_internal_expansion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [-1.0, -1.0], [1.0, 3.0]];

    let bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0, 0.0], 2.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":5,
            \"vcs\":[0.3333333333333333,1.0,1.0,3.0,0.0,0.0,-1.0,-1.0,1.0,1.0],
            \"bcs\":[2.0,2.0,0.0,4.0,0.0,0.0,-1.0,-1.0,1.0,1.0],
            \"brs\":[4.0,2.0,2.0,1.0,1.0],
            \"ns\":[3,1,2,1,1],
            \"parents\":[null,0,0,2,2],
            \"from_dirs\":[null,1,0,0,3],
            \"vs\":[1.0,1.0,-1.0,-1.0,1.0,3.0],
            \"to_leafs\":[4,3,1],
            \"idxs\":[0,0,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}

#[test]
fn check_new_with_two_internal_expansion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [-1.0, -1.0], [9.0, 9.0]];

    let bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0, 0.0], 2.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":6,
            \"vcs\":[3.0,3.0,9.0,9.0,0.0,0.0,0.0,0.0,-1.0,-1.0,1.0,1.0],
            \"bcs\":[6.0,6.0,10.0,10.0,2.0,2.0,0.0,0.0,-1.0,-1.0,1.0,1.0],
            \"brs\":[8.0,4.0,4.0,2.0,1.0,1.0],
            \"ns\":[3,1,2,2,1,1],
            \"parents\":[null,0,0,2,3,3],
            \"from_dirs\":[null,3,0,0,0,3],
            \"vs\":[1.0,1.0,-1.0,-1.0,9.0,9.0],
            \"to_leafs\":[5,4,1],
            \"idxs\":[0,0,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}

#[test]
fn check_new_with_adding_to_same_leaf() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [-1.0, -1.0], [9.0, 9.0]];

    let bht: BHTree<2> = BHTree::with_bounding_and_values_and_limit(&[0.0, 0.0], 2.0, &vals, 10.0);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":1,
            \"vcs\":[3.0,3.0],
            \"bcs\":[6.0,6.0],
            \"brs\":[8.0],
            \"ns\":[3],
            \"parents\":[null],
            \"from_dirs\":[null],
            \"vs\":[1.0,1.0,-1.0,-1.0,9.0,9.0],
            \"to_leafs\":[0,0,0],
            \"idxs\":[0,1,2]
        }",
    )?;

    let calc_bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}

#[test]
fn check_new_with_internal_insertion_and_some_adding_to_same_leaf(
) -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [-1.0, -1.0], [9.0, 9.0]];

    let bht: BHTree<2> = BHTree::with_bounding_and_values_and_limit(&[0.0, 0.0], 2.0, &vals, 2.0);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":4,
            \"vcs\":[3.0,3.0,9.0,9.0,0.0,0.0,0.0,0.0],
            \"bcs\":[6.0,6.0,10.0,10.0,2.0,2.0,0.0,0.0],
            \"brs\":[8.0,4.0,4.0,2.0],
            \"ns\":[3,1,2,2],
            \"parents\":[null,0,0,2],
            \"from_dirs\":[null,3,0,0],
            \"vs\":[1.0,1.0,-1.0,-1.0,9.0,9.0],
            \"to_leafs\":[3,3,1],
            \"idxs\":[0,1,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}

#[test]
fn check_new_with_internal_insertion_and_two_identical_values_adding_to_same_leaf(
) -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[0.0, 0.0], [0.0, 0.0], [9.0, 9.0]];

    let bht: BHTree<2> = BHTree::with_bounding_and_values_and_limit(&[0.0, 0.0], 2.0, &vals, 2.0);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":4,
            \"vcs\":[3.0,3.0,9.0,9.0,0.0,0.0,0.0,0.0],
            \"bcs\":[6.0,6.0,10.0,10.0,2.0,2.0,0.0,0.0],
            \"brs\":[8.0,4.0,4.0,2.0],
            \"ns\":[3,1,2,2],
            \"parents\":[null,0,0,2],
            \"from_dirs\":[null,3,0,0],
            \"vs\":[0.0,0.0,0.0,0.0,9.0,9.0],
            \"to_leafs\":[3,3,1],
            \"idxs\":[0,1,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}

#[test]
fn check_pushing_new_with_internal_insertion_and_some_adding_to_same_leaf(
) -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [-1.0, -1.0], [9.0, 9.0]];

    let mut bht: BHTree<2> = BHTree::with_bounding_and_capacity_and_limit(&[0.0, 0.0], 2.0, 3, 2.0);

    for value in vals {
        bht.push(&value);
    }

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":4,
            \"vcs\":[3.0,3.0,9.0,9.0,0.0,0.0,0.0,0.0],
            \"bcs\":[6.0,6.0,10.0,10.0,2.0,2.0,0.0,0.0],
            \"brs\":[8.0,4.0,4.0,2.0],
            \"ns\":[3,1,2,2],
            \"parents\":[null,0,0,2],
            \"from_dirs\":[null,3,0,0],
            \"vs\":[1.0,1.0,-1.0,-1.0,9.0,9.0],
            \"to_leafs\":[3,3,1],
            \"idxs\":[0,1,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}

#[test]
#[should_panic]
fn check_panic_when_bc_has_inf() {
    BHTree::with_bounding_and_capacity_and_limit(&[0.0, f64::INFINITY], 2.0, 3, 2.0);
}

#[test]
#[should_panic]
fn check_panic_when_br_is_nan() {
    BHTree::with_bounding_and_capacity_and_limit(&[0.0, 0.0], f64::NAN, 3, 2.0);
}

#[test]
#[should_panic]
fn check_panic_when_a_value_is_nan() {
    let vals: Vec<[f64; 2]> = vec![[1.0, f64::NAN], [-1.0, -1.0], [9.0, 9.0]];

    let mut bht: BHTree<2> = BHTree::with_bounding_and_capacity_and_limit(&[0.0, 0.0], 2.0, 3, 2.0);

    for value in vals {
        bht.push(&value);
    }
}

#[test]
#[should_panic]
fn check_panic_when_updated_value_is_nan() {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [-1.0, -1.0], [9.0, 9.0]];

    let mut bht: BHTree<2> = BHTree::with_bounding_and_capacity_and_limit(&[0.0, 0.0], 2.0, 3, 2.0);

    for value in vals {
        bht.push(&value);
    }

    bht.update(2, &[0.0, f64::NAN]);
}
