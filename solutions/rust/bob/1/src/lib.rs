pub fn reply(message: &str) -> &str {
	let mut is_question = false;
	let mut is_yell = false;
	let trimmed_message = message.trim();
	// early return if all whitespace
	if trimmed_message.is_empty() {
		return "Fine. Be that way!";
	} else {
		// check if question
		let mut chars = trimmed_message.chars();
		if chars.next_back().unwrap() == '?' {
			is_question = true;
		}
		// remove non alphabetical and check if all caps (yell)
		while let Some(c) = chars.next_back() {
			if c.is_alphabetic() {
				if c.is_lowercase() {
					is_yell = false;
					break;
				} else {
					is_yell = true;
				}
			}
		}
	}
	if is_question && is_yell {
		"Calm down, I know what I'm doing!"
	} else if is_question {
		"Sure."
	} else if is_yell {
		"Whoa, chill out!"
	} else {
		"Whatever."
	}
}
