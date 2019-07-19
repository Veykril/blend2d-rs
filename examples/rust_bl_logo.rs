use blend2d::{
    geometry::SizeI,
    gradient::{Gradient, GradientStop, LinearGradientValues, RadialGradientValues},
    image::ImageScaleFilter,
    matrix::Matrix2D,
    pattern::Pattern,
    prelude::*,
    ExtendMode,
};

fn main() {
    let c_r = 160.0;
    let c_x = 180.0;
    let c_y = 180.0;

    let mut img = Image::new(480, 480, ImageFormat::PRgb32).expect("Unable to create image");
    let ctx = Context::new(&mut img).expect("Unable to attach rendering context");
    let render = |mut ctx: Context| {
        // Clear the image.
        ctx.set_comp_op(CompOp::SrcCopy);
        ctx.fill_all()?;

        // Draw a circle with a red-white radial gradient.
        let radial = Gradient::new_radial(
            &RadialGradientValues {
                x0: c_x,
                y0: c_y,
                x1: c_x,
                y1: c_y,
                r0: c_r,
            },
            ExtendMode::PadXPadY,
            &[
                GradientStop {
                    offset: 0.0,
                    rgba: 0xFFFFFFFFFFFFFFFF,
                },
                GradientStop {
                    offset: 1.0,
                    rgba: 0xFFFFFFFF6F6F3F3F,
                },
            ],
            None,
        );
        ctx.set_comp_op(CompOp::SrcOver);
        ctx.set_fill_style_gradient(&radial);
        ctx.fill_circle(c_x, c_y, c_r)?;

        // Multiply a circle with our logo scaled to the radius of the circle on top of
        // your image.
        let mut logo = Image::from_path(
            "assets/rust-logo-512x512-blk.png",
            &ImageCodec::built_in_codecs(),
        )?;
        logo.scale(
            SizeI {
                w: 2 * c_r as i32,
                h: 2 * c_r as i32,
            },
            ImageScaleFilter::Bell,
        )?;
        let pattern = Pattern::new(
            &logo,
            None,
            Default::default(),
            &Matrix2D::translation(20.0, 20.0),
        );
        ctx.set_comp_op(CompOp::Multiply);
        ctx.set_fill_style_pattern(&pattern);
        ctx.fill_circle(c_x, c_y, c_r)?;

        // Draw the difference of a square with a blue-white linear gradient to the
        // image with regards to the image.
        let linear = Gradient::new_linear(
            &LinearGradientValues {
                x0: 195.0,
                y0: 195.0,
                x1: 470.0,
                y1: 470.0,
            },
            ExtendMode::PadXPadY,
            &[
                GradientStop {
                    offset: 0.0,
                    rgba: 0xFFFFFFFFFFFFFFFF,
                },
                GradientStop {
                    offset: 1.0,
                    rgba: 0xFFFF3F3F9F9FFFFF,
                },
            ],
            None,
        );
        ctx.set_comp_op(CompOp::Difference);
        ctx.set_fill_style_gradient(&linear);
        ctx.fill_round_rect(195.0, 195.0, 270.0, 270.0, 25.0, 25.0)?;
        ctx.end()
    };
    render(ctx).expect("Rendering to context failed");

    img.write_to_file(
        "rust_bl_logo.bmp",
        ImageCodec::built_in_codecs()
            .find_codec_by_name("BMP")
            .unwrap(),
    )
    .expect("Writing to file failed");
}
