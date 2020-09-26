pub fn a() {
    println!("\n*****\nSLICE_TYPE::A()\n*****\n");
    let s1 = String::from("slice type");
    let first_word: &str = a_word(&s1);
    println!("S1 is: {} \n first word is: {}", s1, first_word);
}

fn a_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
