mod process;

use crate::process::watermark_channel;
use image::{
    imageops, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Luma, Pixel, Rgba,
};

pub fn watermark(
    image: DynamicImage,
    embedding_depth: f64,
    features_quantization_step: f64,
    features_amount: usize,
) -> DynamicImage {
    let color_type = image.color();
    let (width, height) = image.dimensions();

    let mut red_channel: ImageBuffer<Luma<u8>, Vec<<Luma<u8> as Pixel>::Subpixel>> =
        ImageBuffer::new(width, height);
    let mut green_channel: ImageBuffer<Luma<u8>, Vec<<Luma<u8> as Pixel>::Subpixel>> =
        ImageBuffer::new(width, height);
    let mut blue_channel: ImageBuffer<Luma<u8>, Vec<<Luma<u8> as Pixel>::Subpixel>> =
        ImageBuffer::new(width, height);
    let mut alpha_channel: ImageBuffer<Luma<u8>, Vec<<Luma<u8> as Pixel>::Subpixel>> =
        ImageBuffer::new(width, height);
    let mut out: ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>> =
        ImageBuffer::new(width, height);
    let temp: ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>> =
        ImageBuffer::new(width, height);

    for (x, y, _pixel) in temp.enumerate_pixels() {
        let rgba = image.get_pixel(x, y);

        #[allow(deprecated)]
        let (k1, k2, k3, k4) = rgba.channels4();

        red_channel.put_pixel(x, y, Luma([k1]));
        if color_type.has_color() {
            green_channel.put_pixel(x, y, Luma([k2]));
            blue_channel.put_pixel(x, y, Luma([k3]));
        }
        if color_type.has_alpha() {
            alpha_channel.put_pixel(x, y, Luma([k4]));
        }
    }

    red_channel = watermark_channel(
        &red_channel,
        embedding_depth,
        features_quantization_step,
        features_amount,
    );
    if color_type.has_color() {
        green_channel = watermark_channel(
            &green_channel,
            embedding_depth,
            features_quantization_step,
            features_amount,
        );
        blue_channel = watermark_channel(
            &blue_channel,
            embedding_depth,
            features_quantization_step,
            features_amount,
        );
    }
    //if color_type.has_alpha() {
    //    alpha_channel = process_channel(alpha_channel);
    //}

    for (x, y, pixel) in out.enumerate_pixels_mut() {
        let red = red_channel.get_pixel(x, y);
        let green;
        let blue;
        let alpha;

        if color_type.has_color() {
            green = green_channel.get_pixel(x, y);
            blue = blue_channel.get_pixel(x, y);
        } else {
            green = red;
            blue = red;
        }

        if color_type.has_alpha() {
            alpha = alpha_channel.get_pixel(x, y);
        } else {
            alpha = red;
        }

        *pixel = Rgba([red[0], green[0], blue[0], alpha[0]]);
    }

    return if !color_type.has_color() && color_type.has_alpha() {
        DynamicImage::from(imageops::grayscale_alpha(&out))
    } else if color_type.has_color() && color_type.has_alpha() {
        DynamicImage::from(out)
    } else if !color_type.has_color() {
        DynamicImage::from(imageops::grayscale(&out))
    } else {
        let mut rgb_image = DynamicImage::new_rgb8(width, height);
        rgb_image.copy_from(&out, 0, 0).unwrap();
        DynamicImage::from(rgb_image)
    };
}
