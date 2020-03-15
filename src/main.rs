use std::env;

mod math;

use math::Vector3;

fn main() {

    //default values for resolution, filename
    let mut print_dog = false;
    let mut width:u32 = 600;
    let mut height:u32 = 400;
    //let mut filename ="output.png";
    //let mut use_alpha_background = false;

    //get command-line arguments
    //https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let args: Vec<String> = env::args().collect();
    let num_args = args.len();

    //iterate over them
    for (i, arg) in args.iter().enumerate() {

        if (arg == "--width" || arg == "-w") && i+1 < num_args {
            width = args[i+1].parse::<u32>().unwrap();
        }

        if (arg == "--height" || arg == "-h") && i+1 < num_args {
            height = args[i+1].parse::<u32>().unwrap();
        }

        if arg == "--dog" {
            print_dog = true;
        }

    }

    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(-7.0, 8.0, 9.0);
    let c = Vector3::new(1 as f64,1 as f64,1 as f64);
    let d = c.normalize();
    println!("{}",a+b);
    println!("{}",d);

    //print animal emoji
    if print_dog {
        println!("woof, woof 🐶");
    } else {
        println!("meow 🐱");
    }

}
