extern crate console_error_panic_hook;

mod image_info;

use image::io::Reader as ImageReader;
use image::DynamicImage;
use js_sys::Uint8Array;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

use crate::image_info::ImageInfo;
use watermarking;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn watermark_image(uint8array: Uint8Array) -> ImageInfo {
    console_log!("open_image_from_array_buffer {:?}", uint8array);

    let raw_data: Vec<u8> = uint8array.to_vec();
    return watermark(raw_data);
}

pub fn watermark(raw_data: Vec<u8>) -> ImageInfo {
    let data = Cursor::new(raw_data);
    let reader = ImageReader::new(data).with_guessed_format().unwrap();

    let format = reader.format().unwrap();
    let image_format = format!("{:?}", format);

    let image = reader.decode().unwrap();
    let watermarked = watermarking::watermark(DynamicImage::from(image));

    return ImageInfo {
        width: watermarked.width(),
        height: watermarked.height(),
        format: image_format,
    };
}

#[test]
fn watermark_test_jpeg() {
    let jpeg_image_base64 = "/9j/4AAQSkZJRgABAQEBLAEsAAD/4QkiRXhpZgAASUkqAAgAAAAIAA4BAgASAAAAbgAAABIBAwABAAAAAQAAABoBBQABAAAAgAAAABsBBQABAAAAiAAAACgBAwABAAAAAgAAADEBAgANAAAAkAAAADIBAgAUAAAAngAAAGmHBAABAAAAsgAAAOoAAABDcmVhdGVkIHdpdGggR0lNUAAsAQAAAQAAACwBAAABAAAAR0lNUCAyLjEwLjMyAAAyMDIyOjA3OjE4IDIzOjQ4OjE3AAIAhpIHABkAAADQAAAAAaADAAEAAAABAAAAAAAAAAAAAAAAAAAAQ3JlYXRlZCB3aXRoIEdJTVAACQD+AAQAAQAAAAEAAAAAAQQAAQAAAAABAAABAQQAAQAAAAABAAACAQMAAwAAAFwBAAADAQMAAQAAAAYAAAAGAQMAAQAAAAYAAAAVAQMAAQAAAAMAAAABAgQAAQAAAGIBAAACAgQAAQAAALcHAAAAAAAACAAIAAgA/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAgGBgcGBQgHBwcJCQgKDBQNDAsLDBkSEw8UHRofHh0aHBwgJC4nICIsIxwcKDcpLDAxNDQ0Hyc5PTgyPC4zNDL/2wBDAQkJCQwLDBgNDRgyIRwhMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjL/wAARCAEAAQADASIAAhEBAxEB/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/9oADAMBAAIRAxEAPwD5/ooooAKKKKACiiigAooooA6SiiivPPrwooooA2aKKK4z6cKKKKACiiigAooooAxqKKK7D5gKKKKAObooor0D5AKKKKACiiigAooooAKKKKACiiigAooooAKKKKAOkooorzz68KKKKANmiiiuM+nCiiigAooooAKKKKAMaiiiuw+YCiiigDm6KKK9A+QCiiigAooooAKKKKACiiigAooooA3aKKK4j6YKKKKAOuoooryD9FCiiigBaKKKk2CiiigAooooAKKKKAEoooqjEKKKKAORooor1z86CiiigDCooortPmQooooAKKKKACiiigDdoooriPpgooooA66iiivIP0UKKKKAFoooqTYKKKKACiiigAooooASiiiqMQooooA5GiiivXPzoKKKKAMKiiiu0+ZCiiigAooooAKKKKAPQaKKK9k/QQooooA6GiiivTPogooooA2qKKK8k8wKKKKAO5ooor8oPmwooooAkooorxiAooooA89ooor6I5gooooA8Iooor7g+fCiiigAooooAKKKKAPQaKKK9k/QQooooA6GiiivTPogooooA2qKKK8k8wKKKKAO5ooor8oPmwooooAkooorxiAooooA89ooor6I5gooooA8Iooor7g+fCiiigDQooor7c5gooooA7qiiivfPuQooooA6SiiiuA+qCiiigD3qiiivgT8xCiiigDRooorxjmCiiigDIooor8iO8KKKKAOdooor0T5MKKKKAPDaKKK+5PNCiiigDgaKKK/Wj6MKKKKAO6ooor3z7kKKKKAOkooorgPqgooooA96ooor4E/MQooooA0aKKK8Y5gooooAyKKKK/IjvCiiigDnaKKK9E+TCiiigDw2iiivuTzQooooA4qiiiv3w90KKKKALlFFFembhRRRQB7vRRRXwJ9SFFFFAHsdFFFfInzwUUUUAFFFFABRRRQBz9FFFfEHMFFFFAHnlFFFeSfmgUUUUAc7RRRXpnWFFFFAHj1FFFf0mfdBRRRQBcooor0zcKKKKAPd6KKK+BPqQooooA9jooor5E+eCiiigAooooAKKKKAOfooor4g5gooooA88oooryT80CiiigDnaKKK9M6wooooA8dooor+jz7sKKKKAJaKKK9wyCiiigDvKKKK+ePngooooA94ooor4Q4gooooASiiigAooooA5Ciiivij5wKKKKAKlFFFfKGoUUUUAc9RRRXpnrhRRRQB5FRRRX9Gn3QUUUUAS0UUV7hkFFFFAHeUUUV88fPBRRRQB7xRRRXwhxBRRRQAlFFFABRRRQByFFFFfFHzgUUUUAVKKKK+UNQooooA56iiivTPXCiiigDxeiiiv2I/QQooooAv0UUV96coUUUUAOooorpJCiiigD3Oiiivgz87CiiigDbooor5c8sKKKKAMmiiivyo7AooooAgooorQ6AooooA8rooor6s+1CiiigDkaKKK/RD2wooooAv0UUV96coUUUUAOooorpJCiiigD3Oiiivgz87CiiigDbooor5c8sKKKKAMmiiivyo7AooooAgooorQ6AooooA8rooor6s+1CiiigDhKKKK9o+iCiiigCxRRRXtGYUUUUAWqKKK+mMAooooASiiiuMYUUUUAdzRRRX5SfOhRRRQBeooorA5gooooA46iiivYPoAooooA5iiiivRPVCiiigAooooAKKKKALFFFFe0ZhRRRQBaooor6YwCiiigBKKKK4xhRRRQB3NFFFflJ86FFFFAF6iiisDmCiiigDjqKKK9g+gCiiigDmKKKK9E9UKKKKAP/2QD/4Q2SaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wLwA8P3hwYWNrZXQgYmVnaW49Iu+7vyIgaWQ9Ilc1TTBNcENlaGlIenJlU3pOVGN6a2M5ZCI/PiA8eDp4bXBtZXRhIHhtbG5zOng9ImFkb2JlOm5zOm1ldGEvIiB4OnhtcHRrPSJYTVAgQ29yZSA0LjQuMC1FeGl2MiI+IDxyZGY6UkRGIHhtbG5zOnJkZj0iaHR0cDovL3d3dy53My5vcmcvMTk5OS8wMi8yMi1yZGYtc3ludGF4LW5zIyI+IDxyZGY6RGVzY3JpcHRpb24gcmRmOmFib3V0PSIiIHhtbG5zOnhtcE1NPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvbW0vIiB4bWxuczpzdEV2dD0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL3NUeXBlL1Jlc291cmNlRXZlbnQjIiB4bWxuczpkYz0iaHR0cDovL3B1cmwub3JnL2RjL2VsZW1lbnRzLzEuMS8iIHhtbG5zOkdJTVA9Imh0dHA6Ly93d3cuZ2ltcC5vcmcveG1wLyIgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIiB4bXBNTTpEb2N1bWVudElEPSJnaW1wOmRvY2lkOmdpbXA6OGMxMzNkZmYtMWY2ZS00MWMzLWI0MDEtZjQ5ZTJiMjk3NzllIiB4bXBNTTpJbnN0YW5jZUlEPSJ4bXAuaWlkOmYzM2Q4OTc0LTY0ZjAtNGYzZC04YTg3LTEyZTQxZWMxYTA1NyIgeG1wTU06T3JpZ2luYWxEb2N1bWVudElEPSJ4bXAuZGlkOmIzYmZjNGQxLTRkNTEtNGQxNS05YjgyLTg5ZGQ3OTQxOWI2NCIgZGM6Rm9ybWF0PSJpbWFnZS9qcGVnIiBHSU1QOkFQST0iMi4wIiBHSU1QOlBsYXRmb3JtPSJNYWMgT1MiIEdJTVA6VGltZVN0YW1wPSIxNjU4MTc3MzAwMjY1NTI2IiBHSU1QOlZlcnNpb249IjIuMTAuMzIiIHhtcDpDcmVhdG9yVG9vbD0iR0lNUCAyLjEwIiB4bXA6TWV0YWRhdGFEYXRlPSIyMDIyOjA3OjE4VDIzOjQ4OjE3KzAzOjAwIiB4bXA6TW9kaWZ5RGF0ZT0iMjAyMjowNzoxOFQyMzo0ODoxNyswMzowMCI+IDx4bXBNTTpIaXN0b3J5PiA8cmRmOlNlcT4gPHJkZjpsaSBzdEV2dDphY3Rpb249InNhdmVkIiBzdEV2dDpjaGFuZ2VkPSIvIiBzdEV2dDppbnN0YW5jZUlEPSJ4bXAuaWlkOjE0OTI3ZDNhLWE0YmMtNDMzNi04NTQxLWMyZmE2OTUyYzA1OCIgc3RFdnQ6c29mdHdhcmVBZ2VudD0iR2ltcCAyLjEwIChNYWMgT1MpIiBzdEV2dDp3aGVuPSIyMDIyLTA3LTE2VDEyOjI2OjEzKzAzOjAwIi8+IDxyZGY6bGkgc3RFdnQ6YWN0aW9uPSJzYXZlZCIgc3RFdnQ6Y2hhbmdlZD0iLyIgc3RFdnQ6aW5zdGFuY2VJRD0ieG1wLmlpZDo2MjcyN2M3NS1lMjk0LTRkYjUtYTM5Ni1lMzVmNDg5NTZlMzEiIHN0RXZ0OnNvZnR3YXJlQWdlbnQ9IkdpbXAgMi4xMCAoTWFjIE9TKSIgc3RFdnQ6d2hlbj0iMjAyMi0wNy0xOFQyMzo0ODoyMCswMzowMCIvPiA8L3JkZjpTZXE+IDwveG1wTU06SGlzdG9yeT4gPC9yZGY6RGVzY3JpcHRpb24+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+ICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgPD94cGFja2V0IGVuZD0idyI/Pv/iArBJQ0NfUFJPRklMRQABAQAAAqBsY21zBDAAAG1udHJSR0IgWFlaIAfmAAcAEgAUAC8ANmFjc3BBUFBMAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD21gABAAAAANMtbGNtcwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADWRlc2MAAAEgAAAAQGNwcnQAAAFgAAAANnd0cHQAAAGYAAAAFGNoYWQAAAGsAAAALHJYWVoAAAHYAAAAFGJYWVoAAAHsAAAAFGdYWVoAAAIAAAAAFHJUUkMAAAIUAAAAIGdUUkMAAAIUAAAAIGJUUkMAAAIUAAAAIGNocm0AAAI0AAAAJGRtbmQAAAJYAAAAJGRtZGQAAAJ8AAAAJG1sdWMAAAAAAAAAAQAAAAxlblVTAAAAJAAAABwARwBJAE0AUAAgAGIAdQBpAGwAdAAtAGkAbgAgAHMAUgBHAEJtbHVjAAAAAAAAAAEAAAAMZW5VUwAAABoAAAAcAFAAdQBiAGwAaQBjACAARABvAG0AYQBpAG4AAFhZWiAAAAAAAAD21gABAAAAANMtc2YzMgAAAAAAAQxCAAAF3v//8yUAAAeTAAD9kP//+6H///2iAAAD3AAAwG5YWVogAAAAAAAAb6AAADj1AAADkFhZWiAAAAAAAAAknwAAD4QAALbEWFlaIAAAAAAAAGKXAAC3hwAAGNlwYXJhAAAAAAADAAAAAmZmAADypwAADVkAABPQAAAKW2Nocm0AAAAAAAMAAAAAo9cAAFR8AABMzQAAmZoAACZnAAAPXG1sdWMAAAAAAAAAAQAAAAxlblVTAAAACAAAABwARwBJAE0AUG1sdWMAAAAAAAAAAQAAAAxlblVTAAAACAAAABwAcwBSAEcAQv/bAEMAAwICAwICAwMDAwQDAwQFCAUFBAQFCgcHBggMCgwMCwoLCw0OEhANDhEOCwsQFhARExQVFRUMDxcYFhQYEhQVFP/bAEMBAwQEBQQFCQUFCRQNCw0UFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFP/+ABRDcmVhdGVkIHdpdGggR0lNUAD/wgARCAAIAAgDAREAAhEBAxEB/8QAFAABAAAAAAAAAAAAAAAAAAAABv/EABQBAQAAAAAAAAAAAAAAAAAAAAD/2gAMAwEAAhADEAAAARZ//8QAFhABAQEAAAAAAAAAAAAAAAAABAUG/9oACAEBAAEFAo5IKsl//8QAIBEAAQIFBQAAAAAAAAAAAAAAAQMEBREhAAIGMVKR8P/aAAgBAwEBPwGIarVQaYQxBtUjjQme8/dX/8QAHxEAAAUEAwAAAAAAAAAAAAAAEgMEBREAAQIGUWFx/9oACAECAQE/AdpQ2bjE5zdmIUdi58iv/8QAHRAAAQIHAAAAAAAAAAAAAAAAAxIAAgQRFCEyYv/aAAgBAQAGPwI9wRM5Djaien//xAAaEAABBQEAAAAAAAAAAAAAAAAxAAERIWHR/9oACAEBAAE/ISQAAJbV8X//2gAMAwEAAgADAAAAEB//xAAaEQEAAgMBAAAAAAAAAAAAAAABETFBACGB/9oACAEDAQE/EGGICjZZyDHGfW//xAAZEQEBAQADAAAAAAAAAAAAAAABESExQWH/2gAIAQIBAT8QjGLtZRlDy4ku6k//xAAZEAEAAwEBAAAAAAAAAAAAAAABESFBAFH/2gAIAQEAAT8QwKmnIBC3ZY2eJ7//2Q==";
    let image_array = base64::decode(jpeg_image_base64).unwrap();

    let expected = ImageInfo {
        width: 8,
        height: 8,
        format: String::from("Jpeg"),
    };

    assert_eq!(expected, watermark(image_array));
}

