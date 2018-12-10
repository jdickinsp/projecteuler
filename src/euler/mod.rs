pub mod problems_1_10;
pub mod problems_11_20;

pub mod utils {
    pub fn get_primes(n: i64) -> Vec<i64> {
        let mut primes: Vec<i64> = vec!();
        let mut not_primes = vec![1; (n+2) as usize];
        let mut i = 2;
        loop {
            if not_primes[i] == 1 {
                primes.push(i as i64);
                let mut j = i;
                loop {
                    j += i;
                    if j > (n as usize) { break }
                    not_primes[j] = 0;
                }
            }
            i += 1;
            if i > (n as usize) { break }
        }
        primes
    }
}
