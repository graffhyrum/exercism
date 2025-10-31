use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};
use std::thread;

//Count the frequency of letters in texts using parallel computation.
pub fn frequency(input: &[&str], thread_count: usize) -> HashMap<char, usize> {
	let mut hm = HashMap::new();
	if input.is_empty() {
		return hm;
	}
	let (tx, rx) = channel();

	thread::scope(|s| {
		for i in 0..thread_count {
			// get every nth line from input
			let this_workers_lines = input
				.iter()
				.enumerate()
				.filter(move |(j, _)| j % thread_count == i)
				.map(|(_, line)| line);

			// clone the transmitter for this worker
			let tx = Sender::clone(&tx);

			// spawn a thread for this worker
			s.spawn(move || {
				let mut hm = HashMap::new();
				for line in this_workers_lines {
					for (c, count) in count_letters(line) {
						*hm.entry(c).or_insert(0) += count;
					}
				}
				tx.send(hm).unwrap();
			});
		}
	});


	// drop the transmitter so the receiver knows when to stop
	drop(tx);

	// collect the results from the workers
	for worker_hm in rx {
		for (c, count) in worker_hm {
			*hm.entry(c).or_insert(0) += count;
		}
	}
	hm
}


// function to count the frequency of letters in a single line, case insensitive
fn count_letters(line: &str) -> HashMap<char, usize> {
	let mut hm = HashMap::new();
	for c in line.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
		*hm.entry(c).or_insert(0) += 1;
	}
	hm
}