#![allow(dead_code)]

use std::hash::{Hash, Hasher};

use bitvec::prelude::BitVec;

struct BloomFilter {
    storage: BitVec,
    no_of_hashes: u64,
}

impl BloomFilter {
    pub fn new(size_of_bit: usize, no_of_hashes: u64) -> Self {
        return BloomFilter {
            storage: BitVec::repeat(false, size_of_bit),
            no_of_hashes,
        };
    }

    pub fn add<T>(&mut self, value: &T)
    where
        T: Hash,
    {
        let hash1 = Self::murmur_hash(value);
        let hash2 = Self::lookup3_hash(value);

        self.indicies(hash1, hash2).for_each(|hash| {
            self.storage.set(hash as usize, true);
        })
    }

    pub fn check<T>(&self, value: &T) -> bool
    where
        T: Hash,
    {
        let hash1 = Self::murmur_hash(value);
        let hash2 = Self::lookup3_hash(value);

        self.indicies(hash1, hash2)
            .all(|hash| self.storage[hash as usize])
    }

    fn murmur_hash<T>(value: &T) -> u64
    where
        T: Hash,
    {
        let mut murmur_hasher: fasthash::MurmurHasher = Default::default();

        value.hash(&mut murmur_hasher);

        return murmur_hasher.finish();
    }

    fn lookup3_hash<T>(value: &T) -> u64
    where
        T: Hash,
    {
        let mut lookup3_hasher: fasthash::Lookup3Hasher = Default::default();

        value.hash(&mut lookup3_hasher);

        return lookup3_hasher.finish();
    }

    fn indicies(&self, hash1: u64, hash2: u64) -> impl Iterator<Item = u64> {
        // We generate `no_of_hashes`number of hashes using algorithm
        // g_i = h_1 + i * h2 from "Less hashing, same performance: Building a better
        // bloom filter" by Kirtz and Mitzenmacher

        let len = self.storage.len() as u64;
        ((0u64)..self.no_of_hashes)
            // In rust arithmetic operations are not expected to overflow, when it does overflow it
            // may panic (in certain configuration) too. Therefore for algorithm which expects and relies
            // upon modular arithmetic we should use `wrapping_*` functions
            .map(move |i| hash1.wrapping_add((hash2).wrapping_mul(i)))
            .map(move |new_hash| (new_hash % len))
    }
}

#[cfg(test)]
mod test {
    use super::BloomFilter;

    #[test]
    fn test_bloom_filter() {
        let value: Vec<String> = (0..10).map(|v| format!("{}", v)).collect();

        // Expected FP rate is 1.0E-7 and since we are only storing 11 (0..10) items
        // so we need 370 bits and 23 hashes
        let mut bloom_filter = BloomFilter::new(370, 23);

        value.iter().for_each(|v| bloom_filter.add(v));

        value.iter().for_each(|v| {
            assert_eq!(bloom_filter.check(v), true);
        })
    }

    #[test]
    fn test_fp_rate_of_bloom_filter() {
        let value: Vec<String> = (0..10).map(|v| format!("{}", v)).collect();

        // Expected FP rate is 1.0E-1 and since we are only storing 11 (0..10) items
        // so we need 53 bits and 3 hashes
        let mut bloom_filter = BloomFilter::new(53, 3);

        value.iter().for_each(|v| bloom_filter.add(v));

        value.iter().for_each(|v| {
            assert_eq!(bloom_filter.check(v), true);
        });

        let mut no_of_false_postive = 0;

        (11..1000).map(|v| format!("{}", v)).for_each(|v| {
            let is_false_present = bloom_filter.check(&v);

            if is_false_present {
                no_of_false_postive += 1;
            }
        });

        // It does following algorithm in floating point
        // fp_rate = ( no_false_positive / (100 - 11 + 1) )
        let fp_rate: f64 = (no_of_false_postive as f64) / (1000f64 - 11f64 + 1f64);

        println!("False postive rate: {}", fp_rate);

        assert!(fp_rate < 0.1)
    }
}
