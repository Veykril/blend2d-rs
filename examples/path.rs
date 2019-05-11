use blend2d::{path::Path, prelude::*};

fn main() {
    let mut img = Image::new(480, 480, ImageFormat::PRgb32).expect("Unable to create image");

    // Attach a rendering context into `img`.
    let ctx = Context::new(&mut img).expect("Unable to attach rendering context");
    // The closure here just acts as a `try` block to catch possible errors
    let render = |mut ctx: Context| {
        // Clear the image.
        ctx.set_comp_op(CompOp::SrcCopy)?;
        ctx.fill_all()?;

        // Fill some path.
        let mut path = Path::new();
        path.move_to(26.0, 31.0)?;
        path.cubic_to(642.0, 132.0, 587.0, -136.0, 25.0, 464.0)?;
        path.cubic_to(882.0, 404.0, 144.0, 267.0, 27.0, 31.0)?;

        ctx.set_comp_op(CompOp::SrcOver)?;
        ctx.set_fill_style_rgba32(0xFFFFFFFF)?;
        ctx.fill_geometry(&path)?;

        // Detach the rendering context from `img`.
        ctx.end()
    };
    render(ctx).expect("Rendering to context failed");

    // Let's use some built-in codecs provided by Blend2D.
    let codecs = ImageCodec::built_in_codecs();
    img.write_to_file(
        "bl-getting-started-1.bmp",
        codecs.find_codec_by_name("BMP").unwrap(),
    )
    .expect("Writing to file failed");
}
