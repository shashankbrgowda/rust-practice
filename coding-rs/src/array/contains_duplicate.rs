use std::collections::HashSet;

fn contains_duplicate(nums: Vec<i32>) {
    let mut set = HashSet::new();

    for num in nums.iter() {
        if set.contains(num) {
            println!("Contains duplicate");
        } else {
            set.insert(num);
        }
    }

    println!("Doesn't contain duplicate");
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::array::contains_duplicate::contains_duplicate;

    #[test]
    fn test_contains_duplicate() {
        // assert_eq!(contains_duplicate(vec![1,2,3,1]), true);
        // assert_eq!(contains_duplicate(vec![1,2,3]), false);
        contains_duplicate(vec![1,2,3,1]);
        contains_duplicate(vec![1,2,3]);

    }
}