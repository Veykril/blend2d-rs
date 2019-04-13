use blend2d::{
    codec::ImageCodec,
    context::{CompOp, Context},
    format::ImageFormat,
    gradient::{LinearGradient, LinearGradientValues},
    image::Image,
};

fn main() {
    let mut img = Image::new_with(480, 480, ImageFormat::PRgb32).expect("Unable to create image");
    let mut ctx = Context::from_image(&mut img).expect("Unable to attach rendering context");
    ctx.set_comp_op(CompOp::SrcCopy).unwrap();
    ctx.fill_all().unwrap();

    let mut linear = LinearGradient::new_with(
        &LinearGradientValues {
            x0: 0.0,
            y0: 0.0,
            x1: 0.0,
            y1: 480.0,
        },
        Default::default(),
        &[],
        None,
    );
    linear.add_stop32(0.0, 0xFFFFFFFF).unwrap();
    linear.add_stop32(0.5, 0xFF5FAFDF).unwrap();
    linear.add_stop32(1.0, 0xFF2F5FDF).unwrap();

    ctx.set_fill_style_gradient(&linear).unwrap();

    ctx.set_comp_op(CompOp::SrcOver).unwrap();
    ctx.fill_all().unwrap();
    ctx.end();
    let codec = ImageCodec::new_by_name("BMP").unwrap();
    img.write_to_file("gradient_example.bmp", &codec).unwrap();
}
