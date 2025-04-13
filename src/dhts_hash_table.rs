struct DoubleHashingTombstoneHashTable {
    table: Vec<Option<(Option<usize>, Vec<char>)>>,
    capacity: usize,
    size: usize,
}

const TOMBSTONE: Option<(Option<usize>, Vec<char>)> = Some((None, vec![]));

impl DoubleHashingTombstoneHashTable {
    fn new(capacity: usize) -> Self {
        DoubleHashingTombstoneHashTable {
            table: vec![None; capacity],
            capacity,
            size: 0,
        }
    }

    fn get_preferred_index(&self, key: usize) -> usize {
        // For testing simplicity, we just mod the key by the capacity.
        // We would ideally use a proper hash function here.
        key % self.capacity
    }

    fn get_secondary_hash(&self, _key: usize) -> usize {
        // For testing simplicity, we just return 1 as the hash value for everything
        1
    }

    fn get_probe_index(&self, preferred_index: usize, i: usize, key: usize) -> usize {
        if i > 0 {
            (preferred_index + i * self.get_secondary_hash(key)) % self.capacity
        } else {
            preferred_index
        }
    }

    fn resize_to(&mut self, new_capacity: usize) -> bool {
        if new_capacity < self.capacity {
            return false;
        }
        self.capacity = new_capacity;
        let old_table = std::mem::take(&mut self.table);
        self.table = vec![None; self.capacity];
        self.size = 0;
        for entry in old_table {
            if let Some((key, value)) = entry {
                if key.is_some() {
                    self.insert(key.unwrap(), value);
                }
            }
        }
        return true;
    }

    fn resize(&mut self) {
        assert!(self.resize_to(self.capacity * 2));
    }

    fn insert(&mut self, key: usize, value: Vec<char>) {
        // Resize the table because it's full.
        if self.size >= self.capacity {
            self.resize();
        }

        // Try to resize 4 times and try probing again if the inner loop fails.
        for _ in 0..3 {
            let preferred_index = self.get_preferred_index(key);
            let mut tombstone_found = false;
            let mut tombstone_idx: usize = 0;

            // Loop until a free location is found.
            for i in 0..self.capacity {
                // Loop around to the front of the vector as needed.
                let probe_index = self.get_probe_index(preferred_index, i, key);

                if self.table[probe_index].is_none() {
                    if tombstone_found {
                        // We hit an empty bucket, but we encountered a tombstone earlier
                        self.table[tombstone_idx] = Some((Some(key), value));
                    } else {
                        // Empty bucket found and we didn't find a tombstone in the probing earlier
                        // Insert at found empty bucket
                        self.table[probe_index] = Some((Some(key), value));
                    }
                    self.size += 1;
                    return;
                } else if let Some((existing_key, _)) = &self.table[probe_index] {
                    if *existing_key == Some(key) {
                        if tombstone_found {
                            // Key already exists, but we found a tombstone earlier during probing
                            // Move the entry to the earlier tombstone and set the current entry to a tombstone
                            self.table[tombstone_idx] = Some((Some(key), value));
                            self.table[probe_index] = TOMBSTONE;
                        } else {
                            // Key already exists and we didn't find a tombstone during probing, replace the value
                            self.table[probe_index] = Some((Some(key), value));
                        }
                        return;
                    } else if existing_key.is_none() && !tombstone_found {
                        // This is the first tombstone we've found in probing; so record its position

                        tombstone_found = true;
                        tombstone_idx = probe_index;
                    }
                }
            }

            // Could not find a bucket for insertion. Resize and try again.
            self.resize();
        }
        panic!("Could not find a place to insert key {} after resizing 4 times", key);
    }

    fn find_index_for_key_if_exists(&mut self, key: usize) -> Option<usize> {
        let preferred_index = self.get_preferred_index(key);

        for i in 0..self.capacity {
            let probe_index: usize = self.get_probe_index(preferred_index, i, key);

            if let Some((existing_key, _)) = &self.table[probe_index] {
                // We've hit an occupied bucket. Check if the key matches, in which case, we have a hit.
                // If the key doesn't match, continue probing by jumping to the line incrementing i.
                if *existing_key == Some(key) {
                    return Some(probe_index);
                }
            } else {
                // We've hit an empty bucket during probing.
                return None;
            }
        }
        // Table is full, key not found after wrap-around probing
        return None;

    }

