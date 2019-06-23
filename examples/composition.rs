use blend2d::{
    gradient::{Gradient, LinearGradientValues, RadialGradientValues},
    prelude::*,
    ExtendMode,
};

fn main() {
    let mut img = Image::new(480, 480, ImageFormat::PRgb32).expect("Unable to create image");
    let ctx = Context::new(&mut img).expect("Unable to attach rendering context");
    let render = |mut ctx: Context| {
        ctx.set_comp_op(CompOp::SrcCopy);
        ctx.fill_all()?;

        let mut radial = Gradient::new_radial(
            &RadialGradientValues {
                x0: 180.0,
                y0: 180.0,
                x1: 180.0,
                y1: 180.0,
                r0: 180.0,
            },
            ExtendMode::PadXPadY,
            &[],
            None,
        );
        radial.add_stop32(0.0, 0xFFFFFFFF);
        radial.add_stop32(1.0, 0xFFFF6F3F);

        ctx.set_comp_op(CompOp::SrcOver);
        ctx.set_fill_style_gradient(&radial);
        ctx.fill_circle(180.0, 180.0, 160.0)?;

        let mut linear = Gradient::new_linear(
            &LinearGradientValues {
                x0: 195.0,
                y0: 195.0,
                x1: 470.0,
                y1: 470.0,
            },
            ExtendMode::PadXPadY,
            &[],
            None,
        );
        linear.add_stop32(0.0, 0xFFFFFFFF);
        linear.add_stop32(1.0, 0xFF3F9FFF);

        ctx.set_comp_op(CompOp::Difference);
        ctx.set_fill_style_gradient(&linear);
        ctx.fill_round_rect(195.0, 195.0, 270.0, 270.0, 25.0, 25.0)?;
        ctx.end()
    };
    render(ctx).expect("Rendering to context failed");

    img.write_to_file(
        "bl-getting-started-5.bmp",
        ImageCodec::built_in_codecs()
            .find_codec_by_name("BMP")
            .unwrap(),
    )
    .expect("Writing to file failed");
}
