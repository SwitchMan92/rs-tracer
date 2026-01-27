use range2d::Range2D;
use rayon::prelude::*;

// ########################################

pub fn apply_msaa(screen_width: usize, buffer: &mut [u8], strides: (i32, i32)) {
    let x_offset = screen_width * 4;
    let slice_end = buffer.len() - x_offset - 4;

    let test: Vec<u8> = (x_offset + 4..slice_end)
        .into_par_iter()
        .map(|x| {
            let mut accumulator: u16 = 0;

            for row in -strides.0..=strides.0 {
                for col in -strides.1..=strides.1 {
                    accumulator +=
                        buffer[(x as i32 + (x_offset as i32 * row + col * 4)) as usize] as u16
                }
            }

            accumulator /= ((strides.0 * 2 + 1) * (strides.1 * 2 + 1)) as u16;
            accumulator as u8
        })
        .collect();

    buffer[x_offset + 4..slice_end].copy_from_slice(test.as_slice());
}

pub fn apply_msaa_2d(screen_width: isize, buffer: &mut [u8], strides: (isize, isize)) {
    let x_offset: isize = screen_width * 4;
    let slice_end: isize = buffer.len() as isize - x_offset - 4;
    let rng = Range2D::new(-strides.0..strides.0 + 1, -strides.1..strides.1 + 1);

    let test: Vec<u8> = (x_offset + 4..slice_end)
        .map(|x: isize| {
            (rng.to_owned().fold(0_u16, |a, b| {
                a + buffer[(x + (b.0 * x_offset + b.1 * 4)) as usize] as u16
            }) / ((strides.0 * 2 + 1) * (strides.1 * 2 + 1)) as u16) as u8
        })
        .collect();

    buffer[(x_offset + 4) as usize..slice_end as usize].copy_from_slice(test.as_slice());
}

// ########################################

pub fn apply_msaa_3x3(screen_width: usize, buffer: &mut [u8]) {
    let x_offset = screen_width * 4;
    let slice_end = buffer.len() - x_offset - 4;

    let test: Vec<u8> = (x_offset + 4..slice_end)
        .into_par_iter()
        .map(|x| {
            ((buffer[x - x_offset - 4] as u16
                + buffer[x - x_offset] as u16
                + buffer[x - x_offset + 4] as u16
                + buffer[x - 4] as u16
                + buffer[x] as u16
                + buffer[x + 4] as u16
                + buffer[x + x_offset - 4] as u16
                + buffer[x + x_offset] as u16
                + buffer[x + x_offset + 4] as u16)
                / 9) as u8
        })
        .collect();

    buffer[x_offset + 4..slice_end].copy_from_slice(&test.as_slice());
}

// ########################################

pub fn apply_msaa_3x3_serial(screen_width: usize, buffer: &mut [u8]) {
    let x_offset = screen_width * 4;
    let slice_end = buffer.len() - x_offset - 4;

    (x_offset + 4..slice_end).into_iter().for_each(|x| {
        buffer[x] = ((buffer[x - x_offset - 4] as u16
            + buffer[x - x_offset] as u16
            + buffer[x - x_offset + 4] as u16
            + buffer[x - 4] as u16
            + buffer[x] as u16
            + buffer[x + 4] as u16
            + buffer[x + x_offset - 4] as u16
            + buffer[x + x_offset] as u16
            + buffer[x + x_offset + 4] as u16)
            / 9) as u8
    });
}
