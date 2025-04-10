use bitvec;

struct BloomFilter {
    filter: bitvec::vec::BitVec,
    capacity: usize,
}

impl BloomFilter {
    fn new(capacity: usize) -> Self {
        BloomFilter {
            filter: bitvec::vec::BitVec::repeat(false, capacity),
            capacity
        }
    }

    fn get_hashes(&self, item: usize) -> Vec<usize> {
        return vec![item % self.capacity, (item + 10) % self.capacity];
    }

    fn insert(&mut self, item: usize) {
        for hash in self.get_hashes(item) {
            self.filter.set(hash, true);
        }
    }

    fn contains(&self, item: usize) -> bool {
        for hash in self.get_hashes(item) {
            let bitval_opt = self.filter.get(hash);
            if bitval_opt.is_none() || bitval_opt.unwrap() == false {
                return false;
            }
        }
        return true;
    }
}

pub fn run() {
    let mut bloom_filter = BloomFilter::new(100);
    bloom_filter.insert(1);
    bloom_filter.insert(2);
    bloom_filter.insert(3);
    bloom_filter.insert(4);
    bloom_filter.insert(5);
    bloom_filter.insert(6);
    bloom_filter.insert(15);
    bloom_filter.insert(16);
    bloom_filter.insert(35);
    bloom_filter.insert(36);
    assert!(bloom_filter.contains(1));
    assert!(bloom_filter.contains(2));
    assert!(bloom_filter.contains(3));
    assert!(bloom_filter.contains(4));
    assert!(bloom_filter.contains(5));
    assert!(bloom_filter.contains(6));
    assert!(bloom_filter.contains(15));
    assert!(bloom_filter.contains(16));
    assert!(bloom_filter.contains(35));
    assert!(bloom_filter.contains(36));
    // False positives are expected
    assert!(bloom_filter.contains(25));
    assert!(bloom_filter.contains(26));
    println!("All tests passed");
}
