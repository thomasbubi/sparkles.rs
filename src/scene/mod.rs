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
            filename: "output.png".to_string()
        }
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution_x = width;
        self.resolution_y = height;
    }

    pub fn render(&self) {
        render();
    }
}

fn render(){
    println!("render");
}