#[test]
fn watermark_test_png() {
    let png_image_base64 = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABmJLR0QA/wD/AP+gvaeTAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAB3RJTUUH4gEdFQog0ycfAgAAAIJJREFUOMulU0EOwCAIK2T/f/LYwWAAgZGtJzS1BbVEuEVAAACCQOsKlkOrEicwgeVz5tC5R1yrDdnKuo6j6J5ydgd+npOUHfaGEJkQq+6cQNVqP1oQiCJxvAjGT3Dn3l1sKpAdfhPhqXP5xDYLXz7SkYUuUNnrcBWULkRlFqZxtvwH8zGCEN6LErUAAAAASUVORK5CYII=";
    let image_array = base64::decode(png_image_base64).unwrap();

    let expected = ImageInfo {
        width: 16,
        height: 16,
        format: String::from("Png"),
    };

    assert_eq!(expected, watermark(image_array));
}

#[test]
fn watermark_test_gif() {
    let gif_image_base64 = "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";
    let image_array = base64::decode(gif_image_base64).unwrap();

    let expected = ImageInfo {
        width: 128,
        height: 128,
        format: String::from("Gif"),
    };

    assert_eq!(expected, watermark(image_array));
}

#[test]
fn watermark_test_bmp() {
    let gif_image_base64 = "Qk1KAQAAAAAAAIoAAAB8AAAACAAAAAgAAAABABgAAAAAAMAAAAAjLgAAIy4AAAAAAAAAAAAAAAD/AAD/AAD/AAAAAAAAAEJHUnMAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAMCQwMDlQgFjgATCYMZAJcAC0cBBQAAYQAA9xoM9QAa/7kLtf8ABv8UHOoVAFkTxBoP+QAA6ncF8f908/97b+YIAv8AHcgC2RwH9AAD/4LK+/r///fwgfzQAP8AHdAPcAgA/w6a/wD6/9P51//2C///C/SYEIECAQUGaA2wwAD9nS39HaH/BML3J3anAAEADAEAFgBoARnnBAD9CQD9BwD0AABaAyIBAwAYGAIAJAhsHRO6ASPQESVsBhwAAAAZ";
    let image_array = base64::decode(gif_image_base64).unwrap();

    let expected = ImageInfo {
        width: 8,
        height: 8,
        format: String::from("Bmp"),
    };

    assert_eq!(expected, watermark(image_array));
}

