use std::collections::HashSet;

// solves project euler projects

/*
1. If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.
*/
fn problem_1() -> i64 {
    let mut xs = vec!();
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            xs.push(i)
        }
    }
    xs.iter().sum()
}


/*
Each new term in the Fibonacci sequence is generated by adding the previous two terms.
By starting with 1 and 2, the first 10 terms will be:
1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
*/
fn problem_2() -> i64 {
    let mut fib = vec![1, 2];
    let mut a = 1;
    let mut b = 2;
    let mut c;
    let limit = 4_000_000;
    loop {
        c = a + b;
        a = b;
        b = c;
        if c > limit { break; }
        fib.push(c);
    }
    fib.iter().filter(|&x| x % 2 == 0).sum()
}


/*
The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143 ?
*/

fn problem_3() -> i64 {
    // let n = 13195;
    let n: i64 = 600851475143;
    let mut primes = vec!();
    let mut t = n;
    let mut i = 2;
    loop {
        if (t % i == 0) {
            t = t / i;
            primes.push(i);
        }
        if (t <= 1) {
            break
        }
        i += 1;
    }
    primes[primes.len()-1]
}



fn main() {
    // let answer = problem_2();
    // let answer = problem_2();
    let answer = problem_3();
    println!("{}", answer);
}