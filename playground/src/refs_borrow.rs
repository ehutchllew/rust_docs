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
    // When this function returns `s` goes out of scope.
    // if it owned whatever value was associated with `s` then
    // that ref would be GC'd; however, since it's a ref type
    // it simply REFERS to the value, but doesn't own it.
    // This then has no side-effect when the function returns.
}

/**
 * Function borrowing and mutating value
 */
pub fn c() {
    let mut s = String::from("mutate me.");
    println!("S before mutation is: {}", s);

    c_mut(&mut s);
    println!("S AFTER mutation is: {}", s);
}

fn c_mut(str: &mut String) {
    str.push_str(" Done!");
}
