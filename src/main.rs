extern crate image;

use std::convert::TryFrom;

use image::{DynamicImage, GenericImageView};

fn calc_pixel_energy(x: u32, y: u32, img: &DynamicImage) -> u64 {
    let middle = img.get_pixel(x, y);

    let rgba_m = middle.0;
    let r_m = rgba_m[0] as i64;
    let g_m = rgba_m[1] as i64;
    let b_m = rgba_m[2] as i64;

    let left = if x > 0 && img.in_bounds(x - 1, y) {
        let rgba_l = img.get_pixel(x - 1, y).0;
        let r_l = rgba_l[0] as i64;
        let g_l = rgba_l[1] as i64;
        let b_l = rgba_l[2] as i64;

        (r_l - r_m).pow(2) + (g_l - g_m).pow(2) + (b_l - b_m).pow(2)
    } else {
        0
    };

    let right = if img.in_bounds(x + 1, y) {
        let rgba_r = img.get_pixel(x + 1, y).0;
        let r_r = rgba_r[0] as i64;
        let g_r = rgba_r[1] as i64;
        let b_r = rgba_r[2] as i64;

        (r_r - r_m).pow(2) + (g_r - g_m).pow(2) + (b_r - b_m).pow(2)
    } else {
        0
    };

    (left as f64 + right as f64).sqrt() as u64
}

#[derive(Copy, Clone)]
struct SeamEnergy {
    x: usize,
    y: usize,
    energy: u64,
    previous: Option<(usize, usize)>,
}

fn find_low_energy_seam(energies: &Vec<Vec<u64>>, width: u32, height: u32) -> Vec<(u32, u32)> {
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

    for y in 1..(height as usize) {
        let mut row = vec![];
        for x in 0..(width as usize) {
            let mut min_prev = 625;
            let mut min_prev_x = x;

            if x > 0 && x < (width as usize) - 1 {
                for i in x - 1..x + 2 {
                    if i < (width as usize) && seam_energies[y - 1][i].energy < min_prev {
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

    for x in 0..(width as usize) {
        let y = (height as usize) - 1;
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
                let c_x = u32::try_from(current.x).unwrap();
                let c_y = u32::try_from(current.y).unwrap();
                seam.push((c_x, c_y));

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
    let img = match image::open("./src/landscape.png") {
        Err(why) => panic!("No image: {:?}", why),
        Ok(i) => i,
    };

    let (width, height) = img.dimensions();
    let mut energies = vec![vec![625; width as usize]; height as usize];

    for pixel in img.pixels() {
        let (x, y, _) = pixel;

        energies[y as usize][x as usize] = calc_pixel_energy(x, y, &img);
    }

    let seam = find_low_energy_seam(&energies, width, height);

    println!("Dimensions {:?}", seam);
    println!("Dimensions {:?}", img.dimensions());
}
