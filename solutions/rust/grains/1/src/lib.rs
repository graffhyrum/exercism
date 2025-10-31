pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 2_u64.pow(s-1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    let mut total: u64 = 0;
    for i in 1..=64 {
        total += square(i);
    }
    total
}
