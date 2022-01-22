fn main() {
    let mut x: u64 = 1; // a mutable 64 bit unsigned integer

    println!("The value of x is: {}", x); // printing variables in rust
    x = 1;
    println!("The value of x is: {}", x);

    loop {
        if x > 999999999999999999 {
            break;
        }
        x = x*2;
        println!("x: {}", x);
    }
    x = 3;

    let y = true;
    match(x, y)
    {
        (3, true) => println!("x is 3, y is true"),
        (3, false) => println!("x is 3, y is false"),
        (_, _) => println!("other combinations")
    }

    for x in 0..=10{ // loops from 0 to 10, inclusive
        println!("x = {}", x);
    }
}
