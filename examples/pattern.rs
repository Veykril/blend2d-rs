use blend2d::{
    codec::ImageCodec,
    context::{CompOp, Context},
    format::ImageFormat,
    geometry::RoundRect,
    gradient::{LinearGradient, LinearGradientValues},
    image::Image,
    pattern::Pattern,
};

fn main() {
    let mut img = Image::new_with(480, 480, ImageFormat::PRgb32).expect("Unable to create image");
    let ctx = Context::from_image(&mut img).expect("Unable to attach rendering context");
    let render = |mut ctx: Context| {
        ctx.set_comp_op(CompOp::SrcCopy)?;
        ctx.fill_all()?;

        // Read an image from file.
        let mut texture = Image::new();
        texture.read_from_file("examples/ferris.png", ImageCodec::built_in_codecs())?;

        // Create a pattern and use it to fill a rounded-rect.
        let pattern = Pattern::new_with(&texture, None, Default::default(), None);

        ctx.set_comp_op(CompOp::SrcOver)?;
        // Draw a solid background.
        ctx.set_fill_style_rgba32(0xFFFFFFFF);
        ctx.fill_round_rect(40.0, 40.0, 400.0, 400.0, 45.5, 45.5)?;
        // Draw the pattern.
        ctx.set_fill_style_pattern(&pattern)?;
        ctx.fill_round_rect(40.0, 40.0, 400.0, 400.0, 45.5, 45.5)?;
        ctx.end()
    };
    render(ctx).expect("Rendering to context failed");

    let codec = ImageCodec::new_by_name("BMP").unwrap();
    img.write_to_file("bl-getting-started-3.bmp", &codec)
        .expect("Writing to file failed");
}
