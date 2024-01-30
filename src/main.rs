use blurhash::{decode, encode};
use image::{open, GenericImageView};
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("/home/remon/Downloads/octocat.png");
    // let path = PathBuf::from("/home/remon/Pictures/pxfuel.jpg");
    // let path = PathBuf::from("/home/remon/Pictures/10405.Longest Common Subsequence.png");
    let img = open(path).unwrap();
    let (width, height) = img.dimensions();
    let b = img.to_rgba8().into_raw();
    let blurhash = encode(4, 3, width, height, &b);

    match blurhash {
        Ok(blurhash) => {
            println!("{}", blurhash);
            let pixels = decode(&blurhash, 300, 300, 1.0);
            match pixels {
                Ok(pixels) => match image::RgbaImage::from_vec(300, 300, pixels) {
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
