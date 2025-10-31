use std::fmt::{self, Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Clock {
  hours: i32,
  minutes: i32,
}

impl Display for Clock {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:02}:{:02}", self.hours, self.minutes)
  }
}

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {
    let mut clock_hours = 0;
    let mut clock_minutes = 0;

    match minutes {
      m if m >= 0 && m < 60 => clock_minutes = m,
      m if m < 0 => {
        clock_hours -= 1 + i32::abs(m / 60);
        clock_minutes = 60 + (m % 60);
      }
      m if m >= 60 => {
        clock_hours += m / 60;
        clock_minutes = m % 60;
      }
      _ => (),
    }

    match hours {
      h if h >= 0 && h < 24 => clock_hours += h,
      h if h < 0 => clock_hours += 24 + (h % 24),
      h if h >= 24 => clock_hours += h % 24,
      _ => (),
    }

    if clock_minutes == 60 {
      clock_hours += 1;
      clock_minutes = 0;
    }

    if clock_hours < 0 {
      clock_hours = 24 + (clock_hours % 24);
    }
    if clock_hours >= 24 {
      clock_hours = clock_hours % 24;
    }

    Clock { hours: clock_hours, minutes: clock_minutes }
  }

  pub fn add_minutes(&self, minutes: i32) -> Self {
    let mut current_hours = self.hours;
    let mut current_minutes = self.minutes + minutes;

    if current_minutes < 0 {
      current_hours -= 1 + i32::abs(current_minutes / 60);
      current_minutes = 60 + (current_minutes % 60);
    }

    if current_minutes >= 60 {
      current_hours += current_minutes / 60;
      current_minutes = current_minutes % 60;
    }

    if current_hours < 0 {
      current_hours = 24 + (current_hours % 24);
    }

    if current_hours >= 24 {
      current_hours = current_hours % 24;
    }


    Clock { hours: current_hours, minutes: current_minutes }
  }
}
