use crate::simulation::{ServerState, Simulation};
use std::collections::HashMap;

pub struct Data {
    pub rho: f64,
    pub lambda: f64,
    pub mu: f64,
    pub theta: f64,
    pub avg_stay_time: f64,
    pub probability_p_on: f64,
    pub probability_p_off: f64,
    pub probability_p_setup: f64,
    pub n_simulations: usize,
}

impl Data {
    pub fn new(
        rho: f64,
        lambda: f64,
        mu: f64,
        theta: f64,
        avg_stay_time: f64,
        probability_p_on: f64,
        probability_p_off: f64,
        probability_p_setup: f64,
        n_simulations: usize,
    ) -> Self {
        Self {
            rho,
            lambda,
            mu,
            theta,
            avg_stay_time,
            probability_p_on,
            probability_p_off,
            probability_p_setup,
            n_simulations,
        }
    }
}

pub fn get_probability_entry_state(simulations: &Vec<Simulation>) -> HashMap<ServerState, f64> {
    let triplet = simulations
        .iter()
        .map(|s| {
            (
                s.probability_server_on(),
                s.probability_server_off(),
                s.probability_server_setup(),
            )
        })
        .fold((0f64, 0f64, 0f64), |mut acc, h| {
            acc.0 += h.0;
            acc.1 += h.1;
            acc.2 += h.2;

            acc
        });

    [
        (ServerState::ON, triplet.0 / simulations.len() as f64),
        (ServerState::OFF, triplet.1 / simulations.len() as f64),
        (ServerState::SETUP, triplet.2 / simulations.len() as f64),
    ]
    .iter()
    .cloned()
    .collect()
}

pub fn theoretic_stay_avg(rho: f64, mu: f64, theta: f64) -> f64 {
    (1.0 / mu) / (1.0 - rho) + 1.0 / theta
}

pub fn theoretic_p_off(rho: f64, lambda: f64, theta: f64) -> f64 {
    (1.0 - rho) * (1.0 / lambda) / (1.0 / lambda + 1.0 / theta)
}

pub fn theoretic_p_setup(rho: f64, lambda: f64, theta: f64) -> f64 {
    (1.0 - rho) * (1.0 / theta) / (1.0 / lambda + 1.0 / theta)
}
