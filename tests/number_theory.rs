use rmath::number_theory::*;

#[test]
fn test_gcd_basic() {
    assert_eq!(gcd(12, 8), 4);
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(17, 13), 1);
    assert_eq!(gcd(100, 25), 25);
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
    assert_eq!(gcd(0, 0), 0);
}

#[test]
fn test_gcd_negative() {
    assert_eq!(gcd(-12, 8), 4);
    assert_eq!(gcd(12, -8), 4);
    assert_eq!(gcd(-12, -8), 4);
    assert_eq!(gcd(-48, 18), 6);
}

#[test]
fn test_lcm_basic() {
    assert_eq!(lcm(12, 8), 24);
    assert_eq!(lcm(6, 9), 18);
    assert_eq!(lcm(5, 7), 35);
    assert_eq!(lcm(4, 6), 12);
    assert_eq!(lcm(0, 5), 0);
    assert_eq!(lcm(5, 0), 0);
}

#[test]
fn test_lcm_negative() {
    assert_eq!(lcm(-12, 8), 24);
    assert_eq!(lcm(12, -8), 24);
    assert_eq!(lcm(-12, -8), 24);
}

#[test]
fn test_gcd_lcm_relationship() {
    let test_cases = [(12, 8), (48, 18), (15, 25), (7, 11)];

    for (a, b) in test_cases.iter() {
        let gcd_val = gcd(*a, *b);
        let lcm_val = lcm(*a, *b);
        assert_eq!(gcd_val * lcm_val, a.abs() * b.abs());
    }
}

#[test]
fn test_gcd_multiple() {
    assert_eq!(gcd_multiple(&[12, 8, 16]), 4);
    assert_eq!(gcd_multiple(&[24, 36, 48]), 12);
    assert_eq!(gcd_multiple(&[7, 11, 13]), 1);
    assert_eq!(gcd_multiple(&[100]), 100);
    assert_eq!(gcd_multiple(&[]), 0);
}

#[test]
fn test_lcm_multiple() {
    assert_eq!(lcm_multiple(&[4, 6, 8]), 24);
    assert_eq!(lcm_multiple(&[3, 5, 15]), 15);
    assert_eq!(lcm_multiple(&[2, 3, 5]), 30);
    assert_eq!(lcm_multiple(&[12]), 12);
    assert_eq!(lcm_multiple(&[]), 0);
    assert_eq!(lcm_multiple(&[0, 5, 10]), 0);
}

#[test]
fn test_gcd_properties() {
    assert_eq!(gcd(12, 8), gcd(8, 12));

    assert_eq!(gcd(15, 1), 1);
    assert_eq!(gcd(1, 15), 1);

    let nums = [12, 18, 24];
    assert_eq!(gcd(gcd(nums[0], nums[1]), nums[2]), gcd_multiple(&nums));
}

#[test]
fn test_large_numbers() {
    let a: i64 = 1_000_000_000;
    let b: i64 = 2_000_000_000;

    assert_eq!(gcd(a, b), 1_000_000_000);
    assert_eq!(lcm(a, b), 2_000_000_000);

    let x: i64 = 1_000_000_037;
    let y: i64 = 1_000_000_039;

    assert_eq!(gcd(x, y), 1);
    assert_eq!(lcm(x, y), x * y);

    let values_gcd = [600_000_000, 900_000_000, 1_500_000_000];
    assert_eq!(gcd_multiple(&values_gcd), 300_000_000);

    let values_lcm = [15_000_000, 20_000_000, 30_000_000];
    assert_eq!(lcm_multiple(&values_lcm), 60_000_000);

    let mix = [1_000_000_000, 250, 500];
    assert_eq!(gcd_multiple(&mix), 250);
    assert_eq!(lcm_multiple(&mix), 1_000_000_000);
}

