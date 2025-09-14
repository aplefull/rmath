pub fn fibonacci(n: i32) -> i64 {
    if n == 0 {
        return 0;
    }
    if n == 1 || n == -1 {
        return 1;
    }
    if n == -2 {
        return -1;
    }

    if n < 0 {
        let result = fibonacci(-n);
        return if (-n) % 2 == 0 { -result } else { result };
    }

    if n > 92 {
        panic!("Fibonacci number too large for i64");
    }

    let mut a = 0i64;
    let mut b = 1i64;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

pub fn lucas(n: i32) -> i64 {
    if n == 0 {
        return 2;
    }
    if n == 1 {
        return 1;
    }

    if n < 0 {
        let result = lucas(-n);
        return if n % 2 == 0 { result } else { -result };
    }

    if n > 92 {
        panic!("Lucas number too large for i64");
    }

    let mut a = 2i64;
    let mut b = 1i64;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}
