// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

fn main() {
    // Difference between String and &str?
    // String is an actual String
    // &str is reference to a String
    //
    // rust ownership is just memory management, handled automatically
    // at the scope/compiler level. Follow the rules and it works for
    // you.
    //
    // Generally, if it compiles for you, it will work for you.
    let mut shopping_list: Vec<&str> = Vec::new();
    // Any literal string in rust code is a reference to a string
    // (&str). Push the address of "milk" on to the vector.
    shopping_list.push("milk");
}
