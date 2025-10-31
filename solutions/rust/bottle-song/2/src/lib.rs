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
    format!(
        "{first_verse}{first_verse}{second_line}{third_line}",
        first_verse = get_first_verse(count),
        second_line = "And if one green bottle should accidentally fall,\n",
        third_line = get_third_line(count - 1)
    )
}

fn get_first_verse(count: usize) -> String {
    format!(
        "{} green {} hanging on the wall,\n",
        capitalize(NUM_WORD_MAP[count]),
        get_bottle_string(count)
    )
}

fn get_third_line(count: usize) -> String {
    format!(
        "There'll be {} green {} hanging on the wall.",
        NUM_WORD_MAP[count],
        get_bottle_string(count)
    )
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
