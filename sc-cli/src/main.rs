extern crate image;

use seam_carving::{resize_once, BarePixel};

use std::convert::TryFrom;
use std::env;

fn to_u32(n: usize) -> u32 {
    match u32::try_from(n) {
        Ok(m) => m,
        Err(why) => panic!("Bad usize u32: {:?}", why),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // get path to the image
    let path = args.get(1).unwrap();

    let rgba_img = match image::open(path) {
        Err(why) => panic!("No image: {:?}", why),
        Ok(data) => data.into_rgba8(),
    };

    // get image dimensions
    let (raw_width, raw_height) = rgba_img.dimensions();

    let width = raw_width as usize;
    let height = raw_height as usize;

    // get resizing target
    let target = match args.get(2).unwrap().parse::<usize>() {
        Ok(n) if n > width => panic!("Expanding Images not yet supported"),
        Ok(n) => n,
        Err(why) => panic!("Missing target!: {:?}", why),
    };

    let mut buffer: Vec<Vec<BarePixel>> = vec![];
    for row in rgba_img.rows() {
        let mut row_buffer = vec![];
        for cell in row {
            let r = cell[0];
            let g = cell[1];
            let b = cell[2];
            let a = cell[3];
            let buff = [r, g, b, a];
            let pixel = BarePixel::new(buff);
            row_buffer.push(pixel);
        }
        buffer.push(row_buffer);
    }

    for d in 0..(width - target) {
        let next_width = width - d;
        buffer = resize_once(next_width, height, &buffer);
    }

    let mut save_buff = image::ImageBuffer::new(to_u32(target), to_u32(height));

    for y in 0..height {
        for x in 0..target {
            let pixel = buffer[y][x];
            let rgba8 = pixel.extract();

            save_buff.put_pixel(to_u32(x), to_u32(y), image::Rgba(rgba8));
        }
    }

    match save_buff.save("./sc-cli/carved2.png") {
        Err(why) => panic!("Failed to save: {:?}", why),
        Ok(_) => println!("Saved!"),
    };
}
