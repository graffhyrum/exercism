pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c == ' ' || c == '-')
      // trim to non-empty
        .filter(|word| !word.is_empty())
      // trim to alphabetical
        .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()))
      // take first char, uppercase
        .map(|word| word.chars().next().unwrap().to_ascii_uppercase())
      // collect into string
        .collect::<String>()
}
