// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

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
	fn years_during(d: &Duration) -> f64;
}


pub struct Mercury;

pub struct Venus;

pub struct Earth;

pub struct Mars;

pub struct Jupiter;

pub struct Saturn;

pub struct Uranus;

pub struct Neptune;

enum PlanetType {
	Mercury,
	Venus,
	Earth,
	Mars,
	Jupiter,
	Saturn,
	Uranus,
	Neptune,
}

impl PlanetType {
	fn orbital_period(&self) -> f64 {
		match self {
			PlanetType::Mercury => 0.2408467,
			PlanetType::Venus => 0.61519726,
			PlanetType::Earth => 1.0,
			PlanetType::Mars => 1.8808158,
			PlanetType::Jupiter => 11.862615,
			PlanetType::Saturn => 29.447498,
			PlanetType::Uranus => 84.016846,
			PlanetType::Neptune => 164.79132,
		}
	}
}

#[macro_export]
macro_rules! impl_planet {
	(for $($planet:ident),+) => {
		$(
			impl Planet for $planet {
				fn years_during(d: &Duration) -> f64 {
					let planet_type = PlanetType::$planet;
					let seconds = d.seconds as f64;
					let earth_years = seconds / 31557600.0;
					earth_years / planet_type.orbital_period()
				}
			}
		)+
	};
}

impl_planet!(for Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);
