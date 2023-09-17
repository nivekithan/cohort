#![allow(dead_code)]

use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

struct BloomFilter {
    storage: HashMap<u64, usize>,
}

impl BloomFilter {
    pub fn new() -> Self {
        return BloomFilter {
            storage: HashMap::new(),
        };
    }

    fn calculate_signature<T>(value: &T) -> [u64; 3]
    where
        T: Hash,
    {
        let mut metro_hasher: fasthash::MetroHasher = Default::default();
        let mut lookup3_hasher: fasthash::Lookup3Hasher = Default::default();
        let mut mum_hasher: fasthash::MumHasher = Default::default();

        value.hash(&mut metro_hasher);
        value.hash(&mut lookup3_hasher);
        value.hash(&mut mum_hasher);

        let signature = [
            metro_hasher.finish(),
            lookup3_hasher.finish(),
            mum_hasher.finish(),
        ];

        return signature;
    }

    fn insert_signature(&mut self, signature: [u64; 3]) {
        signature.iter().for_each(|signature_item| {
            self.storage.insert(*signature_item, 1);
        })
    }

    fn check_signature(&self, signature: [u64; 3]) -> [usize; 3] {
        let default_value: &usize = &0;
        return [
            *self
                .storage
                .get(signature.get(0).unwrap())
                .unwrap_or(default_value),
            *self
                .storage
                .get(signature.get(1).unwrap())
                .unwrap_or(default_value),
            *self
                .storage
                .get(signature.get(2).unwrap())
                .unwrap_or(default_value),
        ];
    }

    pub fn add<T>(&mut self, value: &T)
    where
        T: Hash,
    {
        let signature = Self::calculate_signature(value);

        self.insert_signature(signature);
    }

    pub fn check<T>(&self, value: &T) -> bool
    where
        T: Hash,
    {
        let signature = Self::calculate_signature(value);

        return self.check_signature(signature) == [1, 1, 1];
    }
}

#[cfg(test)]
mod test {
    use super::BloomFilter;

    #[test]
    fn test_bloom_filter() {
        let mut bloom_filter = BloomFilter::new();

        let numbers = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11"];

        numbers.iter().for_each(|v| {
            bloom_filter.add(v);
        });

        numbers.iter().for_each(|v| {
            assert_eq!(bloom_filter.check(v), true);
        })
    }
}
