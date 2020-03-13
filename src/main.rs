use std::env;

fn main() {

    //default values for resolution, filename
    //let mut width:u32 = 600;
    //let mut height:u32 = 400;
    //let mut filename ="output.png";

    //get command-line arguments
    //https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let args: Vec<String> = env::args().collect();

    //print animal emoji
    if args.contains(&"--dog".to_string()) {
        println!("woof, woof 🐶");
    } else {
        println!("meow 🐱");
    }

}
