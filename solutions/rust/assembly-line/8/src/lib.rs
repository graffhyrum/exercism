
pub fn production_rate_per_hour(speed: u8) -> f64 {
  let base_rate = 221.0;
  let gross_production = speed as f64 * base_rate;
  match speed {
    1..=4 => gross_production,
    5..=8 => gross_production * 0.9,
    9 | 10 => gross_production * 0.77,
    _ => panic!()
  }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
  (production_rate_per_hour(speed) / 60.0) as u32
}
