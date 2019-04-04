use blend2d::{
    codec::ImageCodec,
    context::{CompOp, Context},
    format::ImageFormat,
    image::Image,
    path::Path,
};

fn main() {
    let mut img = Image::new(480, 480, ImageFormat::PRgb32).expect("Unable to create image");
    {
        let mut ctx =
            Context::new_from_image(&mut img).expect("Unable to attach rendering context");
        ctx.set_comp_op(CompOp::SrcCopy).unwrap();
        ctx.fill_all().unwrap();

        let mut path = Path::new();
        path.move_to(26.0, 31.0).unwrap();
        path.cubic_to(642.0, 132.0, 587.0, -136.0, 25.0, 464.0)
            .unwrap();
        path.cubic_to(882.0, 404.0, 144.0, 267.0, 27.0, 31.0)
            .unwrap();

        ctx.set_comp_op(CompOp::SrcOver).unwrap();
        ctx.set_fill_style_rgba32(0xFFFFFFFF).unwrap();
        ctx.fill_path(&path).unwrap();
    }
    let codec = ImageCodec::by_name("BMP").unwrap();
    img.write_to_file("path_example.bmp", &codec).unwrap();
}
