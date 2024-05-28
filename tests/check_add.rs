use zhifeng_bhtree::BHTree;

#[test]
fn check_add_with_one_internal_insertion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 3.0], [3.0, 1.0]];

    let bht: BHTree<2> = BHTree::new_with_vec(&[0.0, 0.0], &4.0, &vals);

    assert_eq!(bht.to_string(), "{\"dim\":2,\"num\":4,\"vcs\":[4.0,4.0,4.0,4.0,1.0,3.0,3.0,1.0],\"bcs\":[0.0,0.0,2.0,2.0,1.0,3.0,3.0,1.0],\"brs\":[4.0,2.0,1.0,1.0],\"ns\":[2,2,1,1],\"parents\":[null,0,1,1],\"from_dirs\":[null,3,1,2]}");
    Ok(())
}

#[test]
fn check_add_with_two_internal_insertion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 3.0], [3.0, 1.0]];

    let bht: BHTree<2> = BHTree::new_with_vec(&[0.0, 0.0], &8.0, &vals);

    assert_eq!(bht.to_string(), "{\"dim\":2,\"num\":5,\"vcs\":[4.0,4.0,4.0,4.0,4.0,4.0,1.0,3.0,3.0,1.0],\"bcs\":[0.0,0.0,4.0,4.0,2.0,2.0,1.0,3.0,3.0,1.0],\"brs\":[8.0,4.0,2.0,1.0,1.0],\"ns\":[2,2,2,1,1],\"parents\":[null,0,1,2,2],\"from_dirs\":[null,3,0,1,2]}");
    Ok(())
}

#[test]
fn check_add_with_root_expansion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 3.0]];

    let bht: BHTree<2> = BHTree::new_with_vec(&[0.0, 0.0], &2.0, &vals);

    assert_eq!(bht.to_string(), "{\"dim\":2,\"num\":1,\"vcs\":[1.0,3.0],\"bcs\":[2.0,2.0],\"brs\":[4.0],\"ns\":[1],\"parents\":[null],\"from_dirs\":[null]}");
    Ok(())
}

#[test]
fn check_add_with_leaf_expansion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [1.0, 3.0]];

    let bht: BHTree<2> = BHTree::new_with_vec(&[0.0, 0.0], &2.0, &vals);

    assert_eq!(bht.to_string(), "{\"dim\":2,\"num\":3,\"vcs\":[2.0,4.0,1.0,1.0,1.0,3.0],\"bcs\":[2.0,2.0,0.0,0.0,0.0,4.0],\"brs\":[4.0,2.0,2.0],\"ns\":[2,1,1],\"parents\":[null,0,0],\"from_dirs\":[null,0,1]}");
    Ok(())
}

#[test]
fn check_add_with_internal_expansion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [-1.0, -1.0], [1.0, 3.0]];

    let bht: BHTree<2> = BHTree::new_with_vec(&[0.0, 0.0], &2.0, &vals);

    assert_eq!(bht.to_string(), "{\"dim\":2,\"num\":5,\"vcs\":[1.0,3.0,1.0,3.0,0.0,0.0,-1.0,-1.0,1.0,1.0],\"bcs\":[2.0,2.0,0.0,4.0,0.0,0.0,-1.0,-1.0,1.0,1.0],\"brs\":[4.0,2.0,2.0,1.0,1.0],\"ns\":[3,1,2,1,1],\"parents\":[null,0,0,2,2],\"from_dirs\":[null,1,0,0,3]}");
    Ok(())
}

#[test]
fn check_add_with_two_internal_expansion() -> Result<(), Box<dyn std::error::Error>> {
    let vals: Vec<[f64; 2]> = vec![[1.0, 1.0], [-1.0, -1.0], [7.0, 7.0]];

    let bht: BHTree<2> = BHTree::new_with_vec(&[0.0, 0.0], &2.0, &vals);

    assert_eq!(bht.to_string(), "{\"dim\":2,\"num\":6,\"vcs\":[7.0,7.0,7.0,7.0,0.0,0.0,0.0,0.0,-1.0,-1.0,1.0,1.0],\"bcs\":[6.0,6.0,10.0,10.0,2.0,2.0,0.0,0.0,-1.0,-1.0,1.0,1.0],\"brs\":[8.0,4.0,4.0,2.0,1.0,1.0],\"ns\":[3,1,2,2,1,1],\"parents\":[null,0,0,2,3,3],\"from_dirs\":[null,3,0,0,0,3]}");
    Ok(())
}
