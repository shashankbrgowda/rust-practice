use std::collections::HashSet;
use std::cmp;

fn longest_consecutive_subsequence(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut set: HashSet<i32> = HashSet::new();
    for num in nums.iter() {
        set.insert(*num);
    }

    let mut seq_len: i32 = i32::MIN;
    let mut max_seq_len: i32 = i32::MIN;

    for num in nums.iter() {
        if !set.contains(&(num - 1)) {
            let mut val: i32 = *num;
            loop {
                if set.contains(&(val + 1)) {
                    val = val + 1;
                } else {
                    break;
                }
            }
            seq_len = val - *num + 1;
        }
        seq_len = cmp::max(seq_len, 1);
        max_seq_len = cmp::max(max_seq_len, seq_len);
    }

    return max_seq_len;
}

#[cfg(test)]
mod tests {
    use crate::array::longest_consecutive_subsequence::longest_consecutive_subsequence;

    #[test]
    fn test_longest_consecutive_subsequence() {
        let result: i32 = longest_consecutive_subsequence(vec![100,4,200,1,3,2]);
        assert_eq!(result, 4);
    }
}