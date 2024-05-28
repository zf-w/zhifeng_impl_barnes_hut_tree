use zhifeng_bhtree::{BHTree, BHTreeSer, Udim};

fn assert_bht_serde<const D: Udim>(calc_bht_ser: &BHTreeSer<D>, expected_bht_ser: &BHTreeSer<D>) {
    assert_eq!(
        calc_bht_ser.get_num(),
        expected_bht_ser.get_num(),
        "Nodes Number"
    );
    assert_eq!(
        calc_bht_ser.get_bcs(),
        expected_bht_ser.get_bcs(),
        "Bounding Box Centers"
    );
    assert_eq!(
        calc_bht_ser.get_vcs(),
        expected_bht_ser.get_vcs(),
        "Value Centers"
    );
    assert_eq!(
        calc_bht_ser.get_ns(),
        expected_bht_ser.get_ns(),
        "Count Inside Nodes"
    );
    assert_eq!(
        calc_bht_ser.get_parents(),
        expected_bht_ser.get_parents(),
        "Parents"
    );
    assert_eq!(
        calc_bht_ser.get_from_dirs(),
        expected_bht_ser.get_from_dirs()
    );
    assert_eq!(calc_bht_ser.get_vs(), expected_bht_ser.get_vs(), "Values");
    assert_eq!(
        calc_bht_ser.get_to_leafs(),
        expected_bht_ser.get_to_leafs(),
        "To Leafs"
    );
    assert_eq!(
        calc_bht_ser.get_idxs(),
        expected_bht_ser.get_idxs(),
        "Indices in Leafs"
    );
}

#[test]
fn check_add_to_empty() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0]];

    let bht: BHTree<2> = BHTree::new_with_arr(&[0.0, 0.0], &4.0, &vals);

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
    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}

#[test]
fn check_add_with_one_internal_insertion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 3.0], [3.0, 1.0]];

    let bht: BHTree<2> = BHTree::new_with_arr(&[0.0, 0.0], &4.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":4,
            \"vcs\":[4.0,4.0,4.0,4.0,1.0,3.0,3.0,1.0],
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

    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}

#[test]
fn check_add_with_two_internal_insertion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 3.0], [3.0, 1.0]];

    let bht: BHTree<2> = BHTree::new_with_arr(&[0.0, 0.0], &8.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":5,
            \"vcs\":[4.0,4.0,4.0,4.0,4.0,4.0,1.0,3.0,3.0,1.0],
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

    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde(&calc_bht_ser, &expected_bht_ser);

    Ok(())
}

#[test]
fn check_add_with_root_expansion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 3.0]];

    let bht: BHTree<2> = BHTree::new_with_arr(&[0.0, 0.0], &2.0, &vals);

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

    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde(&calc_bht_ser, &expected_bht_ser);

    Ok(())
}

#[test]
fn check_add_with_leaf_expansion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [1.0, 3.0]];

    let bht: BHTree<2> = BHTree::new_with_arr(&[0.0, 0.0], &2.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":3,
            \"vcs\":[2.0,4.0,1.0,1.0,1.0,3.0],
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

    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde(&calc_bht_ser, &expected_bht_ser);

    Ok(())
}

#[test]
fn check_add_with_internal_expansion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [-1.0, -1.0], [1.0, 3.0]];

    let bht: BHTree<2> = BHTree::new_with_arr(&[0.0, 0.0], &2.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":5,
            \"vcs\":[1.0,3.0,1.0,3.0,0.0,0.0,-1.0,-1.0,1.0,1.0],
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

    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}

#[test]
fn check_add_with_two_internal_expansion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [-1.0, -1.0], [7.0, 7.0]];

    let bht: BHTree<2> = BHTree::new_with_arr(&[0.0, 0.0], &2.0, &vals);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":6,
            \"vcs\":[7.0,7.0,7.0,7.0,0.0,0.0,0.0,0.0,-1.0,-1.0,1.0,1.0],
            \"bcs\":[6.0,6.0,10.0,10.0,2.0,2.0,0.0,0.0,-1.0,-1.0,1.0,1.0],
            \"brs\":[8.0,4.0,4.0,2.0,1.0,1.0],
            \"ns\":[3,1,2,2,1,1],
            \"parents\":[null,0,0,2,3,3],
            \"from_dirs\":[null,3,0,0,0,3],
            \"vs\":[1.0,1.0,-1.0,-1.0,7.0,7.0],
            \"to_leafs\":[5,4,1],
            \"idxs\":[0,0,0]
        }",
    )?;

    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}

#[test]
fn check_add_with_adding_to_same_leaf() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [-1.0, -1.0], [7.0, 7.0]];

    let bht: BHTree<2> = BHTree::new_with_arr_and_limit(&[0.0, 0.0], &2.0, &vals, 10.0);

    let expected_bht_ser: BHTreeSer<2> = serde_json::from_str(
        "{
            \"dim\":2,
            \"num\":1,
            \"vcs\":[7.0,7.0],
            \"bcs\":[6.0,6.0],
            \"brs\":[8.0],
            \"ns\":[3],
            \"parents\":[null],
            \"from_dirs\":[null],
            \"vs\":[1.0,1.0,-1.0,-1.0,7.0,7.0],
            \"to_leafs\":[0,0,0],
            \"idxs\":[0,1,2]
        }",
    )?;

    let calc_bht_ser = bht.calc_serde_bhtree();
    assert_bht_serde(&calc_bht_ser, &expected_bht_ser);
    Ok(())
}
