pub struct MyHashMap {
    array: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    pub fn new() -> Self {
        Self { array: vec![-1; 1000001] }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.array[key as usize] = value;
    }

    pub fn get(&self, key: i32) -> i32 {
        self.array[key as usize]
    }

    pub fn remove(&mut self, key: i32) {
        self.array[key as usize] = -1;
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
        let map = MyHashMap::new();
        assert_eq!(map.get(0), -1);
        assert_eq!(map.get(1000000), -1);
    }

    #[test]
    fn should_put_element() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.put(1000000, 1000000);
        assert_eq!(map.get(0), -1);
        assert_eq!(map.get(1), 1);
        assert_eq!(map.get(999999), -1);
        assert_eq!(map.get(1000000), 1000000);
    }

    #[test]
    fn should_remove_element() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.put(3, 3);
        map.put(4, 4);
        map.put(1000000, 1000000);
        map.remove(1);
        map.remove(4);
        map.remove(1000000);
        assert_eq!(map.get(0), -1);
        assert_eq!(map.get(1), -1);
        assert_eq!(map.get(3), 3);
        assert_eq!(map.get(4), -1);
        assert_eq!(map.get(999999), -1);
        assert_eq!(map.get(1000000), -1);
    }
}