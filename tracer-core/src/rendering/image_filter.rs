use rayon::prelude::*;

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