#[test]
fn watermark_test_tiff() {
    let tiff_image_base64 = "TU0AKgAAATIAA6ABAAMAAAABAAEAAKACAAQAAAABAAAACKADAAQAAAABAAAACAAAAAAYAAD/AAEY/20CJP/JDSL/ySIO/20lAv8AGAH/GAAA/wAAGP9aAAD/9BAA//8AAP//AAD/9AAR/1kAAP8AGAD/AAAA/60Rdf//AL7/9iSp//WsJf//vwD/rXQT/wAAAf8ABn3/mA76//8A///51///+//V////AP+Z/BH/AH4N/wUbz/8AAP//y3vz////+////P//zPh7/wD/AP8K0B3/DBjG/wAA//8Ab/X/dv/7/3b8/v8A+W7/AP8A/w7HGv8AA1v/Cxjq/xMA//8Av///AP+//xP/AP8Q7hr/A10J/wAAAP8BBzz/BACZ/xoAgf8bggD/CJoA/wU9Cv8BAAH/ABgBAAADAAAAAQAIAAABAQADAAAAAQAIAAABAgADAAAABAAAAmgBAwADAAAAAQABAAABBgADAAAAAQACAAABCgADAAAAAQABAAABDgACAAAAEgAAAngBEQAEAAAAAQAAADIBEgADAAAAAQABAAABFQADAAAAAQAEAAABFgADAAAAAQAIAAABFwAEAAAAAQAAAQABGgAFAAAAAQAAAlgBGwAFAAAAAQAAAmABHAADAAAAAQABAAABKAADAAAAAQACAAABMQACAAAADQAAAooBMgACAAAAFAAAApgBUgADAAAAAQABAAABUwADAAAABAAAAnACvAABAAAHgQAAAtKDuwAHAAAAJQAAAqyHaQAEAAAAAQAAAAiHcwAHAAACoAAAClQAAAAAAAABLAAAAAEAAAEsAAAAAQAIAAgACAAIAAEAAQABAAFDcmVhdGVkIHdpdGggR0lNUABHSU1QIDIuMTAuMzIAADIwMjI6MDc6MTYgMTI6MjY6MTMAHAFaAAMbJUccAgAAAgACHAJ4ABFDcmVhdGVkIHdpdGggR0lNUAA8eDp4bXBtZXRhIHhtbG5zOng9ImFkb2JlOm5zOm1ldGEvIiB4OnhtcHRrPSJYTVAgQ29yZSA2LjAuMCI+CiAgIDxyZGY6UkRGIHhtbG5zOnJkZj0iaHR0cDovL3d3dy53My5vcmcvMTk5OS8wMi8yMi1yZGYtc3ludGF4LW5zIyI+CiAgICAgIDxyZGY6RGVzY3JpcHRpb24gcmRmOmFib3V0PSIiCiAgICAgICAgICAgIHhtbG5zOkdJTVA9Imh0dHA6Ly93d3cuZ2ltcC5vcmcveG1wLyIKICAgICAgICAgICAgeG1sbnM6ZGM9Imh0dHA6Ly9wdXJsLm9yZy9kYy9lbGVtZW50cy8xLjEvIgogICAgICAgICAgICB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iCiAgICAgICAgICAgIHhtbG5zOnhtcE1NPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvbW0vIgogICAgICAgICAgICB4bWxuczpzdEV2dD0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL3NUeXBlL1Jlc291cmNlRXZlbnQjIj4KICAgICAgICAgPEdJTVA6QVBJPjIuMDwvR0lNUDpBUEk+CiAgICAgICAgIDxHSU1QOlZlcnNpb24+Mi4xMC4zMjwvR0lNUDpWZXJzaW9uPgogICAgICAgICA8R0lNUDpUaW1lU3RhbXA+MTY1Nzk2MzU3MzkzMzU1MjwvR0lNUDpUaW1lU3RhbXA+CiAgICAgICAgIDxHSU1QOlBsYXRmb3JtPk1hYyBPUzwvR0lNUDpQbGF0Zm9ybT4KICAgICAgICAgPGRjOkZvcm1hdD5pbWFnZS9qcGVnPC9kYzpGb3JtYXQ+CiAgICAgICAgIDxkYzpkZXNjcmlwdGlvbj4KICAgICAgICAgICAgPHJkZjpBbHQ+CiAgICAgICAgICAgICAgIDxyZGY6bGkgeG1sOmxhbmc9IngtZGVmYXVsdCI+Q3JlYXRlZCB3aXRoIEdJTVA8L3JkZjpsaT4KICAgICAgICAgICAgPC9yZGY6QWx0PgogICAgICAgICA8L2RjOmRlc2NyaXB0aW9uPgogICAgICAgICA8eG1wOkNyZWF0b3JUb29sPkdJTVAgMi4xMC4zMjwveG1wOkNyZWF0b3JUb29sPgogICAgICAgICA8eG1wOk1vZGlmeURhdGU+MjAyMi0wNy0xNlQxMjoyNjoxMzwveG1wOk1vZGlmeURhdGU+CiAgICAgICAgIDx4bXA6TWV0YWRhdGFEYXRlPjIwMjI6MDc6MTZUMTI6MjY6MTMrMDM6MDA8L3htcDpNZXRhZGF0YURhdGU+CiAgICAgICAgIDx4bXBNTTpPcmlnaW5hbERvY3VtZW50SUQ+eG1wLmRpZDpiM2JmYzRkMS00ZDUxLTRkMTUtOWI4Mi04OWRkNzk0MTliNjQ8L3htcE1NOk9yaWdpbmFsRG9jdW1lbnRJRD4KICAgICAgICAgPHhtcE1NOkhpc3Rvcnk+CiAgICAgICAgICAgIDxyZGY6U2VxPgogICAgICAgICAgICAgICA8cmRmOmxpIHJkZjpwYXJzZVR5cGU9IlJlc291cmNlIj4KICAgICAgICAgICAgICAgICAgPHN0RXZ0OmNoYW5nZWQ+Lzwvc3RFdnQ6Y2hhbmdlZD4KICAgICAgICAgICAgICAgICAgPHN0RXZ0OnNvZnR3YXJlQWdlbnQ+R2ltcCAyLjEwIChNYWMgT1MpPC9zdEV2dDpzb2Z0d2FyZUFnZW50PgogICAgICAgICAgICAgICAgICA8c3RFdnQ6d2hlbj4yMDIyLTA3LTE2VDEyOjI2OjEzKzAzOjAwPC9zdEV2dDp3aGVuPgogICAgICAgICAgICAgICAgICA8c3RFdnQ6aW5zdGFuY2VJRD54bXAuaWlkOjE0OTI3ZDNhLWE0YmMtNDMzNi04NTQxLWMyZmE2OTUyYzA1ODwvc3RFdnQ6aW5zdGFuY2VJRD4KICAgICAgICAgICAgICAgICAgPHN0RXZ0OmFjdGlvbj5zYXZlZDwvc3RFdnQ6YWN0aW9uPgogICAgICAgICAgICAgICA8L3JkZjpsaT4KICAgICAgICAgICAgPC9yZGY6U2VxPgogICAgICAgICA8L3htcE1NOkhpc3Rvcnk+CiAgICAgICAgIDx4bXBNTTpEb2N1bWVudElEPmdpbXA6ZG9jaWQ6Z2ltcDo4YzEzM2RmZi0xZjZlLTQxYzMtYjQwMS1mNDllMmIyOTc3OWU8L3htcE1NOkRvY3VtZW50SUQ+CiAgICAgICAgIDx4bXBNTTpJbnN0YW5jZUlEPnhtcC5paWQ6MmViYTk4MTYtMGQ0ZS00YWUwLThiNGEtNjNjN2ZkN2ViOWE3PC94bXBNTTpJbnN0YW5jZUlEPgogICAgICA8L3JkZjpEZXNjcmlwdGlvbj4KICAgPC9yZGY6UkRGPgo8L3g6eG1wbWV0YT4KAAAAAqBsY21zBDAAAG1udHJSR0IgWFlaIAfmAAcAEAAJABAAAmFjc3BBUFBMAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD21gABAAAAANMtbGNtcwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADWRlc2MAAAEgAAAAQGNwcnQAAAFgAAAANnd0cHQAAAGYAAAAFGNoYWQAAAGsAAAALHJYWVoAAAHYAAAAFGJYWVoAAAHsAAAAFGdYWVoAAAIAAAAAFHJUUkMAAAIUAAAAIGdUUkMAAAIUAAAAIGJUUkMAAAIUAAAAIGNocm0AAAI0AAAAJGRtbmQAAAJYAAAAJGRtZGQAAAJ8AAAAJG1sdWMAAAAAAAAAAQAAAAxlblVTAAAAJAAAABwARwBJAE0AUAAgAGIAdQBpAGwAdAAtAGkAbgAgAHMAUgBHAEJtbHVjAAAAAAAAAAEAAAAMZW5VUwAAABoAAAAcAFAAdQBiAGwAaQBjACAARABvAG0AYQBpAG4AAFhZWiAAAAAAAAD21gABAAAAANMtc2YzMgAAAAAAAQxCAAAF3v//8yUAAAeTAAD9kP//+6H///2iAAAD3AAAwG5YWVogAAAAAAAAb6AAADj1AAADkFhZWiAAAAAAAAAknwAAD4QAALbEWFlaIAAAAAAAAGKXAAC3hwAAGNlwYXJhAAAAAAADAAAAAmZmAADypwAADVkAABPQAAAKW2Nocm0AAAAAAAMAAAAAo9cAAFR8AABMzQAAmZoAACZnAAAPXG1sdWMAAAAAAAAAAQAAAAxlblVTAAAACAAAABwARwBJAE0AUG1sdWMAAAAAAAAAAQAAAAxlblVTAAAACAAAABwAcwBSAEcAQg==";
    let tiff_lzw_image_base64 = "TU0AKgAAASoAA6ABAAMAAAABAAEAAKACAAQAAAABAAAACKADAAQAAAABAAAACAAAAACABgAAB/ugAwI2gEGAAuAt/QMKuwAKQBvQAJN5v8ABh0RqBhh/loAOgAJoIQMFvCBysAPUABEAGUAO8AKeBSx/q0InUAFJ3kkAPcSOuCIg+AAFBNtgBXLUJgApox3QMDH1/pgEH0AGd5AUAPprwMBChrAACAAVyV+zAzoJ+AAChtnv99uUYABlnuLDRCAiBv2zs1+UcaAdC0hoh0AAwMMZ/vR0DmBm97AA7JADX8BgBFP04VRJAAHMgNQMBlt/gsKo8AAh0BUAO1fysgMAAU8ggB+u/SvM3zSVv8AgceAABvkugALSMAAFBH/Yzd+qMFAB+MN7gCAgAAAZAQAAAwAAAAEACAAAAQEAAwAAAAEACAAAAQIAAwAAAAQAAAJsAQMAAwAAAAEABQAAAQYAAwAAAAEAAgAAAQoAAwAAAAEAAQAAAQ4AAgAAABIAAAJ8AREABAAAAAEAAAAyARIAAwAAAAEAAQAAARUAAwAAAAEABAAAARYAAwAAAAEACAAAARcABAAAAAEAAAD3ARoABQAAAAEAAAJcARsABQAAAAEAAAJkARwAAwAAAAEAAQAAASgAAwAAAAEAAgAAATEAAgAAAA0AAAKOATIAAgAAABQAAAKcAT0AAwAAAAEAAgAAAVIAAwAAAAEAAQAAAVMAAwAAAAQAAAJ0ArwAAQAAB4EAAALWg7sABwAAACUAAAKwh2kABAAAAAEAAAAIh3MABwAAAqAAAApYAAAAAAAAASwAAAABAAABLAAAAAEACAAIAAgACAABAAEAAQABQ3JlYXRlZCB3aXRoIEdJTVAAR0lNUCAyLjEwLjMyAAAyMDIyOjA3OjE2IDEyOjI2OjEzABwBWgADGyVHHAIAAAIAAhwCeAARQ3JlYXRlZCB3aXRoIEdJTVAAPHg6eG1wbWV0YSB4bWxuczp4PSJhZG9iZTpuczptZXRhLyIgeDp4bXB0az0iWE1QIENvcmUgNi4wLjAiPgogICA8cmRmOlJERiB4bWxuczpyZGY9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkvMDIvMjItcmRmLXN5bnRheC1ucyMiPgogICAgICA8cmRmOkRlc2NyaXB0aW9uIHJkZjphYm91dD0iIgogICAgICAgICAgICB4bWxuczpHSU1QPSJodHRwOi8vd3d3LmdpbXAub3JnL3htcC8iCiAgICAgICAgICAgIHhtbG5zOmRjPSJodHRwOi8vcHVybC5vcmcvZGMvZWxlbWVudHMvMS4xLyIKICAgICAgICAgICAgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIgogICAgICAgICAgICB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIKICAgICAgICAgICAgeG1sbnM6c3RFdnQ9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZUV2ZW50IyI+CiAgICAgICAgIDxHSU1QOkFQST4yLjA8L0dJTVA6QVBJPgogICAgICAgICA8R0lNUDpWZXJzaW9uPjIuMTAuMzI8L0dJTVA6VmVyc2lvbj4KICAgICAgICAgPEdJTVA6VGltZVN0YW1wPjE2NTc5NjM1NzM5MzM1NTI8L0dJTVA6VGltZVN0YW1wPgogICAgICAgICA8R0lNUDpQbGF0Zm9ybT5NYWMgT1M8L0dJTVA6UGxhdGZvcm0+CiAgICAgICAgIDxkYzpGb3JtYXQ+aW1hZ2UvanBlZzwvZGM6Rm9ybWF0PgogICAgICAgICA8ZGM6ZGVzY3JpcHRpb24+CiAgICAgICAgICAgIDxyZGY6QWx0PgogICAgICAgICAgICAgICA8cmRmOmxpIHhtbDpsYW5nPSJ4LWRlZmF1bHQiPkNyZWF0ZWQgd2l0aCBHSU1QPC9yZGY6bGk+CiAgICAgICAgICAgIDwvcmRmOkFsdD4KICAgICAgICAgPC9kYzpkZXNjcmlwdGlvbj4KICAgICAgICAgPHhtcDpDcmVhdG9yVG9vbD5HSU1QIDIuMTAuMzI8L3htcDpDcmVhdG9yVG9vbD4KICAgICAgICAgPHhtcDpNb2RpZnlEYXRlPjIwMjItMDctMTZUMTI6MjY6MTM8L3htcDpNb2RpZnlEYXRlPgogICAgICAgICA8eG1wOk1ldGFkYXRhRGF0ZT4yMDIyOjA3OjE2VDEyOjI2OjEzKzAzOjAwPC94bXA6TWV0YWRhdGFEYXRlPgogICAgICAgICA8eG1wTU06T3JpZ2luYWxEb2N1bWVudElEPnhtcC5kaWQ6YjNiZmM0ZDEtNGQ1MS00ZDE1LTliODItODlkZDc5NDE5YjY0PC94bXBNTTpPcmlnaW5hbERvY3VtZW50SUQ+CiAgICAgICAgIDx4bXBNTTpIaXN0b3J5PgogICAgICAgICAgICA8cmRmOlNlcT4KICAgICAgICAgICAgICAgPHJkZjpsaSByZGY6cGFyc2VUeXBlPSJSZXNvdXJjZSI+CiAgICAgICAgICAgICAgICAgIDxzdEV2dDpjaGFuZ2VkPi88L3N0RXZ0OmNoYW5nZWQ+CiAgICAgICAgICAgICAgICAgIDxzdEV2dDpzb2Z0d2FyZUFnZW50PkdpbXAgMi4xMCAoTWFjIE9TKTwvc3RFdnQ6c29mdHdhcmVBZ2VudD4KICAgICAgICAgICAgICAgICAgPHN0RXZ0OndoZW4+MjAyMi0wNy0xNlQxMjoyNjoxMyswMzowMDwvc3RFdnQ6d2hlbj4KICAgICAgICAgICAgICAgICAgPHN0RXZ0Omluc3RhbmNlSUQ+eG1wLmlpZDoxNDkyN2QzYS1hNGJjLTQzMzYtODU0MS1jMmZhNjk1MmMwNTg8L3N0RXZ0Omluc3RhbmNlSUQ+CiAgICAgICAgICAgICAgICAgIDxzdEV2dDphY3Rpb24+c2F2ZWQ8L3N0RXZ0OmFjdGlvbj4KICAgICAgICAgICAgICAgPC9yZGY6bGk+CiAgICAgICAgICAgIDwvcmRmOlNlcT4KICAgICAgICAgPC94bXBNTTpIaXN0b3J5PgogICAgICAgICA8eG1wTU06RG9jdW1lbnRJRD5naW1wOmRvY2lkOmdpbXA6OGMxMzNkZmYtMWY2ZS00MWMzLWI0MDEtZjQ5ZTJiMjk3NzllPC94bXBNTTpEb2N1bWVudElEPgogICAgICAgICA8eG1wTU06SW5zdGFuY2VJRD54bXAuaWlkOjJlYmE5ODE2LTBkNGUtNGFlMC04YjRhLTYzYzdmZDdlYjlhNzwveG1wTU06SW5zdGFuY2VJRD4KICAgICAgPC9yZGY6RGVzY3JpcHRpb24+CiAgIDwvcmRmOlJERj4KPC94OnhtcG1ldGE+CgAAAAKgbGNtcwQwAABtbnRyUkdCIFhZWiAH5gAHABAACQAQAAJhY3NwQVBQTAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA9tYAAQAAAADTLWxjbXMAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA1kZXNjAAABIAAAAEBjcHJ0AAABYAAAADZ3dHB0AAABmAAAABRjaGFkAAABrAAAACxyWFlaAAAB2AAAABRiWFlaAAAB7AAAABRnWFlaAAACAAAAABRyVFJDAAACFAAAACBnVFJDAAACFAAAACBiVFJDAAACFAAAACBjaHJtAAACNAAAACRkbW5kAAACWAAAACRkbWRkAAACfAAAACRtbHVjAAAAAAAAAAEAAAAMZW5VUwAAACQAAAAcAEcASQBNAFAAIABiAHUAaQBsAHQALQBpAG4AIABzAFIARwBCbWx1YwAAAAAAAAABAAAADGVuVVMAAAAaAAAAHABQAHUAYgBsAGkAYwAgAEQAbwBtAGEAaQBuAABYWVogAAAAAAAA9tYAAQAAAADTLXNmMzIAAAAAAAEMQgAABd7///MlAAAHkwAA/ZD///uh///9ogAAA9wAAMBuWFlaIAAAAAAAAG+gAAA49QAAA5BYWVogAAAAAAAAJJ8AAA+EAAC2xFhZWiAAAAAAAABilwAAt4cAABjZcGFyYQAAAAAAAwAAAAJmZgAA8qcAAA1ZAAAT0AAACltjaHJtAAAAAAADAAAAAKPXAABUfAAATM0AAJmaAAAmZwAAD1xtbHVjAAAAAAAAAAEAAAAMZW5VUwAAAAgAAAAcAEcASQBNAFBtbHVjAAAAAAAAAAEAAAAMZW5VUwAAAAgAAAAcAHMAUgBHAEI=";
    let image_array = base64::decode(tiff_image_base64).unwrap();
    let image_array_lzw = base64::decode(tiff_lzw_image_base64).unwrap();

    let expected = ImageInfo {
        width: 8,
        height: 8,
        format: String::from("Tiff"),
    };

    assert_eq!(expected, watermark(image_array));
    assert_eq!(expected, watermark(image_array_lzw));
}
