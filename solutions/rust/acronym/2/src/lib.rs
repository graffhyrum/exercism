pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c == ' ' || c == '-')
      .filter_map(|word| {
          word.chars()
            // Find the first character that is alphabetic
              .find(|c| c.is_alphabetic())
            // If there is one, return it as uppercase
              .map(|c| c.to_ascii_uppercase())
      })
      .collect()
}
