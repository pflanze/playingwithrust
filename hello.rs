use std::env;
//use std::option::Option;

fn main () {
    let x = env::args_os();
    let a: [i32; 4] = [1, 2, 3, 4];     // As usual, the type annotation is optional.
    
    println!("The second element is {}", expect(x.nth(0), "no arg 0"));
}
