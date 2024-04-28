pub fn swap_values(arr: &mut Vec<isize>, index_a: usize, index_b: usize) {
    arr[index_a] = arr[index_a] + arr[index_b]; // a = a + b
    arr[index_b] = arr[index_a] - arr[index_b]; // b = a - b => (a + b) - b => a
    arr[index_a] = arr[index_a] - arr[index_b]; // a = a - b => (a + b) - a => b
}
