use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        if s.len() as i32 == k {
            return false;
        }

        let mut h: HashSet<&str> = HashSet::new();

        for i in 0..s.len() {
            if (i + k as usize) <= s.len() {
                let curr = &s[i..i + k as usize];
                h.insert(curr);
            }
        }

        if h.len() == 1 << k {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic() {
        assert!(Solution::has_all_codes("00110110".to_string(), 2));
        assert!(Solution::has_all_codes("0110".to_string(), 1));
        assert!(!Solution::has_all_codes("0110".to_string(), 2));
    }

    #[test]
    fn fails() {
        assert!(Solution::has_all_codes("00110".to_string(), 2));
    }
}
