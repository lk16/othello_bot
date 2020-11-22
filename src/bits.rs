use packed_simd::*;

pub fn upper_bit(mut x: u64x4) -> u64x4 {
    x = x | (x >> 1);
    x = x | (x >> 2);
    x = x | (x >> 4);
    x = x | (x >> 8);
    x = x | (x >> 16);
    x = x | (x >> 32);
    let lowers: u64x4 = x >> 1;
    x & !lowers
}

pub fn nonzero(x: u64x4) -> u64x4 {
    let zero = u64x4::new(0, 0, 0, 0);
    let mask = x.ne(zero);
    let one = u64x4::new(1, 1, 1, 1);
    one & u64x4::from_cast(mask)
}

#[cfg(test)]
mod tests {
    // TODO test fn nonzero(x: u64x4) -> u64x4
    // TODO test fn upper_bit(mut x: u64x4) -> u64x4
}
