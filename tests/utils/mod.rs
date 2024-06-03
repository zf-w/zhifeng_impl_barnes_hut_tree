use std::fmt::Debug;

use zhifeng_impl_barnes_hut_tree as zbht;

use zbht::BarnesHutTreeSer;

type Udim = usize;

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
