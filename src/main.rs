use zhifeng_bhtree::BHTree;

fn main() {
    let t: BHTree<2> = BHTree::new_with_vec(&[0.0, 0.0], &2.0, &[[0.5, 0.5], [1.5, 1.5]]);
    println!("{}", t);
}
