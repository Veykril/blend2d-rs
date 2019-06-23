use blend2d::{pattern::Pattern, prelude::*};

fn main() {
    let mut img = Image::new(480, 480, ImageFormat::PRgb32).expect("Unable to create image");
    let ctx = Context::new(&mut img).expect("Unable to attach rendering context");
    let render = |mut ctx: Context| {
        ctx.set_comp_op(CompOp::SrcCopy);
        ctx.fill_all()?;

        // Read an image from file.
        let texture = Image::from_path("assets/ferris.png", &ImageCodec::built_in_codecs())?;

        // Create a pattern and use it to fill a rounded-rect.
        let pattern = Pattern::new(&texture, None, Default::default(), None);

        ctx.set_comp_op(CompOp::SrcOver);
        // Draw a solid background.
        ctx.set_fill_style_rgba32(0xFFFFFFFF);
        ctx.fill_round_rect(40.0, 40.0, 400.0, 400.0, 45.5, 45.5)?;
        // Draw the pattern.
        ctx.set_fill_style_pattern(&pattern);
        ctx.fill_round_rect(40.0, 40.0, 400.0, 400.0, 45.5, 45.5)?;
        ctx.end()
    };
    render(ctx).expect("Rendering to context failed");

    img.write_to_file(
        "bl-getting-started-3.bmp",
        ImageCodec::built_in_codecs()
            .find_codec_by_name("BMP")
            .unwrap(),
    )
    .expect("Writing to file failed");
}
