use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut word_chars: Vec<char> = word_lower.chars().collect();
    word_chars.sort();

    let mut anagrams = HashSet::new();

    for &candidate in possible_anagrams {
        let candidate_lower = candidate.to_lowercase();

        // Skip if it's the same word (case-insensitive)
        if word_lower == candidate_lower {
            continue;
        }

        // Check if it's an anagram by sorting characters
        let mut candidate_chars: Vec<char> = candidate_lower.chars().collect();
        candidate_chars.sort();

        if word_chars == candidate_chars {
            anagrams.insert(candidate);
        }
    }

    anagrams
}
