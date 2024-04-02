mod tests;

use fizz_buzz::*;

fn main() {
    let result = fizz_buzz(301);
    // println!("{:?}", result);

    result.iter().for_each(|x| println!("{}", x));

    let fizz_buzz_count = result
        .iter()
        .filter(|&x| x == "fizz buzz")
        .count();

    println!("fizz buzz count: {}", fizz_buzz_count);
}


