/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
	let mut winning_hands = Vec::new();
	let mut winning_hand_rank = rank_hand(hands[0]);
	winning_hands.push(hands[0]);
	if hands.len() == 1 {
		return winning_hands;
	}
	for hand in hands.iter().skip(1) {
		let this_hand_rank = rank_hand(hand);
		if this_hand_rank > winning_hand_rank {
			winning_hand_rank = this_hand_rank;
			winning_hands.clear();
			winning_hands.push(*hand);
		} else if this_hand_rank == winning_hand_rank {
			winning_hands.push(*hand);
		}
	}
	winning_hands
}

fn rank_hand(hand_str: &str) -> Hands {
	let hand = parse_hand(hand_str);
	let mut rank_counts = [0; 14];
	let mut suit_counts = [0; 4];
	for card in hand.cards.iter() {
		rank_counts[card.rank] += 1;
		suit_counts[card.suit] += 1;
	}
	let mut rank_count_counts = [0; 5];
	for rank_count in rank_counts.iter() {
		rank_count_counts[*rank_count as usize] += 1;
	}
	let is_flush = is_flush(&suit_counts);
	let is_straight = is_straight(rank_counts);
	if is_flush && is_straight {
		parse_straight_flush(hand)
	} else if rank_count_counts[4] == 1 {
		parse_four_of_kind(rank_counts)
	} else if rank_count_counts[3] == 1 && rank_count_counts[2] == 1 {
		parse_full_house(rank_counts)
	} else if is_flush {
		parse_flush(hand)
	} else if is_straight {
		parse_straight(hand)
	} else if rank_count_counts[3] == 1 {
		parse_three_of_kind(rank_counts)
	} else if rank_count_counts[2] == 2 {
		parse_two_pair(rank_counts)
	} else if rank_count_counts[2] == 1 {
		parse_pair(rank_counts)
	} else {
		Hands::HighCard([
			hand.cards[4].rank,
			hand.cards[3].rank,
			hand.cards[2].rank,
			hand.cards[1].rank,
			hand.cards[0].rank,
		])
	}
}

fn parse_straight_flush(hand: Hand) -> Hands {
	// even though an ace is usually high, a 5-high straight flush is the lowest-scoring straight flush
	if hand.cards[4].rank == 13 {
		Hands::StraightFlush(4)
	} else {
		Hands::StraightFlush(hand.cards[4].rank)
	}
}

fn parse_straight(hand: Hand) -> Hands {
	// even though an ace is usually high, a 5-high straight is the lowest-scoring straight
	if hand.cards[4].rank == 13 {
		Hands::Straight(4)
	} else {
		Hands::Straight(hand.cards[4].rank)
	}
}

fn parse_flush(hand: Hand) -> Hands {
	Hands::Flush([
		hand.cards[4].rank,
		hand.cards[3].rank,
		hand.cards[2].rank,
		hand.cards[1].rank,
		hand.cards[0].rank,
	])
}

fn is_straight(rank_counts: [i32; 14]) -> bool {
	//special ace check, then normal check
	let ace_straight = rank_counts[1] == 1
		&& rank_counts[2] == 1
		&& rank_counts[3] == 1
		&& rank_counts[4] == 1
		&& rank_counts[13] == 1;
	if ace_straight {
		return true;
	}
	rank_counts.windows(5).enumerate().any(|(_, window)| {
		window.iter().all(|&count| count == 1)
	})
}

fn is_flush(suit_counts: &[i32; 4]) -> bool {
	suit_counts.iter().any(|&count| count == 5)
}

fn parse_four_of_kind(counts: [i32; 14]) -> Hands {
	let mut rank = 0;
	let mut kicker = 0;
	for (i, &count) in counts.iter().enumerate() {
		if count == 4 {
			rank = i;
		} else if count == 1 {
			kicker = i;
		}
	}
	Hands::FourOfAKind(rank, kicker)
}