    fn get(&mut self, key: usize) -> Option<&Vec<char>> {
        let found_index = self.find_index_for_key_if_exists(key);
        if found_index.is_none() {
            return None;
        }
        if let Some((_, value)) = &self.table[found_index.unwrap()] {
            return Some(value);
        } else {
            return None;
        }
    }

    fn delete(&mut self, key: usize) {
        let found_index = self.find_index_for_key_if_exists(key);
        if found_index.is_none() {
            return;
        }
        let actual_index = found_index.unwrap();
        if let Some((_, _)) = &self.table[actual_index] {
            self.size -= 1;
            self.table[actual_index] = TOMBSTONE;
        }
    }

    fn print(&self) {
        println!("Capacity: {}", self.capacity);
        println!("Size: {}", self.size);
        println!("Entries:");
        for entry in &self.table {
            if let Some((key, value)) = entry {
                if key.is_none() {
                    println!("\t<TOMBSTONE>");
                } else {
                    println!("\t{}: {}", key.unwrap(), value.iter().collect::<String>());
                }
            } else {
                println!("\t<None>");
            }
        }
    }
}

pub fn run() {
    let capacity: usize= 10;
    let mut hash_table = DoubleHashingTombstoneHashTable::new(capacity);

    let one: Vec<char> = "one".chars().collect();
    let two: Vec<char> = "two".chars().collect();
    let three: Vec<char> = "three".chars().collect();
    let four: Vec<char> = "four".chars().collect();
    let five: Vec<char> = "five".chars().collect();
    let six: Vec<char> = "six".chars().collect();
    let seven: Vec<char> = "seven".chars().collect();
    let eight: Vec<char> = "eight".chars().collect();
    let nine: Vec<char> = "nine".chars().collect();
    let ten: Vec<char> = "ten".chars().collect();
    let eleven: Vec<char> = "eleven".chars().collect();
    let twelve: Vec<char> = "twelve".chars().collect();

    hash_table.insert(1, one.clone());
    hash_table.insert(2, two.clone());
    hash_table.insert(3, three.clone());
    hash_table.insert(4, four.clone());
    hash_table.insert(5, five.clone());
    hash_table.insert(6, six.clone());
    hash_table.insert(7, seven.clone());
    hash_table.insert(8, eight.clone());
    // Should collide with 1
    hash_table.insert(11, eleven.clone());
    println!("Initial:");
    hash_table.print();
    assert!(hash_table.capacity == 10);
    assert!(hash_table.size == 9);
    let mut return_val = hash_table.get(1);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(one.clone()).filter(|&(a, b)| *a != b).count() == 0);
    return_val = hash_table.get(2);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(two.clone()).filter(|&(a, b)| *a != b).count() == 0);
    return_val = hash_table.get(3);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(three.clone()).filter(|&(a, b)| *a != b).count() == 0);
    return_val = hash_table.get(4);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(four.clone()).filter(|&(a, b)| *a != b).count() == 0);
    return_val = hash_table.get(5);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(five.clone()).filter(|&(a, b)| *a != b).count() == 0);
    return_val = hash_table.get(6);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(six.clone()).filter(|&(a, b)| *a != b).count() == 0);
    return_val = hash_table.get(7);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(seven.clone()).filter(|&(a, b)| *a != b).count() == 0);
    return_val = hash_table.get(8);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(eight.clone()).filter(|&(a, b)| *a != b).count() == 0);
    return_val = hash_table.get(11);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(eleven.clone()).filter(|&(a, b)| *a != b).count() == 0);

    hash_table.delete(4);
    return_val = hash_table.get(4);
    assert!(return_val.is_none());
    println!("After deleting key 4:");
    hash_table.print();
    assert!(hash_table.size == 8);

    hash_table.insert(11, eleven.clone());
    println!("Re-inserted 11, which should be moved to the earlier tombstone:");
    hash_table.print();
    assert!(hash_table.size == 8);

    hash_table.insert(12, twelve.clone());
    println!("Inserted 12, which should collide with 2:");
    hash_table.print();
    assert!(hash_table.size == 9);

    hash_table.insert(9, nine.clone());
    hash_table.insert(10, ten.clone());
    println!("Inserted 9 & 10, table should be resized to 20:");
    hash_table.print();
    assert!(hash_table.capacity == 20);
    assert!(hash_table.size == 11);
}
