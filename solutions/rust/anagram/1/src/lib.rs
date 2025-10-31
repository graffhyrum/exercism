use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  possible_anagrams.iter()
    .fold(HashSet::new(), |mut acc, &x| {
      if is_anagram(word, x) {
        acc.insert(x);
      }
      acc
    })
}

fn is_anagram(word: &str, possible_anagram: &str) -> bool {
  let word = word.to_lowercase();
  let possible_anagram = possible_anagram.to_lowercase();
  if word == possible_anagram {
    return false;
  }
  let mut word_chars: Vec<char> = word.chars().collect();
  let mut possible_anagram_chars: Vec<char> = possible_anagram.chars().collect();
  word_chars.sort_unstable();
  possible_anagram_chars.sort_unstable();
  word_chars == possible_anagram_chars
}