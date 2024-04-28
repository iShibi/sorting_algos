use crate::utils::swap_values;

pub fn bubble_sort(mut arr: Vec<isize>) -> Vec<isize> {
    let arr_size = arr.len();
    for i in 0..arr_size {
        let mut is_swaped = false;
        for j in 0..arr_size - i {
            if j + 1 < arr_size && arr[j] > arr[j + 1] {
                swap_values(&mut arr, j, j + 1);
                is_swaped = true
            }
        }
        if is_swaped == false {
            break;
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort() {
        let result = bubble_sort(vec![2, 4, 1, 3]);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }
}
