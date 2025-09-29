fn main() {
    let mut s = String::from("hello world");

    // let hello = &mut s[0..5];
    // let world = &mut s[6..11];

    // let part = &s[3..s.len()];

    let k = first_word(&mut s);

    // println!("{}, {}, {}", hello, world);

    println!("{}", k);

    let j = &s[3..4];

    println!("{}", j)
}

fn first_word(s: &mut str) -> &str {
    &s[3..5]
}
