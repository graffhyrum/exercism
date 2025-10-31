use std::collections::HashMap;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut unique_factors = HashMap::new();
    factors
        .iter()
        .filter(|&factor| factor != &0_u32)
        .for_each(|&factor| {
            let mut i = 1;
            while factor * i < limit {
                unique_factors.insert(factor * i, true);
                i += 1;
            }
        });
    unique_factors.keys().sum()
}
