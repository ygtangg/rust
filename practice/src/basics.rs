fn basic() {
  // Defining variable and printing strings
    // variables are immutable by default
    let first = "first";
    let last = "LAST";
    println!("Hello, {first} {}!", last.to_lowercase());

    // Printing arrays to terminal in debug mode -- :?
    let data = [1, 2, 3, 4];
    println!("{data:?}");

    // Two types of strings: 
    // a string *literal*; type is `&'static str` (a string slice).
    // it's baked into the binary (read-only “static” data) and is immutable.
    let normal = "normal_string_def";
    println!("{normal}");

    // a growable, owned UTF-8 string.
    // the `String` value (ptr/len/cap) is on the stack; its contents live on the heap.
    // make the binding `mut` if you plan to modify it.
    let mut other = String::from("string_type_instance"); 
    other.push('!');
    println!("{other}");
}