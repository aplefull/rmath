use rmath::sequences::*;

#[test]
fn test_fibonacci() {
    const FIBONACCI: [i64; 41] = [
        -6765, 4181, -2584, 1597, -987, 610, -377, 233, -144, 89, -55, 34, -21, 13, -8, 5, -3, 2,
        -1, 1, 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
        6765,
    ];

    for n in -20..=20 {
        assert_eq!(fibonacci(n), FIBONACCI[(n + 20) as usize]);
    }
}

#[test]
fn test_lucas() {
    const LUCAS: [i64; 41] = [
        15127, -9349, 5778, -3571, 2207, -1364, 843, -521, 322, -199, 123, -76, 47, -29, 18, -11,
        7, -4, 3, -1, 2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123, 199, 322, 521, 843, 1364, 2207, 3571,
        5778, 9349, 15127,
    ];

    for n in -20..=20 {
        assert_eq!(lucas(n), LUCAS[(n + 20) as usize]);
    }
}

#[test]
fn test_fibonacci_lucas_relationship() {
    // L(n) = F(n-1) + F(n+1)
    for n in 1..60 {
        assert_eq!(lucas(n), fibonacci(n - 1) + fibonacci(n + 1));
    }
}
