pub struct UV {
    pub u: f64,
    pub v: f64
}

impl UV {
    pub fn new(u: f64, v:f64) -> UV {
        UV{u,v}
    }
}

pub trait Texture {
    fn get_value_at(&self, uv: UV) -> Color;
}