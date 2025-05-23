use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn confuse(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let img_w = images[0].width().min(500);
        let image = images[0].resize_width(img_w);
        let mut surface = new_surface(image.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&image, (0, 0), None);
        let mask = load_image(format!("confuse/{i:02}.png"))?;
        let mask = mask.resize_fit(image.dimensions(), Fit::Cover);
        canvas.draw_image(&mask, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 100,
            duration: 0.02,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "confuse",
    confuse,
    min_images = 1,
    max_images = 1,
    keywords = &["迷惑"],
    date_created = local_date(2022, 9, 4),
    date_modified = local_date(2023, 2, 14),
);
