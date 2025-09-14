use rmath::noise::*;
use std::collections::HashSet;

#[test]
fn test_random_deterministic() {
    assert_eq!(random(1.0), random(1.0));
    assert_eq!(random(0.0), random(0.0));
    assert_eq!(random(42.5), random(42.5));
}

#[test]
fn test_random_range() {
    for i in 0..100 {
        let x = i as f64 * 0.1;
        let r = random(x);
        assert!(r >= 0.0 && r <= 1.0, "random({}) = {} is out of range [0,1]", x, r);
    }
}

#[test]
fn test_random_distribution() {
    let mut values = Vec::new();
    for i in 0..1000 {
        values.push(random(i as f64));
    }

    let mean = values.iter().sum::<f64>() / values.len() as f64;
    assert!((mean - 0.5).abs() < 0.1, "Mean {} should be close to 0.5", mean);

    let low_count = values.iter().filter(|&&x| x < 0.25).count();
    let high_count = values.iter().filter(|&&x| x > 0.75).count();

    assert!(low_count > 200, "Should have a reasonable number of low values");
    assert!(high_count > 200, "Should have a reasonable number of high values");
}

#[test]
fn test_random_negative_inputs() {
    assert_eq!(random(-1.0), random(-1.0));
    assert_ne!(random(-1.0), random(1.0));
    assert_ne!(random(-2.5), random(-2.6));
}

#[test]
fn test_random_special_values() {
    let r_inf = random(f64::INFINITY);
    let r_neg_inf = random(f64::NEG_INFINITY);
    let r_nan = random(f64::NAN);

    assert!(r_inf >= 0.0 && r_inf <= 1.0);
    assert!(r_neg_inf >= 0.0 && r_neg_inf <= 1.0);
    assert!(r_nan >= 0.0 && r_nan <= 1.0);

    assert_eq!(random(f64::INFINITY), random(f64::INFINITY));
    assert_eq!(random(f64::NEG_INFINITY), random(f64::NEG_INFINITY));
}

#[test]
fn test_simple_hash_deterministic() {
    assert_eq!(simple_hash(1.0), simple_hash(1.0));
    assert_eq!(simple_hash(0.0), simple_hash(0.0));
    assert_eq!(simple_hash(-5.5), simple_hash(-5.5));
}

#[test]
fn test_simple_hash_avalanche() {
    let h1 = simple_hash(1.0);
    let h2 = simple_hash(1.0000000000000002);

    assert_ne!(h1, h2);

    let diff_bits = (h1 ^ h2).count_ones();
    assert!(diff_bits > 10, "Should have good avalanche effect, got {} differing bits", diff_bits);
}

#[test]
fn test_simple_hash_distribution() {
    let mut hashes = HashSet::new();
    let mut collisions = 0;

    for i in 0..1000 {
        let hash = simple_hash(i as f64 * 0.1);
        if !hashes.insert(hash) {
            collisions += 1;
        }
    }

    assert!(collisions < 50, "Too many hash collisions: {}", collisions);
    assert!(hashes.len() > 950, "Should have mostly unique hashes, got {}", hashes.len());
}

#[test]
fn test_simple_hash_negative_values() {
    let h_pos = simple_hash(5.0);
    let h_neg = simple_hash(-5.0);
    assert_ne!(h_pos, h_neg);

    assert_eq!(simple_hash(-1.0), simple_hash(-1.0));
}

#[test]
fn test_simple_hash_special_values() {
    let _h_zero = simple_hash(0.0);
    let _h_neg_zero = simple_hash(-0.0);

    let _h_inf = simple_hash(f64::INFINITY);
    let _h_neg_inf = simple_hash(f64::NEG_INFINITY);

    assert_eq!(simple_hash(f64::INFINITY), simple_hash(f64::INFINITY));
    assert_eq!(simple_hash(f64::NEG_INFINITY), simple_hash(f64::NEG_INFINITY));
}
