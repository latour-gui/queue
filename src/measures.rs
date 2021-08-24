pub struct Data {
    pub rho: f64,
    pub lambda: f64,
    pub mu: Option<f64>,   // only for exp
    pub k: Option<usize>,  // only for erlang
    pub beta: Option<f64>, // only for erlang
    pub theta: f64,
    pub avg_stay_time: f64,
    pub probability_p_off: f64,
    pub probability_p_setup: f64,
    pub n_simulations: usize,
}
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

pub fn theoretic_stay_avg_erlang(lambda: f64, theta: f64, rho: f64, k: usize, beta: f64) -> f64 {
    assert!(1.0 < (1.0 / beta));
    let e_b = 1.0 / ((1.0 - beta).powi(k as i32)); // with moment generator
    let e_b = k as f64 * beta; // mean
    let e_t = 1.0 / (1.0 - 1.0 / theta); // with moment generator
    let e_t = 1.0 / theta; // mean

    let e_bb = k as f64 / lambda / lambda; // variance
    let e_bb = 1.0 / ((1.0 - 2.0 * beta).powi(k as i32)); // with moment generator
    let e_tt = theta * theta; // variance
    let e_tt = 1.0 / (1.0 - 2.0 / theta); // with moment generator

    let e_rb = e_bb / 2.0 / e_b;
    let e_rt = e_tt / 2.0 / e_t;

    let e_w = rho * e_rb / (1.0 - rho)
        + (1.0 / lambda) / (1.0 / lambda + e_t) * e_t
        + e_t / (1.0 / lambda + e_t) * e_rt;

    e_w + e_b
}
