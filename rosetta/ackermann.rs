// The Ackermann function is a classic example of a recursive function, 
// notable especially because it is not a primitive recursive function. 
// It grows very quickly in value, as does the size of its call tree.
//
// The Ackermann function is usually defined as follows:
//
//      A(m,n)=:
//          n+1 if m=0 
//          A(m-1,1) if m>0 and n=0 
//          A(m-1,A(m,n-1)) if m>0 and n>0
// 
// Task
// Write a function which returns the value of A(m,n)
// 


fn ackermann(m: i64, n: i64) -> i64 {
    let mn = (m, n);

    match mn {
        (0, n) => n+1,
        (m, 0) => ackermann(m-1, 1),
        _ => ackermann(m-1, ackermann(m, n-1)),
    }
}


fn main() {
    println!("A(1,2): {}", ackermann(1, 2));
}


#[test]
fn test_ackermann() {
    assert_eq!(ackermann(0, 0), 1);
    assert_eq!(ackermann(0, 2), 3);
    assert_eq!(ackermann(1, 4), 6);
    assert_eq!(ackermann(2, 4), 11);
    assert_eq!(ackermann(3, 4), 125);
    assert_eq!(ackermann(3, 3), 61);
}
