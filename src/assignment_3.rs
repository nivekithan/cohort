#![allow(dead_code)]

fn find_median(arr_1: &Vec<usize>, arr_2: &Vec<usize>) -> f64 {
    let mut merged_arr: Vec<usize> = Vec::new();

    merged_arr.extend(arr_1.iter());
    merged_arr.extend(arr_2.iter());

    merged_arr.sort_unstable();

    let len = merged_arr.len();

    assert_ne!(len, 0);

    let is_len_even = len % 2 == 0;

    if !is_len_even {
        let middle_index = ((len + 1) / 2) - 1;
        return *merged_arr.get(middle_index).unwrap() as f64;
    } else {
        let middle_index_1 = (len / 2) - 1;
        let middle_index_2 = len / 2;

        let middle_value_1 = *merged_arr.get(middle_index_1).unwrap();
        let middle_value_2 = *merged_arr.get(middle_index_2).unwrap();

        return (middle_value_1 + middle_value_2) as f64 / 2.0;
    }
}

#[cfg(test)]
mod test {
    use super::find_median;

    #[test]
    fn test_uneven_array() {
        let actual_median = find_median(&vec![1, 2, 3], &vec![4, 5, 6, 7]);

        assert_eq!(actual_median, 4.0);
    }

    #[test]
    #[should_panic]
    fn panic_if_both_empty_array() {
        find_median(&vec![], &vec![]);
    }

    #[test]
    fn unsorted_array() {
        let actual_median = find_median(&vec![5, 2, 1], &vec![6, 7, 3, 4]);

        assert_eq!(actual_median, 4.0);
    }

    #[test]
    fn repeating_numbers() {
        let actual_median = find_median(&vec![1, 2, 3, 1, 2, 3], &vec![3, 2, 1, 3, 2, 1]);

        assert_eq!(actual_median, 2.0);
    }

    #[test]
    fn even_length_merged_arr() {
        let acutal_median = find_median(&vec![1, 2, 3], &vec![4, 5, 6]);

        assert_eq!(acutal_median, 3.5);
    }

    #[test]
    fn one_array_is_empty() {
        let actual_median = find_median(&vec![1, 2, 3, 4], &vec![]);

        assert_eq!(actual_median, 2.5);
    }
}
