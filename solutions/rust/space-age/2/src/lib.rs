#[derive(Debug)]
pub struct Duration {
	seconds: u64,
}

impl From<u64> for Duration {
	fn from(s: u64) -> Self {
		Duration { seconds: s }
	}
}

pub trait Planet {
	fn orbital_period() -> f64;
	fn years_during(d: &Duration) -> f64;
}

enum Planets {
	Mercury,
	Venus,
	Earth,
	Mars,
	Jupiter,
	Saturn,
	Uranus,
	Neptune,
}


#[macro_export]
macro_rules! impl_planet {
	(for $($p:ident),+) => {
		$(
		  pub struct $p;
			impl Planet for $p {
				fn orbital_period() -> f64 {
					match Planets::$p {
						Planets::Mercury => 0.2408467,
						Planets::Venus => 0.61519726,
						Planets::Earth => 1.0,
						Planets::Mars => 1.8808158,
						Planets::Jupiter => 11.862615,
						Planets::Saturn => 29.447498,
						Planets::Uranus => 84.016846,
						Planets::Neptune => 164.79132,
					}
				}
				fn years_during(d: &Duration) -> f64 {
					d.seconds as f64 / 31557600.0 / Self::orbital_period()
				}
			}
		)+
	};
}

impl_planet!(for Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);