fn parse_full_house(counts: [i32; 14]) -> Hands {
	let mut three_of_kind = 0;
	let mut pair = 0;
	for (i, &count) in counts.iter().enumerate() {
		if count == 3 {
			three_of_kind = i;
		} else if count == 2 {
			pair = i;
		}
	}
	Hands::FullHouse(three_of_kind, pair)
}

fn parse_three_of_kind(counts: [i32; 14]) -> Hands {
	let mut rank = 0;
	let mut kickers = [0; 2];
	for (i, &count) in counts.iter().enumerate() {
		if count == 3 {
			rank = i;
		} else if count == 1 {
			if kickers[0] == 0 {
				kickers[0] = i;
			} else {
				kickers[1] = i;
			}
		}
	}
	Hands::ThreeOfAKind(rank, kickers)
}

fn parse_two_pair(counts: [i32; 14]) -> Hands {
	let mut pairs = [0; 2];
	let mut kicker = 0;
	for (i, &count) in counts.iter().enumerate() {
		if count == 2 {
			if pairs[0] == 0 {
				pairs[0] = i;
			} else {
				pairs[1] = i;
			}
		} else if count == 1 {
			kicker = i;
		}
	}
	Hands::TwoPair(pairs[1], pairs[0], kicker)
}

fn parse_pair(counts: [i32; 14]) -> Hands {
	let mut rank = 0;
	let mut kickers = [0; 3];
	for (i, &count) in counts.iter().enumerate() {
		if count == 2 {
			rank = i;
		} else if count == 1 {
			if kickers[0] == 0 {
				kickers[0] = i;
			} else if kickers[1] == 0 {
				kickers[1] = i;
			} else {
				kickers[2] = i;
			}
		}
	}
	Hands::Pair(rank, kickers)
}

// Parse a (sorted) hand from a string slice
fn parse_hand(hand_str: &str) -> Hand {
	let mut cards = [Card::new(0, 0).unwrap(); 5];
	for (i, card_str) in hand_str.split_whitespace().enumerate() {
		cards[i] = Card::from(card_str);
	}
	cards
		.sort_by(|a, b| a.rank.partial_cmp(&b.rank).unwrap());
	Hand { cards }
}

#[derive(PartialEq, PartialOrd)]
enum Hands {
	HighCard([usize; 5]),
	Pair(usize, [usize; 3]),
	TwoPair(usize, usize, usize),
	ThreeOfAKind(usize, [usize; 2]),
	Straight(usize),
	Flush([usize; 5]),
	FullHouse(usize, usize),
	FourOfAKind(usize, usize),
	StraightFlush(usize),
}

#[derive(PartialEq, PartialOrd)]
struct Hand {
	cards: [Card; 5],
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
struct Card {
	rank: usize,
	//0 is empty
	suit: usize,
}

impl Card {
	fn new(rank: usize, suit: usize) -> Option<Self> {
		if rank > 13 || suit > 3 {
			None
		} else {
			Some(Card { rank, suit })
		}
	}
}

impl From<&str> for Card {
	fn from(input: &str) -> Self {
		match input.len() {
			2..=3 => {
				let rank = Card::get_rank(input);
				let suit = Card::get_suit(input);
				Card { rank, suit }
			}
			_ => panic!("Invalid card"),
		}
	}
}

impl Card {
	fn get_rank(input: &str) -> usize {
		let rank = match input.chars().next().unwrap() {
			'2' => 1,
			'3' => 2,
			'4' => 3,
			'5' => 4,
			'6' => 5,
			'7' => 6,
			'8' => 7,
			'9' => 8,
			'1' => 9,
			'J' => 10,
			'Q' => 11,
			'K' => 12,
			'A' => 13,
			_ => panic!("Invalid rank"),
		};
		rank
	}
	fn get_suit(input: &str) -> usize {
		let suit = match input.chars().last().unwrap() {
			'C' => 0,
			'D' => 1,
			'H' => 2,
			'S' => 3,
			_ => panic!("Invalid suit"),
		};
		suit
	}
}
