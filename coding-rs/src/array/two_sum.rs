// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//
// Example 2:
// Input: nums = [3,2,4], target = 6
// Output: [1,2]
//
// Example 3:
// Input: nums = [3,3], target = 6
// Output: [0,1]

use std::collections::HashMap;

fn two_sum(arr: Vec<i32>, target: i32) {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, num) in arr.iter().enumerate() {
        if map.contains_key(&(target - num)) {
            println!("Index of two numbers whose sum is {0} are {1} and {2}", target,
                     map.get(&(target - num)).unwrap(), i);
        } else {
            map.insert(*num, i as i32);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_two_sum() {
        two_sum(vec![2,7,11,15], 9);
    }
}