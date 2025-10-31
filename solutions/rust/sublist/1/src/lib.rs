#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
	Equal,
	Sublist,
	Superlist,
	Unequal,
}

impl Comparison {
	fn reverse(&self) -> Comparison {
		match self {
			Comparison::Equal => Comparison::Equal,
			Comparison::Sublist => Comparison::Superlist,
			Comparison::Superlist => Comparison::Sublist,
			Comparison::Unequal => Comparison::Unequal,
		}
	}
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
	// first list is a sublist of second list if by dropping 0 or more
	// elements from the front of the second list
	// and 0 or more elements from the back of the second list
	// you get a list that's completely equal to the first list.

	if _first_list.len() > _second_list.len() {
		return sublist(_second_list, _first_list).reverse();
	}
	let mut result = Comparison::Unequal;

	match (_first_list.len(), _second_list.len()) {
		(0, 0) => result = Comparison::Equal,
		(0, _) => result = Comparison::Sublist,
		(_, 0) => result = Comparison::Superlist,
		(_, _) => {
			if _first_list == _second_list {
				result = Comparison::Equal;
			} else {
				let mut first = _first_list;
				let mut second = _second_list;

				while first.len() <= second.len() {
					let is_front_sublist = first[..] == second[..first.len()];
					let is_back_sublist = first[..] == second[second.len() - first.len()..];
					if is_front_sublist || is_back_sublist {
						result = Comparison::Sublist;
						break;
					}
					second = &second[1..];
				}
			}
		}
	}
	result
}

