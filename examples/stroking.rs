use blend2d::{
    gradient::{LinearGradient, LinearGradientValues},
    path::{Path, StrokeCap},
    prelude::*,
    ExtendMode,
};

fn main() {
    let mut img = Image::new(480, 480, ImageFormat::PRgb32).expect("Unable to create image");
    let ctx = Context::new(&mut img).expect("Unable to attach rendering context");
    let render = |mut ctx: Context| {
        ctx.set_comp_op(CompOp::SrcCopy);
        ctx.fill_all()?;

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
        linear.add_stop32(0.0, 0xFFFFFFFF);
        linear.add_stop32(1.0, 0xFF1F7FFF);

        let mut path = Path::new();
        path.move_to(119.0, 49.0);
        path.cubic_to(259.0, 29.0, 99.0, 279.0, 275.0, 267.0);
        path.cubic_to(537.0, 245.0, 300.0, -170.0, 274.0, 430.0);

        ctx.set_comp_op(CompOp::SrcOver);
        ctx.set_stroke_style_gradient(&linear);
        ctx.set_stroke_width(15.0);
        ctx.set_stroke_start_cap(StrokeCap::Round);
        ctx.set_stroke_end_cap(StrokeCap::Butt);
        ctx.stroke_path(&path)?;
        ctx.end()
    };
    render(ctx).expect("Rendering to context failed");

    img.write_to_file(
        "bl-getting-started-6.bmp",
        ImageCodec::built_in_codecs()
            .find_codec_by_name("BMP")
            .unwrap(),
    )
    .expect("Writing to file failed");
}
