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
    }
    println!("testing clone");
    {
        let mut ctx =
            Context::new_from_image(&mut img).expect("Unable to attach rendering context");
        ctx.clone();
    }
    println!("testing double clone");
    {
        let mut ctx =
            Context::new_from_image(&mut img).expect("Unable to attach rendering context");
        ctx.clone();
        ctx.clone();
    }
}
