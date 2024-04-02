#![cfg(test)]

use fizz_buzz::*;

#[test]
fn test_fizz_buzz_count() {
    let res = fizz_buzz(301);
    let fizz_buzz_count = res
        .iter()
        .filter(|&x| x == "fizz buzz")
        .count();
    assert_eq!(fizz_buzz_count, 21);
}

#[test]
fn test_not_divisible_by_3_nor_5() {
    let res = fizz_buzz(1);
    assert_eq!(res.len(), 1);
}

#[test]
fn test_divisible_by_3() {
    let res = fizz_buzz(3);
    assert_eq!(res.len(), 2);
    assert_eq!(res[1], "fizz".to_string());
}

#[test]
fn test_divisible_by_5() {
    let res = fizz_buzz(5);
    assert_eq!(res.len(), 3);
    assert_eq!(res[2], "buzz".to_string());
}

#[test]
fn test_divisible_by_3_and_5_inclusive() {
    let res = fizz_buzz(15);
    assert_eq!(res.len(), 8);
    assert_eq!(res[7], "fizz buzz".to_string());
}

#[test]
fn test_zero_divisible_by_3_and_5() {
    let res = fizz_buzz(0);
    assert_eq!(res.len(), 1);
    assert_eq!(res[0], "fizz buzz".to_string());
}
