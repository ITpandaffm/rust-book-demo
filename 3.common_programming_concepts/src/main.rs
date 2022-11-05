fn main() {
    println!("Hello, world!");

    // variables
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("now x is {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("constant :: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let y = 6;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");

    }

    println!("THe value of y is {y}");
}
