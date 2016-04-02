use std::env;
//use std::option::Option;



fn main () {
    let mut x = env::args_os();
    println!("The element is {}",
             x.nth(1).expect("no arg 1")
             .to_str().expect("not unicode"));
}
