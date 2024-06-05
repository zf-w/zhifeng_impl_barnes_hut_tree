# Barnes-Hut Tree for Quick N-body Force Calculation

Thank you for checking out the crate. This is Zhifeng's coding practice of implementing Barnes-Hut Tree for accelerated N-body force calculation.

The crate consists of two parts: the [BarnesHutTree] struct and the [utils] module containing a few helper functions. The [BarnesHutTree] is a data structure that accelerates the N-body force calculation by approximating and treating groups of "bodies" relatively "far" away as a super node. To make the force calculation more customizable, the [BarnesHutTree] takes in a closure (Anonymous Function) determining whether a current node can be seen as "far enough" and a closure determining how to calculate the force or any other relationship between the target node and the super node. The [utils] module contains several closure factory functions of the repulsive force calculation mentioned in a ([paper](#reference)) about graph drawing.

## Usage Example

```rust
use zhifeng_impl_barnes_hut_tree as zbht;

use zbht::BarnesHutTree as BHTree;

let mut bht: BHTree<2> = BHTree::new();
bht.push(&[-1.0,1.0]);
bht.push(&[1.0,1.0]);

let mut ans_displacement = [0.0; 2];

let calc_fn = zbht::utils::factory_of_repulsive_displacement_calc_fn::<2>(1.0, 0.2);

// Since the closure implies every super node is not "far" enough,
// the line is calculating the exact displacement acting on value 0.
bht.calc_force_on_value(0, |_,_,_| -> bool {false}, &calc_fn, &mut ans_displacement);

assert_eq!(ans_displacement, [(-2.0 * 0.2) / (2.0 * 2.0), 0.0]);

let is_super = zbht::utils::factory_of_is_super_node_fn::<2>(1.2);
// Calculating the approximated displacement acting on value 0.
bht.calc_force_on_value(0, &is_super, &calc_fn, &mut ans_displacement);
```

## Design and Naming

To make the arguments in constructors clearer, below are the key terms I used in the implementation.

- The position of each "body" in the context of "n-body" calculation is called `value`. (Currently, the tree treats each body as equal.)

- The Barnes Hut Tree uses a tree of hyper cubes to quickly find groups of nodes relatively close with each other and calculate the average position of the group of `value`s before hand to accelerate the calculation process.

- The average of `value`s in a node is called value center, `vc` in code.

- The bounding box center, which is the center of a hyper cube, of nodes is called `bc` for short.

- The bounding box radius, which is the half width of a hyper cube, of nodes is called `br` for short.

To make the design safer, I internally use `vec` to store nodes: internal nodes and leaf nodes. When a node needs to be removed, if it is not the last node, the last node in `vec` will replace its place and update the index-based virtual "pointers".

### Current Approach of Handling Too Close or Identical Values

Since the tree is generally trying to put each value into separated leaf to calculate the super nodes, without additional checking or handling, trying to insert two identical value will create an infinte loop.

The tree currently handles two too-close values by allowing us to pre-set a limit bound that the tree node will not continue to divide when its hypercube's radius (half-width) is less than or equal to that bound. When calculating "force", the tree will loop through every value in the same leaf node except the target value itself.

We need to be extra careful when setting this limit value. If the limit is too big, too many values will be held in a single leaf node, resulting in a decrease in efficiency. If all the values are in one single leaf node, the behavior and efficiency of the tree will be the same as looping through all the nodes: the default implementation of N-body calculation. Currently, even though the `f64` will eventually reach zero after some dividing, the limit should be larger than zero and be finite.

### Current Approach of Handling Out-of-root-bounding-range Values

Currently, the tree will try to double its width by creating a new internal node with a larger radius in the direction of the to-include value (similarly for none or only one leaf node above the minimum radius limit). We need to be careful when the to-include value is too far away, creating numeric issues like `INFINITY` or `NaN`.

## Overall Panics

The current tree will panic if any 64-bit float turned to be not finite during construction and value updates.

## Reference

Hu, Y. (2005). Efficient, high-quality force-directed graph drawing. _Mathematica journal, 10_(1), 37-71.

I got the idea of the Barnes-Hut Tree from section four "Barnes-Hut Force Calculation" of this paper. I have put some of my independent thoughts into my implementation (which might be error-prone). For example, in the paper, the author didn't focus on the "value removing" part of the Barnes-Hut Tree because rebuilding the octree structure is adequately efficient, so I need to think about it independently.

## License: AGPL-3.0-only

Zhifeng's Implementation of Barnes-Hut Tree For Accelerated N-body Calculation
Copyright (C) 2024 Zhifeng Wang

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, version 3.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
