// The Sieve of Eratosthenes is a simple algorithm that finds the prime numbers up to a given integer.
// 
// 
// Task
// Implement the   Sieve of Eratosthenes   algorithm, with the only allowed optimization that the outer loop can stop at the square root of the limit, and the inner loop may start at the square of the prime just found.

// That means especially that you shouldn't optimize by using pre-computed wheels, i.e. don't assume you need only to cross out odd numbers (wheel based on 2), numbers equal to 1 or 5 modulo 6 (wheel based on 2 and 3), or similar wheels based on low primes.
 
// If there's an easy way to add such a wheel based optimization, implement it as an alternative version.


fn sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n];
    is_prime[0] = false;
    is_prime[1] = false;
    let sqrt = (n as f64).sqrt() as usize + 1;

    for i in 2..sqrt {
        if is_prime[i] {
            let mut x = i * i;
            while x < n {
                is_prime[x] = false;
                x = x + i;
            }
        } 
    }
     
    let primes = is_prime.iter().enumerate().filter(|x| *x.1).map(|x| x.0).collect(); 

    primes
}

fn main() {
    let n = 30;
    println!("primes below 30: {:?}", sieve(n));
}

#[test]
fn test_primes_below_thirty() {
    let primes = sieve(30);

    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}
