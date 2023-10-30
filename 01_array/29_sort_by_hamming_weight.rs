/*
https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/solutions/4224341/89-93-sort-bit/?envType=daily-question&envId=2023-10-30




*/

// Solution 1

  impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by_key(|&x| (x.count_ones(), x));
        arr
    }
}

// Solution 2
  
impl Solution {
    pub fn weight(num: &i32) -> i32 {
        let mut res = 0;
        let mut num = *num;
        while num != 0 {
            if num & 1 == 1 {
                res += 1;
            }
            num >>= 1;
        }
        return res;
    }

    pub fn cmp(l: &i32, r: &i32) -> std::cmp::Ordering {
        let cntl = Self::weight(l);
        let cntr = Self::weight(r);

        if cntl == cntr {
            return l.cmp(&r);
        }
        return cntl.cmp(&cntr);
    }

    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(|l, r| Self::cmp(l, r));
        return arr;
    }
}
