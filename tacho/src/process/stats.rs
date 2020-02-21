pub struct Stats {
    pub avg: f64,
    pub min: u128,
    pub max: u128,
    pub stddev: f64
}

pub fn calculate_stats(durations: &Vec<u128>) -> Stats {
    let n = &durations.len();
    let nn = *n as f64;
    
    let min = durations.iter().min().unwrap_or(&0);
    let max = durations.iter().max().unwrap_or(&0);
    
    let sum:u128 = durations.iter().sum();
    let avg = sum as f64 / nn;
    let diff_sqt:f64 = durations.iter().map(|x| *x as f64 - avg).map(|x| x*x).sum();
    let variance = diff_sqt / (nn - 1.0); // Bessel correction in variance
    let stddev = variance.sqrt();

    return Stats {
        avg: avg,
        min: *min,
        max: *max,
        stddev: stddev    
    }
}