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

pub fn sublist<T: PartialEq + Clone>(_fl: &[T], _sl: &[T]) -> Comparison {
	if _fl.len() > _sl.len() {
		return sublist(_sl, _fl).reverse();
	}
	let mut result = Comparison::Unequal;

	match (_fl.len(), _sl.len()) {
		(0, 0) => result = Comparison::Equal,
		(0, _) => result = Comparison::Sublist,
		(_, 0) => result = Comparison::Superlist,
		(_, _) => {
			if _fl == _sl {
				result = Comparison::Equal;
			} else {
				// temp vecs
				let mut second = _sl.to_vec();
				let first_len = _fl.len();

				while first_len <= second.len() {
					let second_len = second.len();
					let front_match = _fl[..] == second[..first_len];
					let back_match = _fl[..] == second[second_len - first_len..];
					if front_match || back_match {
						result = Comparison::Sublist;
						break;
					}
					// second.remove(0); works but is slow
					second = second[1..].to_vec();
				}
			}
		}
	}
	result
}

