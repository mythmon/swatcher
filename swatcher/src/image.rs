use crate::errors::Error;
use csscolorparser::Color;
use std::io::Write;

pub fn gen_swatch<W: Write>(writer: W, width: u32, height: u32, color: Color) -> Result<(), Error> {
    let num_pixels = (width * height) as usize;
    let color_type = png::ColorType::Rgba;
    let pixel = color.rgba_u8();

    let image_data: Vec<u8> = [pixel.0, pixel.1, pixel.2, pixel.3]
        .iter()
        .cycle()
        .cloned()
        .take(num_pixels * color_type.samples())
        .collect();

    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set_color(color_type);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(image_data.as_slice())?;

    Ok(())
}
