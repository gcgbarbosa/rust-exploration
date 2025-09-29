fn main() {
    println!("testing");

    let x = 5;

    println!("x is: {x}");

    let x = 10;

    let a = [1, 2, 3, 4, 5];

    let first = a[0];

    println!("x is: {x}, {}, {}", first, a[3]);

    another_function("hello");

    let f_return = function_with_return();

    // this is a comment
    println!("function with return {f_return}");

    /*
     * this is a multiline comment
     * */

    if first == 1 {
        println!("first is one");
    }

    let k = if x == 10 { 5 } else { 0 };

    println!("test {}", k);

    let mut counter = 0;

    loop {
        println!("again! {}", counter);
        counter += 1;

        if counter == 10 {
            break;
        }
    }

    while counter > 0 {
        counter -= 1;
        println!("again! {}", counter);
    }

    for number in (1..10).rev() {
        println!("test {}", number)
    }
}

fn another_function(x: &str) {
    println!("string {x}")
}

fn function_with_return() -> u32 {
    5
}
