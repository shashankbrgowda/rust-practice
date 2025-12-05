// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
// Example 1:
// Input: strs = ["eat","tea","tan","ate","nat","bat"]
// Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
// Example 2:
// Input: strs = [""]
// Output: [[""]]
// Example 3:
// Input: strs = ["a"]
// Output: [["a"]]

use std::collections::HashMap;

fn grouped_anagram(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();

    for s in strs {
        let mut c_arr: Vec<char> = s.chars().collect();
        c_arr.sort();
        map.entry(c_arr).or_insert(Vec::new()).push(s);
    }

    map.values().cloned().collect()
}

#[cfg(test)]
mod tests {
    use crate::array::grouped_anagram::grouped_anagram;

    #[test]
    fn test_grouped_anagram() {
        let res = grouped_anagram(vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|s| s.to_string())
            .collect());
        println!("res = {:?}", res);

        let res = grouped_anagram(vec![""]
            .iter()
            .map(|s| s.to_string())
            .collect());
        println!("res = {:?}", res);

        let res = grouped_anagram(vec!["a"]
            .iter()
            .map(|s| s.to_string())
            .collect());
        println!("res = {:?}", res);
    }
}