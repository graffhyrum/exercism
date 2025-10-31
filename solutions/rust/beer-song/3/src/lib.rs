pub fn sing(start: u32, end: u32) -> String {
	(end..start + 1)
		.rev()
		.map(|n| verse(n))
		.collect::<Vec<_>>()
		.join("\n")
}

pub fn verse(n: u32) -> String {
	format!("{}, {}.\n{}, {}.\n",
	        get_on_the_wall_string(n),
	        get_of_beer_string(n).to_lowercase(),
	        get_third_verse(n),
	        get_on_the_wall_string(if n == 0 { 99 } else { n - 1 }).to_lowercase()
	)
}

fn get_on_the_wall_string(n: u32) -> String {
	format!("{} on the wall", get_of_beer_string(n))
}

fn get_of_beer_string(n: u32) -> String {
	match n {
		0 => "No more bottles of beer".to_string(),
		1 => "1 bottle of beer".to_string(),
		_ => format!("{} bottles of beer", n)
	}
}

fn get_third_verse(n: u32) -> String {
	match n {
		0 => "Go to the store and buy some more".to_string(),
		1 => "Take it down and pass it around".to_string(),
		_ => "Take one down and pass it around".to_string()
	}
}


