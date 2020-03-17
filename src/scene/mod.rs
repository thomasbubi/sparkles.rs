use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use crate::camera::PerspectiveCamera;
use crate::math::Vector3;
use crate::intersectables::Intersectable;
use std::borrow::Borrow;

pub struct Scene {
    resolution_x : u32,
    resolution_y : u32,
    use_alpha_transparency : bool,
    use_aa: bool,
    t_max: f64,
    t_min: f64,
    max_recursion_depth: u32,
    spp_glossy: u32,
    filename: String,
    camera: PerspectiveCamera,
    objects: Vec<Box<Intersectable>>
}

impl Scene {
    pub fn new(mut camera: PerspectiveCamera) -> Scene {
        let null_vec3 = Vector3::new(0.0,0.0, 0.0);
        Scene {
            resolution_x: 600,
            resolution_y: 400,
            use_alpha_transparency: false,
            use_aa: false,
            t_max: 1_000_000.0,
            t_min: 0.05,
            max_recursion_depth: 3,
            spp_glossy: 10,
            filename: "sparkles_rendering.png".to_string(),
            camera,
            objects: Vec::new()
        }
    }

    //using stativ lifetime here, since the objects are needed as long the program is running
    pub fn add_object(&mut self, obj: impl Intersectable + 'static){
        self.objects.push(Box::new(obj) );
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution_x = width;
        self.resolution_y = height;
        self.camera.set_resolution(width, height);
    }

    pub fn set_output_filename(&mut self, filename: &str) {
        self.filename = filename.to_string();
    }

    pub fn render(&self) {
        render(self.camera.borrow(),self.filename.clone(), self.resolution_x,self.resolution_y);
    }
}

fn render(camera: &PerspectiveCamera, filename: String, width_u32: u32, height_u32: u32){
    let width = width_u32 as usize;
    let height = height_u32 as usize;
    let mut num_pixels = width * height * 4;
    let mut image = Vec::with_capacity(num_pixels);
    let mut counter:usize = 0;

    //fill vector with white pixels
    while counter < num_pixels {
        image.push(255u8);
        counter += 1
    }

    //iterate over every pixel
    for j in 0..height {
        for i in 0..width {
            let ray = camera.create_camera_ray(i,j);
            fill_pixel(width,i,j,&mut image, 255,127,127,255);
        }
    }

    write_png(filename, width_u32, height_u32, &image);
}

fn fill_pixel(width: usize, i: usize, j: usize, image: &mut Vec<u8>, r: u8, g: u8, b: u8, a: u8){
    let idx = (j * width + i) * 4;
    image[idx] = r;
    image[idx+1] = g;
    image[idx+2] = b;
    image[idx+3] = a;
}

fn write_png(filename: String, width: u32, height: u32, image: &[u8]) {
    //https://docs.rs/png/0.16.1/png/#using-the-encoder
    let path = Path::new(&filename);
    let file = File::create(path).unwrap();
    let ref mut buf_writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(buf_writer, width, height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(image).unwrap();

}