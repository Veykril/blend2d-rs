use blend2d::{
    codec::ImageCodec,
    context::{CompOp, Context},
    format::ImageFormat,
    gradient::{LinearGradient, LinearGradientValues},
    image::Image,
    ExtendMode,
};

fn main() {
    let mut img = Image::new(480, 480, ImageFormat::PRgb32).expect("Unable to create image");

    // Attach a rendering context into `img`.
    let ctx = Context::new(&mut img).expect("Unable to attach rendering context");
    let render = |mut ctx: Context| {
        ctx.set_comp_op(CompOp::SrcCopy)?;
        ctx.fill_all()?;

        // Coordinates can be specified now or changed later.
        let mut linear = LinearGradient::new(
            &LinearGradientValues {
                x0: 0.0,
                y0: 0.0,
                x1: 0.0,
                y1: 480.0,
            },
            ExtendMode::PadXPadY,
            &[],
            None,
        );

        // Color stops can be added in any order.
        linear.add_stop32(0.0, 0xFFFFFFFF)?;
        linear.add_stop32(0.5, 0xFF5FAFDF)?;
        linear.add_stop32(1.0, 0xFF2F5FDF)?;

        // `setFillStyle()` can be used for both colors and styles.
        ctx.set_fill_style_gradient(&linear)?;

        ctx.set_comp_op(CompOp::SrcOver)?;
        ctx.fill_round_rect(40.0, 40.0, 400.0, 400.0, 45.5, 45.5)?;
        ctx.end()
    };
    render(ctx).expect("Rendering to context failed");

    let codecs = ImageCodec::built_in_codecs();
    img.write_to_file(
        "bl-getting-started-2.bmp",
        codecs.find_codec_by_name("BMP").unwrap(),
    )
    .expect("Writing to file failed");
}
