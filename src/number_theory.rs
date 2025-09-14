pub fn gcd(a: i64, b: i64) -> i64 {
    let mut x = a.abs();
    let mut y = b.abs();

    if x == 0 && y == 0 {
        return 0;
    }

    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }

    x
}

pub fn gcd_multiple(numbers: &[i64]) -> i64 {
    if numbers.is_empty() {
        return 0;
    }

    numbers.iter().fold(numbers[0], |acc, &x| gcd(acc, x))
}

pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }

    let gcd_val = gcd(a, b);
    (a.abs() / gcd_val) * b.abs()
}

pub fn lcm_multiple(numbers: &[i64]) -> i64 {
    if numbers.is_empty() {
        return 0;
    }

    if numbers.iter().any(|&x| x == 0) {
        return 0;
    }

    numbers.iter().fold(numbers[0], |acc, &x| lcm(acc, x))
}

fn is_prime_small(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    is_prime_small(n as u64)
}

pub fn factor_integer(n: i64) -> Vec<(i64, i32)> {
    if n == 0 {
        return vec![];
    }

    let mut num = n.abs();
    let mut factors = Vec::new();

    if num == 1 {
        return vec![(1, 1)];
    }

    let mut count = 0;
    while num % 2 == 0 {
        num /= 2;
        count += 1;
    }
    if count > 0 {
        factors.push((2, count));
    }

    let mut factor = 3;
    while factor * factor <= num {
        count = 0;
        while num % factor == 0 {
            num /= factor;
            count += 1;
        }
        if count > 0 {
            factors.push((factor, count));
        }
        factor += 2;
    }

    if num > 1 {
        factors.push((num, 1));
    }

    factors
}

pub fn divisors(n: i64) -> Vec<i64> {
    if n == 0 {
        return vec![];
    }

    let num = n.abs();
    if num == 1 {
        return vec![1];
    }

    let factors = factor_integer(num);
    let mut divs = vec![1];

    for (prime, power) in factors {
        let mut new_divs = Vec::new();
        let mut prime_power = 1;

        for _ in 0..=power {
            for &div in &divs {
                new_divs.push(div * prime_power);
            }
            prime_power *= prime;
        }

        divs = new_divs;
    }

    divs.sort_unstable();
    divs
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    if limit == 0 {
        is_prime[0] = false;
    }
    if limit >= 1 {
        is_prime[1] = false;
    }

    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in ((i * i)..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    is_prime
}

static mut SMALL_PRIMES: Option<Vec<i64>> = None;
static mut PRIMES_COMPUTED_UP_TO: usize = 0;

fn ensure_primes_computed(up_to: usize) {
    unsafe {
        if PRIMES_COMPUTED_UP_TO < up_to {
            let is_prime = sieve_of_eratosthenes(up_to);
            SMALL_PRIMES = Some(
                (2..=up_to)
                    .filter(|&i| is_prime[i])
                    .map(|i| i as i64)
                    .collect(),
            );
            PRIMES_COMPUTED_UP_TO = up_to;
        }
    }
}

pub fn prime(n: i32) -> i64 {
    if n <= 0 {
        panic!("Prime index must be positive");
    }

    let upper_bound = if n < 6 {
        15
    } else {
        let n_f = n as f64;
        let ln_n = n_f.ln();
        (n_f * (ln_n + ln_n.ln() - 1.0 + (ln_n.ln() - 2.0) / ln_n)) as usize
    };

    ensure_primes_computed(upper_bound.max(1000000));

    unsafe {
        if let Some(ref primes) = SMALL_PRIMES {
            if (n as usize) <= primes.len() {
                return primes[n as usize - 1];
            }
        }
    }

    panic!("Cannot compute prime({}) - need larger sieve", n);
}

pub fn prime_pi(x: i32) -> i64 {
    if x < 2 {
        return 0;
    }

    let limit = x as usize;
    ensure_primes_computed(limit);

    unsafe {
        if let Some(ref primes) = SMALL_PRIMES {
            return primes.iter().take_while(|&&p| p <= x as i64).count() as i64;
        }
    }

    panic!("Cannot compute prime_pi({}) - need larger sieve", x);
}
