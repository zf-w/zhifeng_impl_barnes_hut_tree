use std::ops::Range;
use zhifeng_impl_barnes_hut_tree as zbht;

use rand::Rng;
use zbht::BarnesHutTree as BHTree;

type Fnum = f64;
type Udim = usize;

#[test]
fn check_calc_on_tree_with_one_internal_insertion() -> Result<(), Box<dyn std::error::Error>> {
    const D: usize = 2;
    let vals: Vec<[f64; D]> = vec![[1.0, 3.0], [3.0, 1.0]];

    let bht: BHTree<D> = BHTree::with_bounding_and_values(&[0.0, 0.0], 4.0, &vals);

    let calc_fn = zbht::utils::factory_of_repulsive_displacement_calc_fn::<2>(1.0, 0.2);

    let mut displacement = [0.0; D];
    bht.calc_force_on_value(0, |_, _, _| -> bool { false }, &calc_fn, &mut displacement);
    let mut expected_displacement = [0.0; D];
    calc_fn(&[1.0, 3.0], &[3.0, 1.0], 1, &mut expected_displacement);
    assert_eq!(displacement, expected_displacement);
    println!("{:?}", displacement);
    Ok(())
}

fn generate_random_values<const D: Udim>(len: usize, ranges: &[Range<Fnum>; D]) -> Vec<[Fnum; D]> {
    let mut ans_vec: Vec<[Fnum; D]> = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();

    for _ in 0..len {
        let mut curr = [0.0; D];
        for d in 0..D {
            curr[d] = rng.gen_range(ranges[d].clone());
        }

        ans_vec.push(curr);
    }
    ans_vec
}

fn assert_values_close<const D: Udim>(value_0: &[Fnum; D], value_1: &[Fnum; D], limit: Fnum) {
    let mut close = true;
    for d in 0..D {
        if (value_0[d] - value_1[d]).abs() > limit {
            close = false;
        }
    }
    assert!(close, "     Got:{:?}\nExpected:{:?}", value_0, value_1);
}

fn assert_values_and_energy_close<const D: Udim>(
    value_0: &([Fnum; D], Fnum),
    value_1: &([Fnum; D], Fnum),
    limit: Fnum,
) {
    let mut close = true;
    for d in 0..D {
        if (value_0.0[d] - value_1.0[d]).abs() > limit {
            close = false;
        }
    }
    if (value_0.1 - value_1.1).abs() > limit {
        close = false;
    }
    assert!(close, "     Got:{:?}\nExpected:{:?}", value_0, value_1);
}

#[test]
fn check_exact_calc_on_100_random_values() -> Result<(), Box<dyn std::error::Error>> {
    const D: usize = 2;
    let len = 100;
    let values = generate_random_values(len, &[-10.0..10.0, -10.0..10.0]);
    let bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0, 0.0], 5.0, &values);

    let calc_fn = zbht::utils::factory_of_repulsive_displacement_calc_fn::<2>(1.0, 0.2);

    for value_i in 0..len {
        let mut displacement = [0.0; D];
        let mut expected_displacement = [0.0; D];
        bht.calc_force_on_value(
            value_i,
            |_, _, _| -> bool { false },
            &calc_fn,
            &mut displacement,
        );

        for value_j in 0..len {
            if value_j == value_i {
                continue;
            }
            calc_fn(
                &values[value_i],
                &values[value_j],
                1,
                &mut expected_displacement,
            );
        }

        assert_values_close(&displacement, &expected_displacement, 1e-9);
    }
    Ok(())
}

#[test]
fn check_exact_force_simulation_on_100_random_values_with_energy(
) -> Result<(), Box<dyn std::error::Error>> {
    const D: usize = 2;
    let len = 100;
    let mut values = generate_random_values(len, &[-10.0..10.0, -10.0..10.0]);
    let mut bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0, 0.0], 5.0, &values);

    let calc_fn = zbht::utils::factory_of_repulsive_displacement_with_energy_calc_fn::<2>(1.0, 0.2);
    for value_i in 0..len {
        let mut displacement = ([0.0; D], 0.0);
        let mut expected_displacement = ([0.0; D], 0.0);
        bht.calc_force_on_value(
            value_i,
            |_, _, _| -> bool { false },
            &calc_fn,
            &mut displacement,
        );

        for value_j in 0..len {
            if value_j == value_i {
                continue;
            }
            calc_fn(
                &values[value_i],
                &values[value_j],
                1,
                &mut expected_displacement,
            );
        }

        assert_values_and_energy_close(&displacement, &expected_displacement, 1e-9);

        let mut new_value = values[value_i].clone();
        for d in 0..D {
            new_value[d] += displacement.0[d];
        }

        values[value_i].clone_from(&new_value);
        bht.update(value_i, &new_value);
    }
    Ok(())
}

#[test]
fn check_exact_force_simulation_on_1000_random_values() -> Result<(), Box<dyn std::error::Error>> {
    const D: usize = 2;
    let len = 1000;
    let mut values = generate_random_values(len, &[-10.0..10.0, -10.0..10.0]);
    let mut bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0, 0.0], 5.0, &values);

    let calc_fn = zbht::utils::factory_of_repulsive_displacement_calc_fn::<2>(1.0, 0.2);
    for value_i in 0..len {
        let mut displacement = [0.0; D];
        let mut expected_displacement = [0.0; D];
        bht.calc_force_on_value(
            value_i,
            |_, _, _| -> bool { false },
            &calc_fn,
            &mut displacement,
        );

        for value_j in 0..len {
            if value_j == value_i {
                continue;
            }
            calc_fn(
                &values[value_i],
                &values[value_j],
                1,
                &mut expected_displacement,
            );
        }

        assert_values_close(&displacement, &expected_displacement, 1e-9);

        let mut new_value = values[value_i].clone();
        for d in 0..D {
            new_value[d] += displacement[d];
        }

        values[value_i].clone_from(&new_value);
        bht.update(value_i, &new_value);
    }
    Ok(())
}
