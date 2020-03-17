mod color;
mod shadelessmaterial;

pub use self::shadelessmaterial::*;
pub use self::color::*;

pub trait Material {
    fn shade(&self) -> Color;
}