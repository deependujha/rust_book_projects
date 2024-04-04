fn main() {
    let mut v = vec![31, 21, 35, 43];
    println!("Original vector: {:?}", v);
    selection_sort(&mut v); // passing mutable reference, bcoz we need to mutate vector in order to sort it
    println!("vector after sorting: {:?}", v);
    my_separator_printer(); // helper function to print separator
    let my_v_sum = sum_of_vector(&v); // passing immutable reference, bcoz we don't need to mutate vector to find its sum.
    println!("sum of vector: {my_v_sum}");
    my_separator_printer();
    let my_odd_v_sum = sum_of_odd_idx_in_vector(&v);
    println!("sum of items at odd indices: {my_odd_v_sum}");
}

fn selection_sort(v: &mut Vec<i32>) {
    for i in 0..(v.len()) {
        let mut min_idx = i;
        for j in i + 1..(v.len()) {
            if v[min_idx] > v[j] {
                min_idx = j;
            }
        }
        (v[i], v[min_idx]) = (v[min_idx], v[i]); // swap `i` with `min_idx`
    }
}

fn sum_of_vector(v: &Vec<i32>) -> i32 {
    let mut my_sum = 0;
    for i in v {
        my_sum += i;
    }
    return my_sum;
}

fn sum_of_odd_idx_in_vector(v: &Vec<i32>) -> i32 {
    // function to demonstrate for loop with step_size 2.
    let mut my_sum = 0;
    for i in (0..v.len()).step_by(2) {
        my_sum += v[i];
    }
    return my_sum;
}

fn my_separator_printer() {
    let my_separator = String::from("=").repeat(50);
    println!("{my_separator}");
}
