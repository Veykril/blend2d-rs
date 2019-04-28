use blend2d::{
    codec::ImageCodec,
    context::{CompOp, Context},
    font::FontFace,
    format::ImageFormat,
    geometry::PointD,
    image::Image,
    matrix::MatrixTransform,
};

fn main() {
    let mut img = Image::new(480, 480, ImageFormat::PRgb32).expect("Unable to create image");
    let ctx = Context::new(&mut img).expect("Unable to attach rendering context");
    let render = |mut ctx: Context| {
        ctx.set_comp_op(CompOp::SrcCopy)?;
        ctx.fill_all()?;

        let font_face = FontFace::from_path("assets/NotoSans-Regular.ttf")?;
        let font = font_face.create_font(50.0)?;

        ctx.set_fill_style_rgba32(0xFFFFFFFF)?;
        ctx.fill_utf8_text(PointD { x: 60.0, y: 80.0 }, &font, "Hello Blend2D!")?;

        ctx.rotate(core::f64::consts::FRAC_PI_4)?;
        ctx.fill_utf8_text(PointD { x: 250.0, y: 80.0 }, &font, "Rotated Text!")?;
        ctx.end()
    };
    render(ctx).expect("Rendering to context failed");

    let codec = ImageCodec::built_in_codecs()
        .find_codec_by_name("BMP")
        .unwrap();
    img.write_to_file("bl-getting-started-7.bmp", codec)
        .expect("Writing to file failed");
}
