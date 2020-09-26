pub fn a() {
    println!("\n*****\nSLICE_TYPE::A()\n*****\n");
    let s1 = String::from("slice type");
    let first_word: &str = a_word(&s1);

    // s1.clear(); // This breaks because of first_word using s1 as an immutable reference, whereas
    // this tries to change modify it with a mutable type. Which breaks rust's rules.

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

/**
 * Rewriting a_word (now b_word) to accept a slice type in the parameters.
 */

pub fn b() {
    let s: String = String::from("Normal mutable String");
    let s_first_word: &str = b_word(&s[..]);

    let s_literal: &str = "str Literal -- immutable";
    let s_literal_first_word: &str = b_word(&s_literal[..]);
    let s_literal_first_word_better: &str = b_word(s_literal);

    println!(
        "s_first_word: {}\ns_literal_first_word_better: {}",
        s_first_word, s_literal_first_word_better
    );
}

fn b_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
