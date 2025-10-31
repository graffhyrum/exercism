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
				let mut idx = 0;
				while let Some(slice) = _sl.get(idx..idx + _fl.len()) {
					if slice == _fl {
						result = Comparison::Sublist;
						break;
					}
					idx += 1;
				}
			}
		}
	}
	result
}

