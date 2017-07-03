// Task
// Write a function to return the factorial of a number.
// 
// Solutions can be iterative or recursive.
// 
// Support for trapping negative   n   errors is optional. 


// recursive
fn recursive_factorial(n: i64) -> i64 {
    match n {
        0 => 1,
        _ => n * recursive_factorial(n-1),
    }
}

// iterative
fn iterative_factorial(n: i64) -> i64 {
    (1..n+1).fold(1, |acc, x| acc * x)
}

fn main() {
    println!("10!: {}", recursive_factorial(10));
    println!("14!: {}", iterative_factorial(14));
}


#[test]
fn test_recursive_factorial() {
    assert_eq!(recursive_factorial(4), 24);
}

#[test]
fn test_iterative_factorial() {
    assert_eq!(iterative_factorial(4), 24);
}
