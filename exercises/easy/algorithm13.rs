/*
    Anagram Check
    Given two strings, check if they are anagrams of each other. 
    Anagrams are words or phrases formed by rearranging the letters of another, 
    using all the original letters exactly once. 
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::{collections::HashMap, fmt::{self, Display, Formatter}};

pub fn are_anagrams(s1: String, s2: String) -> bool {
    // TODO: Implement the logic to check if two strings are anagrams
    let normalize = |s: String| {
        s.chars()
            .filter(|ch| ch.is_alphabetic())
            .map(|ch| ch.to_ascii_lowercase())
            .collect::<String>()
    };

    let normalized_s1 = normalize(s1);
    let normalized_s2 = normalize(s2);
    
    if normalized_s1.len() != normalized_s2.len() {
        return false;
    }

    let mut table = HashMap::new();
    for ch in normalized_s1.chars() {
        *table.entry(ch).or_insert(0) += 1;
    }

    for ch in normalized_s2.chars() {
        let count = table.entry(ch).or_insert(0);
        if *count == 0 {
            return false;
        }
        *count -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}
