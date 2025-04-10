struct CountingBloomFilter {
    filter: Vec<usize>,
    capacity: usize,
}

impl CountingBloomFilter {
    fn new(capacity: usize) -> Self {
        CountingBloomFilter {
            filter: vec![0; capacity],
            capacity
        }
    }

    fn get_hashes(&self, item: usize) -> Vec<usize> {
        return vec![item % self.capacity, (item + 10) % self.capacity];
    }

    fn insert(&mut self, item: usize) {
        for hash in self.get_hashes(item) {
            self.filter[hash] += 1;
        }
    }

    fn remove(&mut self, item: usize) {
        for hash in self.get_hashes(item) {
            self.filter[hash] -= 1;
        }
    }

    fn contains(&self, item: usize) -> bool {
        for hash in self.get_hashes(item) {
            if self.filter[hash] == 0 {
                return false;
            }
        }
        return true;
    }
}

pub fn run() {
    let mut counting_bloom_filter = CountingBloomFilter::new(100);
    counting_bloom_filter.insert(1);
    counting_bloom_filter.insert(2);
    counting_bloom_filter.insert(3);
    counting_bloom_filter.insert(4);
    counting_bloom_filter.insert(5);
    counting_bloom_filter.insert(6);
    counting_bloom_filter.insert(15);
    counting_bloom_filter.insert(16);
    counting_bloom_filter.insert(35);
    counting_bloom_filter.insert(36);
    assert!(counting_bloom_filter.contains(1));
    assert!(counting_bloom_filter.contains(2));
    assert!(counting_bloom_filter.contains(3));
    assert!(counting_bloom_filter.contains(4));
    assert!(counting_bloom_filter.contains(5));
    assert!(counting_bloom_filter.contains(6));
    assert!(counting_bloom_filter.contains(15));
    assert!(counting_bloom_filter.contains(16));
    assert!(counting_bloom_filter.contains(35));
    assert!(counting_bloom_filter.contains(36));
    // False positives are expected
    assert!(counting_bloom_filter.contains(25));
    assert!(counting_bloom_filter.contains(26));

    counting_bloom_filter.remove(35);
    counting_bloom_filter.remove(36);
    assert!(!counting_bloom_filter.contains(35));
    assert!(!counting_bloom_filter.contains(36));
    // False positives should be gone with the removal of 35 and 36
    assert!(!counting_bloom_filter.contains(25));
    assert!(!counting_bloom_filter.contains(26));
    println!("All tests passed");
}
