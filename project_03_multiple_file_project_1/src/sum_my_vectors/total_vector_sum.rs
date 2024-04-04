pub fn sum_of_vector(v: &Vec<i32>) -> i32 {
    let mut my_sum = 0;
    for i in v {
        my_sum += i;
    }
    return my_sum;
}
