pub fn selection_sort(v: &mut Vec<i32>) {
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
