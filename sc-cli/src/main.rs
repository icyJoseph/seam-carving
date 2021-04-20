extern crate image;

use seam_carving::{calc_pixel_energy, find_low_energy_seam, BarePixel};

use std::convert::TryFrom;
use std::env;

fn to_u32(n: usize) -> u32 {
    match u32::try_from(n) {
        Ok(m) => m,
        Err(why) => panic!("Bad usize u32: {:?}", why),
    }
}

fn resize(
    width: usize,
    height: usize,
    data: &Vec<Vec<BarePixel>>,
    target: usize,
) -> Vec<Vec<BarePixel>> {
    let mut energies = vec![vec![625; width]; height];

    for y in 0..height {
        for x in 0..width {
            energies[y][x] = calc_pixel_energy(x, y, &data);
        }
    }

    let seam = find_low_energy_seam(&energies, width, height);

    let mut buffer: Vec<Vec<BarePixel>> = vec![];

    for (s_x, s_y) in &seam {
        let mut row_buffer: Vec<BarePixel> = vec![];

        match data.iter().nth(*s_y as usize) {
            None => continue,
            Some(row) => {
                let mut index = 0;
                for cell in row.iter() {
                    if index != *s_x {
                        row_buffer.push(*cell);
                    }

                    index += 1;
                }
            }
        }

        buffer.push(row_buffer);
    }

    for (y, row) in buffer.iter().enumerate() {
        for x in 0..row.len() {
            energies[y][x] = calc_pixel_energy(x, y, &buffer);
        }
    }

    if width == target {
        return buffer;
    } else if target < width {
        return resize(width - 1, height, &buffer, target);
    } else {
        return resize(width + 1, height, &buffer, target);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).unwrap();

    let target = args.get(2).unwrap().parse::<usize>().unwrap();

    let rgba_img = match image::open(path) {
        Err(why) => panic!("No image: {:?}", why),
        Ok(data) => data.into_rgba8(),
    };

    let (raw_width, raw_height) = rgba_img.dimensions();

    let width = raw_width as usize;
    let height = raw_height as usize;

    let mut raw_output: Vec<Vec<BarePixel>> = vec![];
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
        raw_output.push(row_buffer);
    }

    let buffer = resize(width, height, &raw_output, target);

    let mut save_buff = image::ImageBuffer::new(to_u32(target), to_u32(height));

    for y in 0..height {
        for x in 0..width {
            let pixel = buffer[y][x];
            let rgba8 = pixel.extract();

            save_buff.put_pixel(to_u32(x), to_u32(y), image::Rgba(rgba8));
        }
    }

    match save_buff.save("./sc-cli/carved2.png") {
        Err(why) => panic!("Failed to save: {:?}", why),
        Ok(_) => return println!("Saved!"),
    };
}
