pub struct MyHashSet {
    array: Vec<bool>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    pub fn new() -> Self {
        Self { array: vec![false; 1000001] }
    }

    pub fn add(&mut self, key: i32) {
        self.array[key as usize] = true;
    }

    pub fn contains(&self, key: i32) -> bool {
        self.array[key as usize]
    }

    pub fn remove(&mut self, key: i32) {
        self.array[key as usize] = false;
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_map() {
        let map = MyHashSet::new();
        assert_eq!(map.contains(0), false);
        assert_eq!(map.contains(1000000), false);
    }

    #[test]
    fn should_put_element() {
        let mut map = MyHashSet::new();
        map.add(1);
        map.add(1000000);
        assert_eq!(map.contains(0), false);
        assert_eq!(map.contains(1), true);
        assert_eq!(map.contains(999999), false);
        assert_eq!(map.contains(1000000), true);
    }

    #[test]
    fn should_remove_element() {
        let mut map = MyHashSet::new();
        map.add(1);
        map.add(3);
        map.add(4);
        map.add(1000000);
        map.remove(1);
        map.remove(4);
        map.remove(1000000);
        assert_eq!(map.contains(0), false);
        assert_eq!(map.contains(1), false);
        assert_eq!(map.contains(3), true);
        assert_eq!(map.contains(4), false);
        assert_eq!(map.contains(999999), false);
        assert_eq!(map.contains(1000000), false);
    }
}