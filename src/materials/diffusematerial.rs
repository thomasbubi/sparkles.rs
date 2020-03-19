use crate::materials::{Color, Material, ShaderInput};
use crate::textures::{Texture, SingleColorTexture, UV};

pub struct DiffuseMaterial {
    texture: Box<Texture>
}

impl DiffuseMaterial {
    pub fn new(color: Color) -> DiffuseMaterial {
        DiffuseMaterial {
            texture: Box::new( SingleColorTexture::new(color) )
        }
    }
}

impl Material for DiffuseMaterial {
    fn shade(&self, input: ShaderInput) -> Color {
        self.texture.get_value_at(UV::new(0.0,0.0))
    }
}