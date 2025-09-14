pub fn random(x: f64) -> f64 {
    let hash_val = simple_hash(x);
    (hash_val as f64) / (u32::MAX as f64)
}

pub fn simple_hash(x: f64) -> u32 {
    let bits = x.to_bits();

    let low = (bits & 0xFFFFFFFF) as u32;
    let high = (bits >> 32) as u32;

    let mut hash = low ^ high;
    hash = hash.wrapping_mul(0x9e3779b9);
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85ebca6b);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash ^= hash >> 16;

    hash
}
