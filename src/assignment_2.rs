#![allow(dead_code)]

use std::fmt::Display;

pub fn increment_by_n<T>(collection: &Vec<T>, n: T) -> Vec<T>
where
    T: std::ops::Add<T> + Copy,
    Vec<T>: FromIterator<T::Output>,
{
    return collection
        .iter()
        .map(|&v| {
            return v + n;
        })
        .collect();
}

pub fn filter_by_condition<T, F>(collection: &Vec<T>, cb: F) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    let mut output: Vec<T> = Vec::new();

    collection.iter().for_each(|v| {
        let should_include = cb(v);

        if should_include {
            output.push(*v);
        }
    });

    return output;
}

pub fn transform_to_string<T, F>(collection: &Vec<T>, cb: F) -> Vec<String>
where
    F: Fn(&T) -> String,
{
    let mut ouput: Vec<String> = Vec::new();

    collection.iter().for_each(|v| {
        let transformed_value = cb(v);

        ouput.push(transformed_value);
    });

    return ouput;
}

pub fn display_vectors<T>(collection: &Vec<T>)
where
    T: Display,
{
    collection.iter().for_each(|v| {
        println!("{}", v);
    })
}

pub fn increment_by_3_and_display_even_numbers(collection: &Vec<usize>) {
    let output = increment_by_n(collection, 3);
    let output = filter_by_condition(&output, |v| v % 2 == 0);
    let output = transform_to_string(&output, |v| format!("{}", v));

    display_vectors(&output);
}

#[cfg(test)]
mod tests {

    use super::{filter_by_condition, increment_by_n, transform_to_string};

    #[test]
    fn test_increment_by_n() {
        let collection = vec![1, 2, 3, 4, 5, 6];

        let new_collection = increment_by_n(&collection, 10);

        assert_eq!(new_collection, vec![11, 12, 13, 14, 15, 16]);
    }

    #[test]
    fn test_filter_by_condition() {
        let collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let new_collection = filter_by_condition(&collection, |v| {
            return v % 2 == 0;
        });

        assert_eq!(new_collection, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_transform_to_string() {
        let collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let new_collection = transform_to_string(&collection, |v| {
            return format!("{}", v);
        });

        assert_eq!(
            new_collection,
            vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]
        );
    }
}
