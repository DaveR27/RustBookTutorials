fn main() {
    let my_string  = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    
    println!("{}", word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

     println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

     println!("{}", word);
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();//so we can check to see if space

//iter - returns each element in a collection
//enumerate - returns each element in a tuple, 0 = index, 1 = ref to element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
