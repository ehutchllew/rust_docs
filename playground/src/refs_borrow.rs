/**
 * Function giving and returning ownership
 */

pub fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of {} is {},", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

/**
 * Function just BORROWING ownership
 */

pub fn b() {
    let s1 = String::from("hello world");

    let len = b_calc(&s1);

    println!("Length of {} is {}", s1, len);
}

fn b_calc(s: &String) -> usize {
    let length = s.len();
    return length;
}
