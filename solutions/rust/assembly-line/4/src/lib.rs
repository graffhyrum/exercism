// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
  let base_rate = 221;
  match speed {
    1..=4 => f64::from(speed * base_rate),
    5..=8 => speed * base_rate * 0.9,
    9 | 10 => speed * base_rate * 0.77,
    _ => panic!()
  }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
  u32::from((production_rate_per_hour(speed) / 60.0).round())
}
