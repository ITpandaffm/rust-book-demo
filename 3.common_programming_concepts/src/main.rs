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


    another_function(5, 6);


    // Control Flow 
    let z = 3;
    if z < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

//  FUcntions

fn another_function(x: i32, y: i32) {
    println!("Another function. {x}, y = {y}");


}
