use crate::app::styles::generic_size::GenericSize;

#[derive(Debug, Copy, Clone)]
pub enum TextSize {
    L,
    XL,
}

impl TextSize {
    pub fn size(&self) -> f32 {
        match self {
            Self::L => 20.0,
            Self::XL => 30.0,
        }
    }
}

impl From<GenericSize> for TextSize {
    fn from(value: GenericSize) -> Self {
        match value {
            GenericSize::L => Self::L,
            GenericSize::XL => Self::XL,
            _ => unimplemented!(),
        }
    }
}
