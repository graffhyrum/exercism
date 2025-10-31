pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];
    let mut i = 3;
    while primes.len() <= n as usize {
        if primes.iter().all(|&p| i % p != 0) {
            primes.push(i);
        }
        i += 2;
    }
    primes[n as usize]
}
