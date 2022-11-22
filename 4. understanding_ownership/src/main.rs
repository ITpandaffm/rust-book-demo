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

    // let mut s2 = String::from("hello world");

    // let word = first_word(&s2);
    // println!("word:: {}", word);

    // s2.clear();

    // println!("now word: {word}");

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
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


// slice 
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
   return &s[..];

}


