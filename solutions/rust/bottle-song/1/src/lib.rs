pub fn recite(start_bottles: u32, take_down: u32) -> String {
    // todo!("Return the bottle song starting at {start_bottles} and taking down {take_down} bottles")
    let mut round = start_bottles;
    let mut rounds_left = take_down;
    let mut song = String::new();
    loop {
        if rounds_left == 0 {
            break;
        }
        song += &get_stanza(round as usize);
        rounds_left -= 1;
        round -= 1;
        if rounds_left > 0 {
            song += "\n\n";
        }
    }
    song
}

const NUM_WORD_MAP: &[&str] = &[
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

fn get_stanza(count: usize) -> String {
    let mut res = String::new();
    res += &get_first_verse(count);
    res += &get_first_verse(count);
    res += &get_second_line();
    res += &get_third_line( count - 1 );
    res
}

fn get_first_verse(count: usize) -> String {
    capitalize(NUM_WORD_MAP[count])
        + " green "
        + get_bottle_string(count)
        + " hanging on the wall,\n"
}

fn get_second_line() -> String {
    "And if one green bottle should accidentally fall,\n".into()
}

fn get_third_line(count: usize) -> String {
    "There'll be ".to_string()
        + NUM_WORD_MAP[count]
        + " green "
        + get_bottle_string(count)
        + " hanging on the wall."
}

fn get_bottle_string(count: usize) -> &'static str {
    match count {
        1 => "bottle",
        _ => "bottles",
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
