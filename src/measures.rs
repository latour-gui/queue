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

pub fn theoretic_expectation_delay(
    rho: f64,
    expectation_start: f64,
    second_order_moment_start: f64,
    lambda: f64,
    expectation_service: f64,
    second_order_moment_service: f64,
) -> f64 {
    rho * (second_order_moment_service / (2.0 * expectation_service)) / (1.0 - rho)
        + 1.0 / lambda / (1.0 / lambda + expectation_start) * expectation_start
        + expectation_start / (1.0 / lambda + expectation_start) * second_order_moment_start
}

pub fn theoretic_stay_avg_erlang(
    rho: f64,
    expectation_start: f64,
    second_order_moment_start: f64,
    lambda: f64,
    expectation_service: f64,
    second_order_moment_service: f64,
) -> f64 {
    theoretic_expectation_delay(
        rho,
        expectation_start,
        second_order_moment_start,
        lambda,
        expectation_service,
        second_order_moment_service,
    ) + expectation_service
}
