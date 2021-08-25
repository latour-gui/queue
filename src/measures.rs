//! The `measures.rs` file contains the `Data` struct, dump of all metrics calculated for a given
//! rho. It also contains the `Mean` trait, where is defined the `calculate_mean()` function.
//! Finally it's also here that can be found all the function that compute theoretical values & graphs

/// The `Data` struct is a big dump of all the calculated measures from the simulation
/// that may be useful during the print of the graph.
///
/// I am not really proud of how the `Data` nor the graphs functions are designed but looks like
/// I did roll with those so let's go.
///
/// Properties:
///
/// * `rho`: The rho that was used to calculate the following metrics.
/// * `lambda`: Poisson parameter, for inter-arrival.
/// * `mu`: Exponential parameter, for service of the first experiment.
/// * `k`: Shape of the Erlang parameter, for service of the second experiment.
/// * `beta`: Scale of the Erlang parameter, for the service of the second experiment.
/// * `theta`: Exponential parameter, for warmup time.
/// * `avg_stay_time`: The average time spent in the system.
/// * `corrected_variance_avg_stay`: The calculated and corrected variance of the time spent in the
/// system.
/// * `probability_p_off`: Probability that a client finds a server off on arrival.
/// * `corrected_variance_p_off`: The calculated and corrected variance of the probability p_off.
/// * `probability_p_setup`: Probability that a client finds a server warming up on arrival.
/// * `corrected_variance_p_setup`: The calculated and corrected variance of the probability p_setup.
/// * `n_simulations`: The number of simulations made to obtain all the previous data.
#[derive(Copy, Clone)]
pub struct Data {
    pub rho: f64,
    pub lambda: f64,
    pub mu: Option<f64>,   // only for exp (experiment 1)
    pub k: Option<usize>,  // only for erlang (experiment 2)
    pub beta: Option<f64>, // only for erlang (experiment 2)
    pub theta: f64,
    pub avg_stay_time: f64,
    pub corrected_variance_avg_stay: f64,
    pub probability_p_off: f64,
    pub corrected_variance_p_off: f64,
    pub probability_p_setup: f64,
    pub corrected_variance_p_setup: f64,
    pub n_simulations: usize,
}

/// I created this trait because I had enough of calculating the mean by hand (sum / length)
/// Of course the function name `mean()` was already taken by a library used so I had to name the
/// function `calculate_mean()`.
pub trait Mean {
    fn calculate_mean(self) -> f64;
}
impl<F, T> Mean for T
where
    T: Iterator<Item = F>,
    F: std::borrow::Borrow<f64>,
{
    fn calculate_mean(self) -> f64 {
        self.zip(1..).fold(0., |s, (e, i)| {
            (*e.borrow() + s * (i - 1) as f64) / i as f64
        })
    }
}

pub fn theoretic_stay_avg_exp(rho: f64, mu: f64, theta: f64) -> f64 {
    (1.0 / mu) / (1.0 - rho) + 1.0 / theta
}

pub fn theoretic_p_off_exp(rho: f64, lambda: f64, theta: f64) -> f64 {
    (1.0 - rho) * (1.0 / lambda) / (1.0 / lambda + 1.0 / theta)
}

pub fn theoretic_p_setup_exp(rho: f64, lambda: f64, theta: f64) -> f64 {
    (1.0 - rho) * (1.0 / theta) / (1.0 / lambda + 1.0 / theta)
}

pub fn theoretic_p_off_gen(rho: f64, lambda: f64, expectation_start: f64) -> f64 {
    (1.0 - rho) * (1.0 - lambda) / (1.0 / lambda + expectation_start)
}

pub fn theoretic_p_setup_gen(rho: f64, lambda: f64, expectation_start: f64) -> f64 {
    (1.0 - rho) * expectation_start / (1.0 / lambda + expectation_start)
}

/// This function gave me headache.
/// I tried with the function moment generator instead of just using variance but with no luck.
pub fn theoretic_stay_avg_erlang(lambda: f64, theta: f64, rho: f64, k: usize, beta: f64) -> f64 {
    assert!(1.0 < (1.0 / beta));
    // e_b = 𝔼[B]
    let _e_b = 1.0 / ((1.0 - beta).powi(k as i32)); // with moment generator
    let e_b = k as f64 * beta; // mean

    // e_t = 𝔼[T]
    let _e_t = 1.0 / (1.0 - 1.0 / theta); // with moment generator
    let e_t = 1.0 / theta; // mean

    // e_bb = 𝔼[B²]
    let _e_bb = k as f64 / lambda / lambda; // variance
    let e_bb = 1.0 / ((1.0 - 2.0 * beta).powi(k as i32)); // with moment generator

    // e_tt = 𝔼[T²]
    let _e_tt = theta * theta; // variance
    let e_tt = 1.0 / (1.0 - 2.0 / theta); // with moment generator

    let e_rb = e_bb / 2.0 / e_b;
    let e_rt = e_tt / 2.0 / e_t;

    // e_w = 𝔼[W]
    let e_w = rho * e_rb / (1.0 - rho)
        + (1.0 / lambda) / (1.0 / lambda + e_t) * e_t
        + e_t / (1.0 / lambda + e_t) * e_rt;

    e_w + e_b
}

pub fn corrected_standard_deviation(avg: f64, data: &[f64]) -> f64 {
    f64::sqrt(1.0 / (data.len() - 1) as f64 * data.iter().map(|d| (d - avg).powi(2)).sum::<f64>())
}

pub fn test_statistic(avg: f64, theoretical_avg: f64, standard_deviation: f64, n: usize) -> f64 {
    (avg - theoretical_avg) / (standard_deviation / f64::sqrt((n - 1) as f64))
}

pub fn is_inside_interval(value: f64, threshold: f64) -> bool {
    (-threshold) <= value && value <= threshold
}
