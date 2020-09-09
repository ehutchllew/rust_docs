fn five() -> i32 {
    let x: i32 = 3;
    return x + 3;
}

fn main() {
    let x = five();
    println!("The value of x is: {}", x);
}
