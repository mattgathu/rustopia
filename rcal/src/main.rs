#![feature(inclusive_range_syntax)]
extern crate libc;

use std::ops::Rem;
use std::collections::HashMap;
use libc::{localtime, time_t, time};


fn get_month_name(m: usize) -> String {
    let mut map: HashMap<usize, &str> = HashMap::new();
    let ms = vec!["January", "February", "March", "April", "May", "June",
                  "July", "August", "September", "October", "November", "December"];
    for i in 1...12 {
        map.insert(i, ms[i-1]);
    }
    
    map.get(&m).unwrap().to_string()
}

fn main() {
    let now: time_t= unsafe {
        let mut zero: i64 = 0;
        let ptr: *mut i64 = &mut zero;
        time(ptr) 
    };

    let tm = unsafe { * localtime(&now) };

    let (year, month, day) = (tm.tm_year+1900, tm.tm_mon+1, tm.tm_mday);

    let mut days_of_month = HashMap::new();
    for m in vec![1, 3, 5, 7, 8, 10, 12] {
        days_of_month.insert(m, 31);
    }
    for m in vec![4, 6, 9, 11] {
        days_of_month.insert(m, 30);
    }

    if year.rem(4) == 0 {
        days_of_month.insert(2, 29);
    } else {
        days_of_month.insert(2, 28);
    }

    println!("{:^20}", format!("{} {}", get_month_name(month as usize), year));
    println!("{}", "Su Mo Tu We Th Fr Sa");


    let mut days = (1...*days_of_month.get(&month).unwrap()).collect::<Vec<_>>();
    
    let wdays =  unsafe {
        let start = &now - (((day-1) * 86400) as i64);
        let stm = * localtime(&start);
        stm.tm_wday
    };

    for _ in 0..wdays {
        days.insert(0, 99);
    }

    for c in days.chunks(7) {
        for day in c {
            if *day != 99 {
                print!("{:>2} ", day);
            } else {
                print!("{:>2} ", ' ');
            }
        }
        println!()
    }

}


#[test]
fn test_get_month_name() {
    assert_eq!(get_month_name(6usize), "June");
    assert_eq!(get_month_name(5usize), "May");
}
