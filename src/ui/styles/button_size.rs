use crate::ui::styles::generic_size::GenericSize;
use egui::Vec2;

#[derive(Debug, Clone, Copy)]
pub enum ButtonSize {
    L,
    XL,
}

impl ButtonSize {
    pub fn height(&self) -> f32 {
        match self {
            Self::L => 30.0,
            Self::XL => 40.0,
        }
    }

    pub fn min_size(&self) -> Vec2 {
        Vec2::new(0.0, self.height())
    }

    pub fn size(&self, width: f32) -> Vec2 {
        Vec2::new(width, self.height())
    }
}

impl From<GenericSize> for ButtonSize {
    fn from(value: GenericSize) -> Self {
        match value {
            GenericSize::L => Self::L,
            GenericSize::XL => Self::XL,
            _ => unimplemented!(),
        }
    }
}
