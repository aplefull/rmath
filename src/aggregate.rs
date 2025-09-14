pub fn sum(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    if values.iter().any(|x| x.is_nan()) {
        return f64::NAN;
    }

    values.iter().sum()
}

pub fn product(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 1.0;
    }

    if values.iter().any(|x| x.is_nan()) {
        return f64::NAN;
    }

    values.iter().product()
}

pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }

    sum(values) / values.len() as f64
}

pub fn geometric_mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }

    if values.iter().any(|&x| x <= 0.0) {
        return f64::NAN;
    }

    let log_sum: f64 = values.iter().map(|x| x.ln()).sum();
    (log_sum / values.len() as f64).exp()
}

pub fn harmonic_mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }

    if values.iter().any(|&x| x == 0.0) {
        return f64::NAN;
    }

    let reciprocal_sum: f64 = values.iter().map(|x| 1.0 / x).sum();
    values.len() as f64 / reciprocal_sum
}

pub fn variance(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return f64::NAN;
    }

    let m = mean(values);
    if m.is_nan() {
        return f64::NAN;
    }

    let sum_sq_diff: f64 = values.iter().map(|x| (x - m).powi(2)).sum();
    sum_sq_diff / (values.len() - 1) as f64
}

pub fn standard_deviation(values: &[f64]) -> f64 {
    variance(values).sqrt()
}

pub fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }

    if values.iter().any(|x| x.is_nan()) {
        return f64::NAN;
    }

    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = sorted.len();
    if len % 2 == 1 {
        sorted[len / 2]
    } else {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
    }
}

pub fn range(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }

    if values.iter().any(|x| x.is_nan()) {
        return f64::NAN;
    }

    let min_val = values.iter().fold(f64::INFINITY, |acc, &x| acc.min(x));
    let max_val = values.iter().fold(f64::NEG_INFINITY, |acc, &x| acc.max(x));

    max_val - min_val
}

pub fn quantile(values: &[f64], q: f64) -> f64 {
    if values.is_empty() || q < 0.0 || q > 1.0 {
        return f64::NAN;
    }

    if values.iter().any(|x| x.is_nan()) {
        return f64::NAN;
    }

    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let index = q * (sorted.len() - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;

    if lower == upper {
        sorted[lower]
    } else {
        let weight = index - lower as f64;
        sorted[lower] * (1.0 - weight) + sorted[upper] * weight
    }
}

pub fn sum_i64(values: &[i64]) -> i64 {
    values.iter().sum()
}

pub fn product_i64(values: &[i64]) -> i64 {
    if values.is_empty() {
        return 1;
    }
    values.iter().product()
}

pub fn max_i64(values: &[i64]) -> Option<i64> {
    values.iter().max().copied()
}

pub fn min_i64(values: &[i64]) -> Option<i64> {
    values.iter().min().copied()
}

pub fn cumulative_sum(values: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(values.len());
    let mut running_sum = 0.0;

    for &value in values {
        running_sum += value;
        result.push(running_sum);
    }

    result
}

pub fn cumulative_product(values: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(values.len());
    let mut running_product = 1.0;

    for &value in values {
        running_product *= value;
        result.push(running_product);
    }

    result
}

pub fn pairwise_differences(values: &[f64]) -> Vec<f64> {
    if values.len() < 2 {
        return vec![];
    }

    values.windows(2).map(|w| w[1] - w[0]).collect()
}
