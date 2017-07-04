// In mathematics, the Fibonacci numbers are the numbers in the 
// Fibonacci sequence, characterized by the fact that every number 
// after the first two is the sum of the two preceding ones.
//
// Task
// Write a function to generate the   nth   Fibonacci number. 
//
// http://rosettacode.org/wiki/Fibonacci_sequence

fn fib(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2),
    }
}

fn fib_nth(n: i64) -> i64 {
    let mut cnt = 1;
    let mut a_b = (0, 1); 

    while cnt <= n {
        a_b = (a_b.1, a_b.0 + a_b.1); 
        cnt += 1;
    }

    a_b.1
}

fn main() {
    println!("fib(5): {}", fib(5));
    println!("fib_nth(5): {}", fib_nth(5));
}


#[test]
fn test_fib() {
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(3), 2);
    assert_eq!(fib(4), 3);
    assert_eq!(fib(5), 5);
    assert_eq!(fib(6), 8);
    assert_eq!(fib(7), 13);
    assert_eq!(fib(9), 34);
}

#[test]
fn test_fib_nth() {
    assert_eq!(fib_nth(5), 8);
    assert_eq!(fib_nth(6), 13);
    assert_eq!(fib_nth(8), 34);
}
