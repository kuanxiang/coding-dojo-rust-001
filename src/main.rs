use std::*;

#[derive(Debug)]
struct Solution {}

impl Solution {
    //TODO:
    fn add(a: i64, b: i64) -> i64 {
        a + b
    }

    fn sub(a: i64, b: i64) -> i64 {
        a - b
    }

    fn mul(a: i64, b: i64) -> i64 {
        a *b
    }
}

#[test]
fn test_add() {
    assert_eq!(Solution::add(1, 3), 4);
    assert_ne!(Solution::add(7, 9), 6);
}

#[test]
fn test_sub() {
    assert_eq!(Solution::sub(3, 2), 1);
    assert_ne!(Solution::sub(7, 9), 6);
}

#[test]
fn test_mul() {
    assert_eq!(Solution::mul(3, 2), 6);
    assert_ne!(Solution::mul(7, 9), 6);
}


fn main() {
    println!("Hello, world!");
}