#[test]
fn test_primes() {
    const PRIMES: [i64; 20] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
        31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    ];

    for i in 1..=20 {
        let prime_to_test = prime(i);
        assert_eq!(prime_to_test, PRIMES[i as usize - 1]);

        let is_prime = is_prime(PRIMES[i as usize - 1]);
        assert!(is_prime, "Expected {} to be prime, but it was not.", prime_to_test);
    }

    const LARGE_PRIMES: [i64; 11] = [
        882377, 882389, 882391, 882433, 882439,
        882449, 882451, 882461, 882481, 882491,
        882517,
    ];

    for i in 0..=10 {
        let prime_to_test = prime(70000 + i);
        assert_eq!(prime_to_test, LARGE_PRIMES[i as usize]);

       let is_prime = is_prime(LARGE_PRIMES[i as usize]);
       assert!(is_prime, "Expected {} to be prime, but it was not.", prime_to_test);
    }

    const HUGE_PRIMES: [i64; 11] = [
        252097800623, 252097800629, 252097800637, 252097800667,
        252097800737, 252097800743, 252097800839, 252097800853,
        252097800869, 252097800901, 252097800907,
    ];

    for i in 0..=10 {
        let is_prime = is_prime(HUGE_PRIMES[i as usize]);
        assert!(is_prime, "Expected {} to be prime, but it was not.", HUGE_PRIMES[i as usize]);
    }
}

#[test]
fn test_prime_p() {
    let x = prime_pi(10000000);
    assert_eq!(x, 664579, "Expected 9592 primes up to 100000, found {}", x);
}

#[test]
fn test_divisors() {
    let divisors_of_0 = divisors(0);
    let divisors_of_1 = divisors(1);
    let divisors_of_100 = divisors(100);
    let divisors_of_1234 = divisors(1234);
    let divisors_of_1000000000 = divisors(1_000_000_000);
    let divisors_of_2543568463 = divisors(2543568463);

    assert_eq!(divisors_of_0, vec![]);
    assert_eq!(divisors_of_1, vec![1]);
    assert_eq!(divisors_of_100, vec![1, 2, 4, 5, 10, 20, 25, 50, 100]);
    assert_eq!(divisors_of_1234, vec![1, 2, 617, 1234]);
    assert_eq!(divisors_of_1000000000, vec![1, 2, 4, 5, 8, 10, 16, 20, 25, 32, 40, 50, 64, 80, 100, 125, 128, 160, 200, 250, 256, 320, 400, 500, 512, 625, 640, 800, 1000, 1250, 1280, 1600, 2000, 2500, 2560, 3125, 3200, 4000, 5000, 6250, 6400, 8000, 10000, 12500, 12800, 15625, 16000, 20000, 25000, 31250, 32000, 40000, 50000, 62500, 64000, 78125, 80000, 100000, 125000, 156250, 160000, 200000, 250000, 312500, 320000, 390625, 400000, 500000, 625000, 781250, 800000, 1000000, 1250000, 1562500, 1600000, 1953125, 2000000, 2500000, 3125000, 3906250, 4000000, 5000000, 6250000, 7812500, 8000000, 10000000, 12500000, 15625000, 20000000, 25000000, 31250000, 40000000, 50000000, 62500000, 100000000, 125000000, 200000000, 250000000, 500000000, 1000000000]);
    assert_eq!(divisors_of_2543568463, vec![1, 2543568463]);
}

#[test]
fn test_factor_integer() {
    let factors_of_0 = factor_integer(0);
    assert_eq!(factors_of_0, vec![]);

    let factors_of_1 = factor_integer(1);
    assert_eq!(factors_of_1, vec![(1, 1)]);

    let factors_of_72760 = factor_integer(72760);
    assert_eq!(factors_of_72760, vec![(2, 3), (5, 1), (17, 1), (107, 1)]);

    let factors_of_7919 = factor_integer(7919);
    assert_eq!(factors_of_7919, vec![(7919, 1)]);
}
