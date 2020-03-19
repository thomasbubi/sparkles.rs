use std::env;

mod math;
mod scene;
mod camera;
mod intersectables;
mod materials;

use math::Vector3;
use scene::Scene;
use camera::PerspectiveCamera;
use intersectables::Plane;
use intersectables::Sphere;
use materials::Color;
use materials::ShadelessMaterial;
use materials::NormalMaterial;

fn main() {

    //default values for resolution, filename
    let mut print_dog = false;
    let mut width:u32 = 600;
    let mut height:u32 = 400;
    let mut filename ="output.png";
    let mut use_alpha_background = false;

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

    let plane = Plane::new(
        Vector3::new(0.0,0.0,0.0),
        Vector3::new(0.0, 0.0,1.0),
        ShadelessMaterial::new( Color::new(0.2,0.2,0.2,1.0) )
    );

    let sphere = Sphere::new(
        Vector3::new(-0.7,-2.0,0.5),
        0.5,
        NormalMaterial{}
    );

    let mut scene = Scene::new(
        PerspectiveCamera::new(
            Vector3::new(0.0,0.5,0.75),
            Vector3::new(0.0,-1.0,0.0),
            35.0
        )
    );

    if use_alpha_background { scene.use_alpha(); }

    scene.add_object(plane);
    scene.add_object(sphere);
    scene.set_resolution(width, height);
    scene.set_output_filename(filename);
    scene.render();

    //print animal emoji
    if print_dog {
        println!("woof, woof 🐶");
    } else {
        println!("meow 🐱");
    }

}
