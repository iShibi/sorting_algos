use crate::utils::swap_values;

pub fn selection_sort(mut arr: Vec<isize>) -> Vec<isize> {
    let arr_size = arr.len();
    for i in 0..arr_size - 1 {
        let mut smallest = i;
        for j in i + 1..arr_size {
            if arr[j] < arr[smallest] {
                smallest = j;
            }
        }
        swap_values(&mut arr, i, smallest);
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort() {
        let sorted = selection_sort(vec![2, 4, 1, 3]);
        assert_eq!(sorted, vec![1, 2, 3, 4]);
    }
}
