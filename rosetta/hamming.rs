// Generate the sequence of Hamming numbers, in increasing order.   In particular:
// 
// Show the   first twenty   Hamming numbers.
// Show the   1691st   Hamming number (the last one below   2^31).
// Show the   one millionth   Hamming number (if the language – or a convenient library – supports arbitrary-precision integers)
 
fn max_divide(mut a: i64, b: i64) -> i64 {
    while a % b == 0 {
        a = a / b;
    }
    
    a
}


fn is_hamming(mut n: i64) -> bool {
    n = max_divide(n, 2);
    n = max_divide(n, 3);
    n = max_divide(n, 5);

    n == 1
}

fn get_nth_hamming(n: i64) -> i64 {
    let mut i = 1;
    let mut count = 1;

    while n > count {
        i += 1;
        if is_hamming(i) {
            count += 1;
        }
    }
    
    i
}


fn gen_hammings(n: i64) -> Vec<i64> {
    let mut i = 1;
    let mut count = 1;
    let mut nums = vec![1];

    while n > count {
        i += 1;
        if is_hamming(i) {
            nums.push(i);
            count += 1;
        }
    }
    
    nums
}


fn main() {
    println!("First 20 hamming numbers: {:?}", gen_hammings(20));
    println!("150th hamming number: {}", get_nth_hamming(150));
    println!("1691st hamming number: {}", get_nth_hamming(1691));
}

#[test]
fn test_nth_hamming() {
    assert_eq!(get_nth_hamming(20), 36);
    assert_eq!(get_nth_hamming(150), 5832);
}
