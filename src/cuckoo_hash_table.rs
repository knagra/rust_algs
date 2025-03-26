struct LinearProbingRobinHoodHashTable {
    table: Vec<Option<(usize, Vec<char>)>>,
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

    fn get_primary_index(&self, key: usize) -> usize {
        key % self.capacity
    }

    fn get_secondary_index(&self, key: usize) -> usize {
        (self.get_primary_index(key) + 2) % self.capacity
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

        let mut primary_index = self.get_primary_index(key);
        let mut secondary_index = self.get_secondary_index(key);

        if let Some((existing_primary_key, _)) = self.table[primary_index] {
            if existing_primary_key == key {
                self.table[primary_index] = Some((key, value));
            } else if self.table[secondary_index].is_none() {
                self.table[secondary_index] = Some((key, value));
                self.size += 1;
            } else {
                let existing_secondary_key = self.table[secondary_index].as_ref().unwrap().0.clone();
                let existing_secondary_value = self.table[secondary_index].as_ref().unwrap().1.clone();
                if existing_secondary_key == key {
                    self.table[secondary_index] = Some((key, value));
                } else {
                    // Start displacement chain, beginning with the secondary index
                    self.table[secondary_index] = Some((key, value));
                    self.size += 1;
                    let mut excluded_index = secondary_index;
                    let mut prev_key = existing_secondary_key;
                    let mut prev_val = existing_secondary_value.clone();
                    primary_index = self.get_primary_index(prev_key);
                    secondary_index = self.get_secondary_index(prev_key);
                    let mut i: usize = 0;
                    let mut target_index = {
                        if excluded_index == primary_index {
                            secondary_index
                        } else {
                            primary_index
                        }
                    };
                    loop {
                        if self.table[target_index].is_none() {
                            self.table[target_index] = Some((prev_key, prev_val));
                            return;
                        }
                        let existing_key = self.table[target_index].as_ref().unwrap().0.clone();
                        let existing_value = self.table[target_index].as_ref().unwrap().1.clone();
                        self.table[target_index] = Some((prev_key, prev_val));
                        prev_key = existing_key.clone();
                        prev_val = existing_value.clone();
                        excluded_index = target_index;

                        i += 1;
                        if i >= self.capacity {
                            self.resize();
                            // The resize function resets the size; so we have to increment it again.
                            self.size += 1;
                            i = 0;
                        }

                        primary_index = self.get_primary_index(prev_key);
                        secondary_index = self.get_secondary_index(prev_key);
                        if excluded_index == primary_index {
                            target_index = secondary_index;
                        } else {
                            target_index = primary_index;
                        }
                    }
                }
            }
        } else {
            self.table[primary_index] = Some((key, value));
            self.size += 1;
        }
    }

    fn find_index_for_key_if_exists(&mut self, key: usize) -> Option<usize> {
        let primary_index = self.get_primary_index(key);
        if let Some((existing_primary_key, _)) = &self.table[primary_index] {
            if *existing_primary_key == key {
                return Some(primary_index);
            }
        }

        let secondary_index = self.get_secondary_index(key);
        if let Some((existing_secondary_key, _)) = &self.table[secondary_index] {
            if *existing_secondary_key == key {
                return Some(secondary_index);
            }
        }

        return None;
    }

    fn get(&mut self, key: usize) -> Option<&Vec<char>> {
        let found_index = self.find_index_for_key_if_exists(key);
        if found_index.is_none() {
            return None;
        }
        if let Some((_key, value)) = &self.table[found_index.unwrap()] {
            return Some(value);
        } else {
            return None;
        }
    }

    fn delete(&mut self, key: usize) {
        let found_index = self.find_index_for_key_if_exists(key);
        if found_index.is_some() {
            self.table[found_index.unwrap()] = None;
            self.size -= 1;
        }
    }

    fn print(&self) {
        println!("Capacity: {}", self.capacity);
        println!("Size: {}", self.size);
        println!("Entries:");
        for entry in &self.table {
            if let Some((key, value)) = entry {
                println!("\t{}: {}", key, value.iter().collect::<String>());
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
    let nine: Vec<char> = "nine".chars().collect();
    let ten: Vec<char> = "ten".chars().collect();
    let eleven: Vec<char> = "eleven".chars().collect();
    let twelve: Vec<char> = "twelve".chars().collect();
    let fifteen: Vec<char> = "fifteen".chars().collect();
    let sixteen: Vec<char> = "sixteen".chars().collect();
    let thirty_five: Vec<char> = "thirty_five".chars().collect();
    let thirty_six: Vec<char> = "thirty_six".chars().collect();
    let fifty_six: Vec<char> = "fifty_six".chars().collect();

    hash_table.insert(1, one.clone());
    hash_table.insert(2, two.clone());
    hash_table.insert(3, three.clone());
    hash_table.insert(4, four.clone());
    hash_table.insert(5, five.clone());
    hash_table.insert(6, six.clone());
    hash_table.insert(7, seven.clone());
    hash_table.insert(8, eight.clone());
    // Should collide with 1 and shift everything after to the back
    hash_table.insert(11, eleven.clone());
    println!("Initial:");
    hash_table.print();
    assert!(hash_table.capacity == 10, "Capacity {} expected 10", hash_table.capacity);
    assert!(hash_table.size == 9, "Size {}, expected 9", hash_table.size);
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
    println!("After deleting key 4, which should have no impact on other keys:");
    hash_table.print();
    assert!(hash_table.size == 8, "Size {}, expected 8", hash_table.size);

    hash_table.insert(11, eleven.clone());
    println!("Re-inserted 11, which should have no impact:");
    hash_table.print();
    assert!(hash_table.size == 8, "Size {}, expected 8", hash_table.size);

    hash_table.insert(4, four.clone());
    println!("Re-inserted 4, which should have no impact on other keys:");
    hash_table.print();
    assert!(hash_table.size == 9, "Size {}, expected 9", hash_table.size);

    hash_table.insert(12, twelve.clone());
    println!("Inserted 12:");
    hash_table.print();
    assert!(hash_table.size == 10, "Size {}, expected 10", hash_table.size);

    hash_table.delete(12);
    println!("Deleted 12, no impact on other keys:");
    hash_table.print();
    assert!(hash_table.size == 9, "Size {}, expected 9", hash_table.size);
    assert!(hash_table.get(12).is_none());

    hash_table.insert(9, nine.clone());
    hash_table.insert(10, ten.clone());
    hash_table.insert(12, twelve.clone());
    hash_table.insert(16, sixteen.clone());
    hash_table.insert(36, thirty_six.clone());
    hash_table.insert(56, fifty_six.clone());
    println!("Inserted 9, 10, 12, 16, 36, 56, table should be resized to 40:");
    hash_table.print();
    assert!(hash_table.capacity == 40, "Capacity {} expected 40", hash_table.capacity);
    assert!(hash_table.size == 15, "Size {}, expected 15", hash_table.size);

    hash_table.insert(15, fifteen.clone());
    hash_table.insert(35, thirty_five.clone());
    println!("Inserted 15 & 35, keys 16, 36 & 56 should be moved toward the back:");
    hash_table.print();
    assert!(hash_table.capacity == 40, "Capacity {} expected 40", hash_table.capacity);
    assert!(hash_table.size == 17, "Size {}, expected 17", hash_table.size);
    assert!(hash_table.get(15).is_some());
    assert!(hash_table.get(35).is_some());

    hash_table.delete(36);
    println!("Deleted 36, keys 16 & 56 should be moved toward the front:");
    hash_table.print();
    assert!(hash_table.capacity == 40, "Capacity {} expected 40", hash_table.capacity);
    assert!(hash_table.size == 16, "Size {}, expected 16", hash_table.size);
    assert!(hash_table.get(35).is_some());
    assert!(hash_table.get(56).is_some());
    assert!(hash_table.get(16).is_some());
    assert!(hash_table.get(36).is_none());

    hash_table.delete(35);
    println!("Deleted 35, keys 16 & 56 should be moved toward the front:");
    hash_table.print();
    assert!(hash_table.capacity == 40, "Capacity {} expected 40", hash_table.capacity);
    assert!(hash_table.size == 15, "Size {}, expected 15", hash_table.size);
    assert!(hash_table.get(36).is_none());
    assert!(hash_table.get(56).is_some());
    assert!(hash_table.get(16).is_some());
}
