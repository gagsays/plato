mod standard;
mod natural;

pub use self::standard::StandardFrontlight;
pub use self::natural::NaturalFrontlight;
use geom::lerp;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LightLevels {
    pub intensity: f32,
    pub warmth: f32,
}

impl Default for LightLevels {
    fn default() -> Self {
        LightLevels {
            intensity: 0.0,
            warmth: 0.0,
        }
    }
}

impl LightLevels {
    pub fn interpolate(self, other: Self, t: f32) -> Self {
        LightLevels {
            intensity: lerp(self.intensity, other.intensity, t),
            warmth: lerp(self.warmth, other.warmth, t),
        }
    }
}

pub trait Frontlight {
    fn set_intensity(&mut self, value: f32);
    fn set_warmth(&mut self, value: f32);
    fn levels(&self) -> LightLevels;
}

impl Frontlight for LightLevels {
    fn set_intensity(&mut self, value: f32) {
        self.intensity = value;
    }

    fn set_warmth(&mut self, value: f32) {
        self.warmth = value;
    }

    fn levels(&self) -> LightLevels {
        *self
    }
}
