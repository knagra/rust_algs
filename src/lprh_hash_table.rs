struct LinearProbingRobinHoodHashTable {
    table: Vec<Option<(usize, usize, Vec<char>)>>,
    capacity: usize,
    size: usize,
}

impl LinearProbingRobinHoodHashTable {
    fn new(capacity: usize) -> Self {
        LinearProbingRobinHoodHashTable {
            table: vec![None; capacity],
            capacity,
            size: 0,
        }
    }

    fn get_preferred_index(&self, key: usize) -> usize {
        key % self.capacity
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
            if let Some((key, _, value)) = entry {
                self.insert(key, value);
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

        let mut preferred_index = self.get_preferred_index(key);
        let mut i= 0;
        let mut target_key = key;
        let mut target_value = value.clone();

        // Loop until a free location is found.
        // No need to check if the table is full during probing because the table is scaled as needed
        // at the beginning of this method.
        loop {
            // Loop around to the front of the vector as needed.
            let probe_index = (preferred_index + i) % self.capacity;

            if self.table[probe_index].is_none() {
                // Empty bucket found, insert at found empty bucket
                self.table[probe_index] = Some((target_key, i, target_value));
                self.size += 1;
                return;
            } else if let Some((existing_key, existing_offset, existing_value)) = &self.table[probe_index].clone() {
                if *existing_key == target_key {
                    // Key already exists and we didn't find a tombstone during probing, replace the value
                    self.table[probe_index] = Some((target_key, i, target_value));
                    return;
                } else if *existing_offset < i {
                    // Evict existing bucket occupant because its offset is lower than current item's
                    // Continue evicting subsequent items until an empty bucket is found.
                    self.table[probe_index] = Some((target_key, i, target_value));
                    preferred_index = probe_index - existing_offset;
                    target_key = *existing_key;
                    target_value = existing_value.clone();
                    i = *existing_offset;
                    continue;
                }
            }
            i += 1;
        }
    }

    fn find_index_for_key_if_exists(&mut self, key: usize) -> Option<usize> {
        let preferred_index = self.get_preferred_index(key);

        for i in 0..self.capacity {
            let probe_index: usize = (preferred_index + i) % self.capacity;

            if let Some((existing_key, offset, _)) = &self.table[probe_index] {
                // We've hit an occupied bucket
                if *existing_key == key {
                    // Key matches; we have a hit.
                    return Some(probe_index);
                } else if *offset < i {
                    // Since offset is less than the current occupant's offset, the key is not in the table.
                    // If it were in the table, we would have found it already during probing.
                    return None;
                }
                // Else continue probing
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
        if let Some((_, _, value)) = &self.table[found_index.unwrap()] {
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
        let mut target_index = found_index.unwrap();
        loop {
            let next_index = (target_index + 1) % self.capacity;
            if let Some((next_key, next_offset, next_value)) = &self.table[next_index].clone() {
                if *next_offset > 0 {
                    self.table[target_index] = Some((*next_key, *next_offset - 1, next_value.clone()));
                    target_index = next_index;
                } else {
                    self.table[target_index] = None;
                    self.size -= 1;
                    return;
                }
            } else {
                self.table[target_index] = None;
                self.size -= 1;
                return
            }
        }
    }

    fn print(&self) {
        println!("Capacity: {}", self.capacity);
        println!("Size: {}", self.size);
        println!("Entries:");
        for entry in &self.table {
            if let Some((key, offset, value)) = entry {
                println!("\t{} (offset: {}): {}", key, offset, value.iter().collect::<String>());
            } else {
                println!("\t<None>");
            }
        }
    }
}

pub fn run() {
    let capacity: usize= 10;
    let mut hash_table = LinearProbingRobinHoodHashTable::new(capacity);

    let one: Vec<char> = "one".chars().collect();
    let two: Vec<char> = "two".chars().collect();
    let three: Vec<char> = "three".chars().collect();
    let four: Vec<char> = "four".chars().collect();
    let five: Vec<char> = "five".chars().collect();
    let six: Vec<char> = "six".chars().collect();
    let seven: Vec<char> = "seven".chars().collect();
    let eight: Vec<char> = "eight".chars().collect();
    let eleven: Vec<char> = "eleven".chars().collect();

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
    assert!(hash_table.capacity == 10, "Capacity: {}", hash_table.capacity);
    assert!(hash_table.size == 9, "Size: {}", hash_table.size);
    let return_val = hash_table.get(1);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(one.clone()).filter(|&(a, b)| *a != b).count() == 0);
    let return_val = hash_table.get(2);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(two.clone()).filter(|&(a, b)| *a != b).count() == 0);
    let return_val = hash_table.get(3);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(three.clone()).filter(|&(a, b)| *a != b).count() == 0);
    let return_val = hash_table.get(4);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(four.clone()).filter(|&(a, b)| *a != b).count() == 0);
    let return_val = hash_table.get(5);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(five.clone()).filter(|&(a, b)| *a != b).count() == 0);
    let return_val = hash_table.get(6);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(six.clone()).filter(|&(a, b)| *a != b).count() == 0);
    let return_val = hash_table.get(7);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(seven.clone()).filter(|&(a, b)| *a != b).count() == 0);
    let return_val = hash_table.get(8);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(eight.clone()).filter(|&(a, b)| *a != b).count() == 0);
    let return_val = hash_table.get(11);
    assert!(return_val.is_some());
    assert!(return_val.unwrap().iter().zip(eleven.clone()).filter(|&(a, b)| *a != b).count() == 0);

    hash_table.delete(4);
    let return_val = hash_table.get(4);
    assert!(return_val.is_none());
    println!("After deleting key 4, keys 5 to 8 should be moved toward the front:");
    hash_table.print();
    assert!(hash_table.size == 8, "Size: {}", hash_table.size);

    hash_table.insert(11, eleven.clone());
    println!("Re-inserted 11, which should have no impact:");
    hash_table.print();
    assert!(hash_table.size == 8, "Size: {}", hash_table.size);

    hash_table.insert(4, four.clone());
    println!("Re-inserted 4, keys 5 to 8 should be moved toward the back:");
    hash_table.print();
    assert!(hash_table.size == 9, "Size: {}", hash_table.size);

    let twelve: Vec<char> = "twelve".chars().collect();
    hash_table.insert(12, twelve.clone());
    println!("Inserted 12:");
    hash_table.print();
    assert!(hash_table.size == 10);

    hash_table.delete(12);
    println!("Deleted 12, keys 3 to 8 should be moved toward the front:");
    hash_table.print();
    assert!(hash_table.size == 9, "Size: {}", hash_table.size);

    let nine: Vec<char> = "nine".chars().collect();
    let ten: Vec<char> = "ten".chars().collect();
    hash_table.insert(9, nine.clone());
    hash_table.insert(10, ten.clone());
    hash_table.insert(12, twelve.clone());
    println!("Inserted 9, 10 & 12, table should be resized to 20:");
    hash_table.print();
    assert!(hash_table.capacity == 20);
    assert!(hash_table.size == 12);
}
