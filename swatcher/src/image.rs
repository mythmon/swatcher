use crate::{errors::Error, models::SwatchOptions};
use csscolorparser::Color;
use std::io::Write;

pub fn gen_swatch<W: Write>(writer: W, opts: SwatchOptions) -> Result<(), Error> {
    let num_pixels = opts.width() * opts.height();
    let color_type = png::ColorType::Rgba;
    let pixel = opts.color.rgba_u8();

    let mut image_data = ImageData::new(
        opts.width(),
        opts.height(),
        color_type,
        [pixel.0, pixel.1, pixel.2, pixel.3]
            .iter()
            .cycle()
            .cloned()
            .take(num_pixels * color_type.samples()),
    )?;

    if let Some(border_color) = opts.border_color() {
        for t in 0..opts.border_thickness() {
            for x in 0..image_data.width {
                image_data.set(x, t, border_color.clone());
                image_data.set(x, image_data.height - 1 - t, border_color.clone());
            }
            for y in 0..image_data.height {
                image_data.set(t, y, border_color.clone());
                image_data.set(image_data.width - 1 - t, y, border_color.clone());
            }
        }
    }

    let mut encoder = png::Encoder::new(writer, opts.width() as u32, opts.height() as u32);
    encoder.set_color(color_type);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(image_data.as_slice())?;

    Ok(())
}

struct ImageData {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl ImageData {
    fn new<I>(
        width: usize,
        height: usize,
        color_type: png::ColorType,
        iter: I,
    ) -> Result<Self, Error>
    where
        I: IntoIterator<Item = u8>,
    {
        if color_type != png::ColorType::Rgba {
            return Err(Error::ColorFormat);
        }
        let num_subpixels = width * height * 4;
        let data = iter
            .into_iter()
            .chain(std::iter::repeat(0))
            .take(num_subpixels)
            .collect::<Vec<_>>();

        Ok(Self {
            width,
            height,
            data,
        })
    }

    fn set(&mut self, x: usize, y: usize, c: Color) {
        let p = x * 4 + y * 4 * self.height;
        let pixel = c.rgba_u8();
        self.data[p] = pixel.0;
        self.data[p + 1] = pixel.1;
        self.data[p + 2] = pixel.2;
        self.data[p + 3] = pixel.3;
    }

    fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }
}
