mod my_separator;
mod sort_vectors;
mod sum_my_vectors;

use my_separator::my_separator_printer;
use sort_vectors::selection_sort;
use sum_my_vectors::{odd_indices_vector_sum, total_vector_sum};

fn main() {
    let mut v = vec![31, 21, 35, 43];
    println!("Original vector: {:?}", v);
    selection_sort(&mut v); // passing mutable reference, bcoz we need to mutate vector in order to sort it
    println!("vector after sorting: {:?}", v);
    my_separator_printer(); // helper function to print separator
    let my_v_sum = total_vector_sum::sum_of_vector(&v); // passing immutable reference, bcoz we don't need to mutate vector to find its sum.
    println!("sum of vector: {my_v_sum}");
    my_separator_printer();
    let my_odd_v_sum = odd_indices_vector_sum::sum_of_odd_idx_in_vector(&v);
    println!("sum of items at odd indices: {my_odd_v_sum}");
}
