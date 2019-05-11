use blend2d::{font::FontFace, geometry::PointD, glyph_buffer::GlyphBuffer, prelude::*};

fn main() {
    let mut img = Image::new(480, 480, ImageFormat::PRgb32).expect("Unable to create image");
    let ctx = Context::new(&mut img).expect("Unable to attach rendering context");
    let render = |mut ctx: Context| {
        ctx.set_comp_op(CompOp::SrcCopy)?;
        ctx.fill_all()?;
        ctx.set_fill_style_rgba32(0xFFFFFFFF)?;

        let font_face = FontFace::from_path("assets/NotoSans-Regular.ttf", DataAccessFlags::READ)?;
        let font = font_face.create_font(20.0)?;
        let fm = font.font_metrics();

        let mut gb = GlyphBuffer::new();
        let mut p = PointD {
            x: 20.0,
            y: 190.0 + fm.horizontal_ascent as f64,
        };
        let text = r#"Hello Blend2D!
        I'm a simple multiline text example
        that uses GlyphBuffer and fillGlyphRun!"#;

        for line in text.lines() {
            gb.set_utf8_text(line)?;
            font.shape(&mut gb)?;
            let tm = font.get_text_metrics(&mut gb)?;
            p.x = (480.0 - (tm.bounding_box.x1 as f64 - tm.bounding_box.x0 as f64)) / 2.0;
            ctx.fill_glyph_run(p, &font, gb.glyph_run())?;
            p.y += (fm.horizontal_ascent + fm.horizontal_descent + fm.line_gap) as f64;
        }
        ctx.end()
    };
    render(ctx).expect("Rendering to context failed");

    let codecs = ImageCodec::built_in_codecs();
    img.write_to_file(
        "bl-getting-started-8.bmp",
        codecs.find_codec_by_name("BMP").unwrap(),
    )
    .expect("Writing to file failed");
}
