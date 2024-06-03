use std::ops::Range;
use zhifeng_impl_barnes_hut_tree as zbht;

use rand::Rng;
use zbht::BarnesHutTree as BHTree;

type Fnum = f64;
type Udim = usize;

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

pub fn check_vanillia_force_simulation_on_random_values(
    len: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    const D: usize = 2;
    let mut values = generate_random_values(len, &[-10.0..10.0, -10.0..10.0]);

    let calc_fn = zbht::utils::factory_of_repulsive_displacement_calc_fn::<D>(1.0, 0.2);
    for value_i in 0..len {
        let mut displacement = [0.0; D];

        for value_j in 0..len {
            if value_j == value_i {
                continue;
            }
            calc_fn(&values[value_i], &values[value_j], 1, &mut displacement);
        }

        let mut new_value = values[value_i].clone();
        for d in 0..D {
            new_value[d] += displacement[d];
        }

        values[value_i].clone_from(&new_value);
    }
    Ok(())
}

pub fn check_tree_force_simulation_on_random_values(
    len: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    const D: usize = 2;
    let values = generate_random_values(len, &[-10.0..10.0, -10.0..10.0]);
    let mut bht: BHTree<2> = BHTree::with_bounding_and_values(&[0.0, 0.0], 5.0, &values);

    let is_super_fn = zbht::utils::factory_of_is_super_node_fn::<D>(1.2);
    let calc_fn = zbht::utils::factory_of_repulsive_displacement_calc_fn::<2>(1.0, 0.2);
    for value_i in 0..len {
        let mut displacement = [0.0; D];

        bht.calc_force_on_value(value_i, &is_super_fn, &calc_fn, &mut displacement);

        let mut new_value = bht.get(value_i).expect("Should have").clone();
        for d in 0..D {
            new_value[d] += displacement[d];
        }

        bht.update(value_i, &new_value);
    }
    Ok(())
}
