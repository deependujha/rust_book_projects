pub fn sum_of_odd_idx_in_vector(v: &Vec<i32>) -> i32 {
    // function to demonstrate for loop with step_size 2.
    let mut my_sum = 0;
    for i in (0..v.len()).step_by(2) {
        my_sum += v[i];
    }
    return my_sum;
}
