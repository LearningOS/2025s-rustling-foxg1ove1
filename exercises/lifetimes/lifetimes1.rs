// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

// fn longest(x: &str, y: &str) -> &str {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    fn main() {
        let string1 = String::from("abcd");
        let string2 = "xyz";
    
        let result = longest(string1.as_str(), string2);
        println!("The longest string is '{}'", result);
    }
    