pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .as_bytes()
        .windows(len)
        .map(|win| String::from_utf8(win.to_vec()).expect("Invalid UTF-8"))
        .collect()
}
