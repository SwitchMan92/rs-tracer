use range2d::Range2D;
use rayon::prelude::*;

pub fn apply_msaa(screen_width: isize, buffer: &mut [u8], strides: (isize, isize)) {
    let x_offset: isize = screen_width * 4;
    let slice_end: isize = buffer.len() as isize - x_offset - 4;

    let test: Vec<u8> = (x_offset + 4..slice_end)
        .into_par_iter()
        .map(|x: isize| {
            (Range2D::new(-strides.0..strides.0 + 1, -strides.1..strides.1 + 1)
                .fold(0_u16, |a, b| {
                    a + buffer[(x + (b.0 * x_offset + b.1 * 4)) as usize] as u16
                })
                / ((strides.0 * 2 + 1) * (strides.1 * 2 + 1)) as u16) as u8
        })
        .collect();

    buffer[(x_offset + 4) as usize..slice_end as usize].copy_from_slice(test.as_slice());
}
