use csscolorparser::Color;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct SwatchOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub size: Option<u32>,

    #[serde_as(as = "DisplayFromStr")]
    pub color: Color,
}

impl Default for SwatchOptions {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            size: Some(32),
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        }
    }
}

impl SwatchOptions {
    pub fn width(&self) -> u32 {
        self.width.or(self.size).unwrap_or(32)
    }

    pub fn height(&self) -> u32 {
        self.height.or(self.size).unwrap_or(32)
    }
}
