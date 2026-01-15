pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars().filter(|c| "[]{}()".contains(*c)) {
        match c {
            '[' | '{' | '(' => stack.push(c),
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            },
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            },
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            },
            _ => {},
        }
    }
    stack.is_empty()
}
