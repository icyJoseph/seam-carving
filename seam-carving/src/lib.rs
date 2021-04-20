#[derive(Copy, Clone)]
pub struct BarePixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Copy, Clone)]
pub struct SeamEnergy {
    pub x: usize,
    pub y: usize,
    pub energy: u64,
    pub previous: Option<(usize, usize)>,
}

impl BarePixel {
    pub fn new(arr: [u8; 4]) -> Self {
        BarePixel {
            r: arr[0],
            g: arr[1],
            b: arr[2],
            a: arr[3],
        }
    }

    pub fn extract(self: Self) -> [u8; 4] {
        [self.r, self.g, self.g, self.a]
    }
}

pub fn in_bounds(x: usize, y: usize, grid: &Vec<Vec<BarePixel>>) -> bool {
    match grid.get(y) {
        Some(row) => match row.get(x) {
            Some(_) => return true,
            None => return false,
        },
        _ => return false,
    }
}

pub fn calc_pixel_energy(x: usize, y: usize, img: &Vec<Vec<BarePixel>>) -> u64 {
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

pub fn find_low_energy_seam(
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

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
