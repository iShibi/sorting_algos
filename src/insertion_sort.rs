use crate::utils::swap_values;

pub fn insertion_sort(mut arr: Vec<isize>) -> Vec<isize> {
    let arr_size = arr.len();
    for i in 0..arr_size - 1 {
        for j in (1..=i + 1).rev() {
            if arr[j] < arr[j - 1] {
                swap_values(&mut arr, j, j - 1);
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort() {
        let result = insertion_sort(vec![2, 4, 1, 3]);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }
}
