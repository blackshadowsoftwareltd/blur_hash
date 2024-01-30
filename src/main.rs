use blurhash::{decode, encode};
use image::{open, GenericImageView};
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("/home/remon/Pictures/pxfuel.jpg");
    let img = open(path).unwrap();
    let (width, height) = img.dimensions();

    let b = img.to_rgba8().into_raw();
    let blurhash = encode(9, 9, width, height, &b);

    match blurhash {
        Ok(blurhash) => {
            println!("{}", blurhash);
            let (width, height) = dimensions(width, height);
            println!("{}x{}", width, height);
            let pixels = decode(&blurhash, width, height, 1.0);
            match pixels {
                Ok(pixels) => match image::RgbaImage::from_vec(width, height, pixels) {
                    Some(img) => {
                        img.save("test.png").unwrap();
                    }
                    None => {
                        println!("None");
                    }
                },
                Err(err) => println!("{}", err),
            }
        }
        Err(err) => println!("{}", err),
    }
}

fn dimensions(w: u32, h: u32) -> (u32, u32) {
    let pixels = 50; // ? 50 pixels

    // ? ">" ? pixels will be min and max will be calculated
    // ? "<" ? pixels will be max and min will be calculated
    let result = if w > h {
        let ratio = (w as f32) / (h as f32);
        ((pixels as f32 * ratio) as u32, pixels)
    } else {
        let ratio = (h as f32) / (w as f32);
        (pixels, (pixels as f32 * ratio) as u32)
    };
    result
}
