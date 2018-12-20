use std::cmp;
/*
big integer class
- stores large ints in chunk sizes in a stack
- supports add and multiply
*/

const CHUNK_SIZE: usize = 8;

// split string into chunk size integers
fn split_chunks(val: &str) -> Vec<i64> {
    let mut parts = vec!();
    let mut j = val.len();
    let mut i = if j > CHUNK_SIZE { j - CHUNK_SIZE } else { 0 };
    while j > 0 {
        let part = &val[i..j];
        parts.push(part.parse::<i64>().unwrap());
        j = i;
        i = if i > CHUNK_SIZE { i - CHUNK_SIZE } else { 0 };
    }
    parts
}

pub struct BigInt {
    // stored in reverse order
    chunks: Vec<i64>
}

impl BigInt {
    fn new(val: &str) -> BigInt {
        // split into chunks
        let chunks = split_chunks(val);
        // println!("BitInteger::new {:?}", chunks);
        BigInt { chunks: chunks }
    }
    fn add(&self, b: &BigInt) -> BigInt {
        let mut sum = vec!();
        let max_len = cmp::max(self.chunks.len(), b.chunks.len());
        let mut carry = 0;
        for i in 0..max_len {
            let x = if i < self.chunks.len() { self.chunks[i] } else { 0 };
            let y = if i < b.chunks.len() { b.chunks[i] } else { 0 };
            let s = x + y + carry;
            // check overflow
            carry = s / (i64::pow(10, CHUNK_SIZE as u32));
            let rem = s % i64::pow(10, CHUNK_SIZE as u32);
            sum.push(rem);
        }
        if carry > 0 { sum.push(carry) };
        BigInt{ chunks: sum }
    }
    fn multiply(&self, b: &BigInt) -> BigInt {
        let max_len = cmp::max(self.chunks.len(), b.chunks.len());
        let mut middle_sums = vec!();
        for j in 0..b.chunks.len() {
            let mut sum = vec!();
            // add zeros place
            for k in 0..j {
                sum.push(0);
            }
            let mut carry = 0;
            for i in 0..max_len {
                let x = if i < self.chunks.len() { self.chunks[i] } else { 1 };
                let y = b.chunks[j];
                let s = x * y + carry;
                carry = s / (i64::pow(10, CHUNK_SIZE as u32));
                let rem = s % i64::pow(10, CHUNK_SIZE as u32);
                sum.push(rem);
            }
            if carry > 0 { sum.push(carry) };
            middle_sums.push(BigInt{ chunks: sum });
        }
        let mut total = BigInt::new("0");
        for i in middle_sums.iter() {
            total = total.add(i);
        }
        total
    }
    pub fn value(&self) -> String {
        let mut digits = vec!();
        for (i, chunk) in self.chunks.iter().enumerate().rev() {
            let valstr = if i == self.chunks.len() - 1 { chunk.to_string() } else { format!("{:02}", chunk) };
            digits.push(valstr);
        }
        digits.join("")
    }
}


pub fn big_exponential(a: i64, x: i64) -> BigInt {
    let base = BigInt::new(&a.to_string());
    let mut total = BigInt::new("1");
    for _ in 0..x {
        total = total.multiply(&base);
    }
    total
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bigint_add() {
        let a = BigInt::new("99999999923456789");
        let b = BigInt::new("99950999");
        let x = a.add(&b);
        let y: i128 = 99999999923456789 + 99950999;
        assert_eq!(x.value(), y.to_string());
    }

    #[test]
    fn test_bigint_multiply() {
        let a = BigInt::new("99999999923456789");
        let b = BigInt::new("1299950999");
        let x = a.multiply(&b);
        let y: i128 = 99999999923456789 * 1299950999;
        assert_eq!(x.value(), y.to_string());
    }

    #[test]
    fn test_bigint_exponential() {
        let x = big_exponential(2, 10);
        assert_eq!(x.value(), "1024");
    }
}
