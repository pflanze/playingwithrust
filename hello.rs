use std::env;
//use std::option::Option;

fn main () {
    let x = env::args_os();
    println!("The second element is {}",
             x.nth(0).expect("no arg 0").to_str().expect("not unicode"));
}
