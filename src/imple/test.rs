#[cfg(test)]
use crate::{BarnesHutTree, BarnesHutTreeSer};
#[cfg(test)]
use std::fmt::Debug;

#[cfg(test)]
type Udim = usize;

#[cfg(test)]
pub fn assert_bht_serde_eq<const D: Udim>(
    calc_bht_ser: &BarnesHutTreeSer<D>,
    expected_bht_ser: &BarnesHutTreeSer<D>,
) {
    let mut all_match = true;

    fn assert_print<T: PartialEq + Debug>(got: &T, expected: &T, all_match: &mut bool, name: &str) {
        if got != expected {
            *all_match = false;
            println!(
                "{} do not match.\nExpected: {:?}\n     Got: {:?}",
                name, expected, got
            );
        }
    }

    assert_print(
        calc_bht_ser.get_num(),
        expected_bht_ser.get_num(),
        &mut all_match,
        "Tree: Total Node Number",
    );
    assert_print(
        calc_bht_ser.get_bcs(),
        expected_bht_ser.get_bcs(),
        &mut all_match,
        "Node: Bounding Box Centers",
    );
    assert_print(
        calc_bht_ser.get_brs(),
        expected_bht_ser.get_brs(),
        &mut all_match,
        "Node: Bounding Box Ranges",
    );
    assert_print(
        calc_bht_ser.get_vcs(),
        expected_bht_ser.get_vcs(),
        &mut all_match,
        "Node: Value Centers",
    );
    assert_print(
        calc_bht_ser.get_ns(),
        expected_bht_ser.get_ns(),
        &mut all_match,
        "Node: Number of Values Inside",
    );
    assert_print(
        calc_bht_ser.get_parents(),
        expected_bht_ser.get_parents(),
        &mut all_match,
        "Node: Parents",
    );

    assert_print(
        calc_bht_ser.get_from_dirs(),
        expected_bht_ser.get_from_dirs(),
        &mut all_match,
        "Node: From which direction",
    );

    assert_print(
        calc_bht_ser.get_vs(),
        expected_bht_ser.get_vs(),
        &mut all_match,
        "Value: value of bodies",
    );

    assert_print(
        calc_bht_ser.get_to_leafs(),
        expected_bht_ser.get_to_leafs(),
        &mut all_match,
        "Value: each value's corresponding leaf node",
    );

    assert_print(
        calc_bht_ser.get_idxs(),
        expected_bht_ser.get_idxs(),
        &mut all_match,
        "Value: each value's corresponding index inside leaf node",
    );
    assert!(all_match);
}

#[test]
fn check_sub_from_leaf_tree() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0]];

    let mut bht: BarnesHutTree<2> =
        BarnesHutTree::with_bounding_and_values(&[0.0, 0.0], 4.0, &vals);

    let expected_bht_ser: BarnesHutTreeSer<2> = serde_json::from_str(
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
    let bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&bht_ser, &expected_bht_ser);

    bht.sub(0);

    let expected_bht_ser: BarnesHutTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":0,
            \"vcs\":[],
            \"bcs\":[],
            \"brs\":[],
            \"ns\":[],
            \"parents\":[],
            \"from_dirs\":[],

            \"vs\":[1.0,1.0],
            \"to_leafs\":[null],
            \"idxs\":[null]
        }",
    )
    .unwrap();
    let bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&bht_ser, &expected_bht_ser);

    Ok(())
}

#[test]
fn check_sub_from_tree_with_one_internal_insertion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 3.0], [3.0, 1.0]];

    let mut bht: BarnesHutTree<2> =
        BarnesHutTree::with_bounding_and_values(&[0.0, 0.0], 4.0, &vals);

    let expected_bht_ser: BarnesHutTreeSer<2> = serde_json::from_str(
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

    let bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&bht_ser, &expected_bht_ser);

    bht.sub(0);

    let expected_bht_ser: BarnesHutTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":1,
            \"vcs\":[3.0,1.0],
            \"bcs\":[0.0,0.0],
            \"brs\":[4.0],
            \"ns\":[1],

            \"parents\":[null],
            \"from_dirs\":[null],
            \"vs\":[1.0,3.0,3.0,1.0],
            \"to_leafs\":[null,0],
            \"idxs\":[null,0]
        }",
    )?;

    let bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&bht_ser, &expected_bht_ser);

    Ok(())
}

#[test]
fn check_sub_from_tree_with_two_internal_insertion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 3.0], [3.0, 1.0]];

    let mut bht: BarnesHutTree<2> =
        BarnesHutTree::with_bounding_and_values(&[0.0, 0.0], 8.0, &vals);

    let expected_bht_ser: BarnesHutTreeSer<2> = serde_json::from_str(
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

    let bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&bht_ser, &expected_bht_ser);

    bht.sub(0);

    let expected_bht_ser: BarnesHutTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":1,
            \"vcs\":[3.0,1.0],
            \"bcs\":[0.0,0.0],
            \"brs\":[8.0],
            \"ns\":[1],

            \"parents\":[null],
            \"from_dirs\":[null],
            \"vs\":[1.0,3.0,3.0,1.0],
            \"to_leafs\":[null,0],
            \"idxs\":[null,0]
        }",
    )?;

    let bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&bht_ser, &expected_bht_ser);

    Ok(())
}

#[test]
fn check_sub_from_tree_with_adding_to_same_leaf() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [-1.0, -1.0], [9.0, 9.0]];

    let mut bht: BarnesHutTree<2> =
        BarnesHutTree::with_bounding_and_values_and_limit(&[0.0, 0.0], 2.0, &vals, 10.0);

    let expected_bht_ser: BarnesHutTreeSer<2> = serde_json::from_str(
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

    let bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&bht_ser, &expected_bht_ser);

    bht.sub(0);

    let expected_bht_ser: BarnesHutTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":1,
            \"vcs\":[4.0,4.0],
            \"bcs\":[6.0,6.0],
            \"brs\":[8.0],
            \"ns\":[2],

            \"parents\":[null],
            \"from_dirs\":[null],
            \"vs\":[1.0,1.0,-1.0,-1.0,9.0,9.0],
            \"to_leafs\":[null,0,0],
            \"idxs\":[null,1,0]
        }",
    )?;

    let bht_ser = bht.calc_serialized();
    assert_bht_serde_eq(&bht_ser, &expected_bht_ser);
    Ok(())
}
