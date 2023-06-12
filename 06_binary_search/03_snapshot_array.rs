/*
https://leetcode.com/problems/snapshot-array/description/


Implement a SnapshotArray that supports the following interface:

SnapshotArray(int length) initializes an array-like data structure with the given length. Initially, each element equals 0.
void set(index, val) sets the element at the given index to be equal to val.
int snap() takes a snapshot of the array and returns the snap_id: the total number of times we called snap() minus 1.
int get(index, snap_id) returns the value at the given index, at the time we took the snapshot with the given snap_id
 

Example 1:

Input: ["SnapshotArray","set","snap","set","get"]
[[3],[0,5],[],[0,6],[0,0]]
Output: [null,null,0,null,5]
Explanation: 
SnapshotArray snapshotArr = new SnapshotArray(3); // set the length to be 3
snapshotArr.set(0,5);  // Set array[0] = 5
snapshotArr.snap();  // Take a snapshot, return snap_id = 0
snapshotArr.set(0,6);
snapshotArr.get(0,0);  // Get the value of array[0] with snap_id = 0, return 5
 

Constraints:

1 <= length <= 5 * 104
0 <= index < length
0 <= val <= 109
0 <= snap_id < (the total number of times we call snap())
At most 5 * 104 calls will be made to set, snap, and get.
*/


use std::collections::HashMap;

struct SnapshotArray {
    snap_ids_: Vec<Vec<i32>>,
    values_: Vec<Vec<i32>>,
    snap_id_: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {

    fn new(length: i32) -> Self {
        return SnapshotArray {
            snap_ids_: vec![vec![0]; length as usize],
            values_: vec![vec![0]; length as usize],
            snap_id_: 0
        };
    }
    
    fn set(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let last_snap = self.snap_ids_[index].len() - 1;

        if self.snap_ids_[index][last_snap] as usize == self.snap_id_  as usize{
            self.values_[index][last_snap as usize] = val;
        } else {
            self.snap_ids_[index as usize].push(self.snap_id_);
            self.values_[index as usize].push(val);
        }
        
    }
    
    fn snap(&mut self) -> i32 {
        self.snap_id_ += 1;
        return self.snap_id_ - 1;
    }
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let (index, snap_id) = (index as usize, snap_id as i32);
        //println!("{:?} {:?}", self.snap_ids_, self.values_);
        

        match self.snap_ids_[index as usize].binary_search(&snap_id) {
            Err(snap) => self.values_[index][snap - 1],
            Ok(snap) => self.values_[index][snap]
        }
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
