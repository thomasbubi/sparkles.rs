use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

pub struct Scene {
    resolution_x : u32,
    resolution_y : u32,
    use_alpha_transparency : bool,
    use_aa: bool,
    t_max: f64,
    t_min: f64,
    max_recursion_depth: u32,
    spp_glossy: u32,
    filename: String
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            resolution_x: 600,
            resolution_y: 400,
            use_alpha_transparency: false,
            use_aa: false,
            t_max: 1_000_000.0,
            t_min: 0.05,
            max_recursion_depth: 3,
            spp_glossy: 10,
            filename: "sparkles_rendering.png".to_string()
        }
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution_x = width;
        self.resolution_y = height;
    }

    pub fn set_output_filename(&mut self, filename: &str) {
        self.filename = filename.to_string();
    }

    pub fn render(&self) {
        render(self.filename.clone(), self.resolution_x,self.resolution_y);
    }
}

fn render(filename: String, width: u32, height: u32){
    let mut num_pixels = (width * height * 4) as usize;
    let mut image = Vec::with_capacity(num_pixels);
    let mut counter:usize = 0;

    //fill vector with white pixels
    while counter < num_pixels {
        image.push(255u8);
        counter += 1
    }

    write_png(filename, width, height, &image);
}

fn write_png(filename: String, width: u32, height: u32, image: &[u8]) {
    let path = Path::new(&filename);
    let file = File::create(path).unwrap();
    let ref mut buf_writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(buf_writer, width, height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(image).unwrap();

}