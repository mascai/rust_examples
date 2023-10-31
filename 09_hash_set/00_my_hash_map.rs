/*
https://leetcode.com/problems/design-hashmap/description/
*/


struct MyHashMap {
    m_data: Vec<Vec<(i32, i32)>>,
    m_size: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        let n: usize = 1000;
       return  MyHashMap{
           m_data: vec![vec![]; n],
           m_size: n
        }
    }

    fn hash(n: i32) -> usize {
        return n as usize % 1000;
    }

    fn find_key(&mut self, key: i32) -> Option<(usize, usize)> {
        let hash_key = Self::hash(key);
        let bucket = &mut self.m_data[hash_key];
        if bucket.len() > 0 {
            for i in 0..bucket.len() {
                if bucket[i].0 == key {
                    return Some((hash_key, i));
                }
            }
        }
        return None;
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some((i, j)) = self.find_key(key) {
            self.m_data[i][j].1 = value;
        } else {
            self.m_data[Self::hash(key)].push((key, value));
        }
        
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some((i, j)) = self.find_key(key) {
            return self.m_data[i][j].1;
        }
        return -1;
    }
    
    fn remove(&mut self, key: i32) {
        if let Some((i, j)) = self.find_key(key) {
            self.m_data[i].remove(j);
        }
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */
