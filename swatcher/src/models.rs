use csscolorparser::Color;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct SwatchOptions {
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub size: Option<usize>,

    #[serde_as(as = "DisplayFromStr")]
    pub color: Color,

    pub border_thickness: Option<usize>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub border_color: Option<Color>,
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
            border_thickness: None,
            border_color: None,
        }
    }
}

impl SwatchOptions {
    pub fn width(&self) -> usize {
        self.width.or(self.size).unwrap_or(32)
    }

    pub fn height(&self) -> usize {
        self.height.or(self.size).unwrap_or(32)
    }

    pub fn border_color(&self) -> Option<Color> {
        match (&self.border_color, self.border_thickness) {
            (Some(color), _) => Some(color.clone()),
            (None, Some(thickness)) if thickness > 0 => Some(Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }),
            _ => None,
        }
    }

    pub fn border_thickness(&self) -> usize {
        match (&self.border_color, self.border_thickness) {
            (_, Some(thickness)) => thickness,
            (Some(_color), None) => 1,
            (None, None) => 0,
        }
    }
}
