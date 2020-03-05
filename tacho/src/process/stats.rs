pub struct Stats {
    pub avg: f64,
    pub min: f64,
    pub max: f64,
    pub stddev: f64,
    pub conf_interval_95: f64,
    pub n_recommended: f64,
}

pub fn min_max(values: &[f64]) -> Option<(f64, f64)> {
    fn min_max_rec(values: &[f64], min: f64, max: f64) -> Option<(f64, f64)> {
        match values {
            [] => Some((min, max)),
            _ => {
                let min_new = if values[0] < min { values[0] } else { min };
                let max_new = if values[0] > max { values[0] } else { max };
                min_max_rec(&values[1..], min_new, max_new)
            }
        }
    }

    match values {
        [] => None,
        [v] => Some((*v, *v)),
        _ => min_max_rec(
            &values[1..],
            *values.first().unwrap(),
            *values.first().unwrap(),
        ),
    }
}

pub fn calculate_stats(values: &[f64]) -> Stats {
    let n = &values.len();
    let nn = *n as f64;
    let (min, max) = min_max(values).unwrap_or((0.0, 0.0));

    let sum: f64 = values.iter().sum();
    let avg = sum as f64 / nn;
    let diff_sqt: f64 = values.iter().map(|x| *x - avg).map(|x| x * x).sum();
    let variance = diff_sqt / (nn - 1.0); // Bessel correction in variance
    let stddev = variance.sqrt();
    let conf_interval = stddev / nn.sqrt();
    let z95 = 1.96;
    let conf_interval_95 = z95 * conf_interval;

    // Calculate n for 95% confidence interval < 0.05 avg

    let avg5 = 0.05 * avg;
    let n_recommended = (z95 / avg5) * (z95 / avg5) * variance;

    Stats {
        avg,
        min,
        max,
        stddev,
        conf_interval_95,
        n_recommended,
    }
}
