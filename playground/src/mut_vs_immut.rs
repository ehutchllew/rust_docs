pub fn a() {
    println!("\n*****\nMUT_VS_IMMUT::a()\n*****");
    let mut s = String::from("mutable string");

    let r1 = &s;
    let r2 = &s;
    //    let r3 = &mut s;

    println!("{} and {}", r1, r2);

    // Rust won't let you define r3 before
    // r1 & r2 are invoked no more references
    // exist to them. So even though the
    // println! above only invokes r1 & r2,
    // we can't define r3 until after the print.
    let r3 = &mut s;
    println!("{}", r3);
}
