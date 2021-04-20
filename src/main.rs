extern crate image;

use std::convert::TryFrom;

fn in_bounds(x: usize, y: usize, grid: &Vec<Vec<BarePixel>>) -> bool {
    match grid.get(y) {
        Some(row) => match row.get(x) {
            Some(_) => return true,
            None => return false,
        },
        _ => return false,
    }
}

fn calc_pixel_energy(x: usize, y: usize, img: &Vec<Vec<BarePixel>>) -> u64 {
    let middle = img[y][x];

    let r_m = middle.r as i64;
    let g_m = middle.g as i64;
    let b_m = middle.b as i64;

    let left = if x > 0 && in_bounds(x - 1, y, &img) {
        let rgba_l = img[y][x - 1];
        let r_l = rgba_l.r as i64;
        let g_l = rgba_l.g as i64;
        let b_l = rgba_l.b as i64;

        (r_l - r_m).pow(2) + (g_l - g_m).pow(2) + (b_l - b_m).pow(2)
    } else {
        0
    };

    let right = if in_bounds(x + 1, y, &img) {
        let rgba_r = img[y][x + 1];
        let r_r = rgba_r.r as i64;
        let g_r = rgba_r.g as i64;
        let b_r = rgba_r.b as i64;

        (r_r - r_m).pow(2) + (g_r - g_m).pow(2) + (b_r - b_m).pow(2)
    } else {
        0
    };

    (left as f64 + right as f64).sqrt() as u64
}

#[derive(Copy, Clone)]
struct BarePixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Copy, Clone)]
struct SeamEnergy {
    x: usize,
    y: usize,
    energy: u64,
    previous: Option<(usize, usize)>,
}

fn find_low_energy_seam(
    energies: &Vec<Vec<u64>>,
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let mut seam_energies: Vec<Vec<SeamEnergy>> = vec![];

    let mut seed = vec![];
    for x in 0..(width as usize) {
        let y = 0;

        seed.push(SeamEnergy {
            x,
            y,
            energy: energies[y][x],
            previous: None,
        });
    }

    seam_energies.push(seed);

    for y in 1..height {
        let mut row = vec![];

        for x in 0..width {
            let mut min_prev = 625;
            let mut min_prev_x = x;

            if x > 0 && x < width - 1 {
                for i in x - 1..x + 2 {
                    if i < width && seam_energies[y - 1][i].energy < min_prev {
                        min_prev = seam_energies[y - 1][i].energy;
                        min_prev_x = i;
                    }
                }
            }

            row.push(SeamEnergy {
                energy: min_prev + energies[y][x],
                x,
                y,
                previous: Some((min_prev_x, y - 1)),
            });
        }

        seam_energies.push(row);
    }

    let mut last_min = None;
    let mut min_seam_energy = 625;

    for x in 0..width {
        let y = height - 1;
        if seam_energies[y][x].energy < min_seam_energy {
            min_seam_energy = seam_energies[y][x].energy;
            last_min = Some((x, y));
        }
    }

    let mut seam = vec![];

    match last_min {
        None => seam,
        Some((x, y)) => {
            let mut current = seam_energies[y][x];

            loop {
                seam.push((current.x, current.y));

                match current.previous {
                    None => return seam,
                    Some((p_x, p_y)) => {
                        current = seam_energies[p_y][p_x];
                    }
                }
            }
        }
    }
}

fn main() {
    let rgba_img = match image::open("landscape.png") {
        Err(why) => panic!("No image: {:?}", why),
        Ok(data) => data.into_rgba8(),
    };

    let (raw_width, raw_height) = rgba_img.dimensions();

    let orig_width = raw_width as usize;
    let height = raw_height as usize;

    let mut energies = vec![vec![625; orig_width]; height];

    let mut raw_output: Vec<Vec<BarePixel>> = vec![];

    for row in rgba_img.rows() {
        let mut row_buffer = vec![];
        for cell in row {
            let r = cell[0];
            let g = cell[1];
            let b = cell[2];
            let a = cell[3];

            let pixel = BarePixel { r, g, b, a };

            row_buffer.push(pixel);
        }

        raw_output.push(row_buffer);
    }

    for y in 0..height {
        for x in 0..orig_width {
            energies[y][x] = calc_pixel_energy(x, y, &raw_output);
        }
    }

    let target = 2 * 1920 / 3;

    let mut width = orig_width;

    loop {
        if width == target {
            // end
            // save image

            let to_u32 = |n: usize| match u32::try_from(n) {
                Ok(m) => m,
                Err(why) => panic!("Bad usize u32: {:?}", why),
            };

            let mut save_buff = image::ImageBuffer::new(to_u32(width), to_u32(height));

            for y in 0..height {
                for x in 0..width {
                    let pixel = raw_output[y][x];
                    save_buff.put_pixel(
                        to_u32(x),
                        to_u32(y),
                        image::Rgba([pixel.r, pixel.g, pixel.b, pixel.a]),
                    );
                }
            }

            match save_buff.save("carved.png") {
                Err(why) => panic!("Failed to save: {:?}", why),
                Ok(_) => return println!("Saved!"),
            };
        }

        let seam = find_low_energy_seam(&energies, width, height);

        let mut buffer: Vec<Vec<BarePixel>> = vec![];

        for (s_x, s_y) in &seam {
            let mut row_buffer: Vec<BarePixel> = vec![];

            match raw_output.iter().nth(*s_y as usize) {
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

        raw_output = buffer;

        width -= 1;
    }
}
