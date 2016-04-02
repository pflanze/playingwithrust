use std::env::args_os;

fn main () {
    let x = args_os();
    let a: [i32; 4] = [1, 2, 3, 4];     // As usual, the type annotation is optional.
    println!("The second element is {}", x[1]);
}
