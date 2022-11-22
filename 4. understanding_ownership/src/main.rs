fn main() {
    println!("Hello, world!");

    {

       let _s = "hello";
    }

    let mut s1 = String::from("hello world");
    // let s2 = s1;

    // takes_ownership(s1);


    println!("{}, world!!", s1);

    // reference borrowing

    change(&mut s1);

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}



fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

