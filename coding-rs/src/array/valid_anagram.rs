use std::collections::HashMap;

/// Check If two strings are valid anagram
fn valid_anagram(s1: String, s2: String) {
    if s1.len() != s2.len() {
        println!("{0} and {1} are not anagram", s1, s2);
        return;
    }

    let mut map: HashMap<char, i32> = HashMap::new();

    for c in s1.chars() {
        /*
        entry().or_insert() is used to insert if the value is not present, if present returns reference to the value
         */
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    for c in s2.chars() {
        if map.contains_key(&c) && map.get(&c).unwrap() > &0 {
            map.insert(c, map.get(&c).unwrap() - 1);
        } else {
            println!("{0} and {1} are not anagram", s1, s2);
            return;
        }
    }

    println!("{0} and {1} are anagram", s1, s2);
}

#[cfg(test)]
mod tests {
    use super::valid_anagram;

    #[test]
    fn test_valid_anagram() {
        valid_anagram(String::from("anagram"), String::from("nagaram"));
        valid_anagram(String::from("cat"), String::from("rat"));
    }
}