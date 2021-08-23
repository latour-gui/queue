//! The `variable.rs` file contains the boilerplate needed to generate values according to
//! a specific distribution.

use rand::distributions::Distribution;
use rand_distr::{Exp, Gamma, Poisson};

/// The generator will generate values according to a specific distribution.
/// The distribution is specified through the `parameter` variable, which can only be certain type.
pub fn generator(parameter: &Parameter, n: usize) -> Vec<f64> {
    match parameter {
        Parameter::Poisson(p) => {
            Poisson::new(p.lambda) // instantiate a Poisson distribution generator
                .unwrap()
                .sample_iter(&mut rand::thread_rng()) // generator following a poisson distribution
                .take(n) // iter n times (generate n values)
                .collect()
        } // collect values into a nice `Vec`tor
        Parameter::Exponential(p) => Exp::new(p.lambda)
            .unwrap()
            .sample_iter(&mut rand::thread_rng())
            .take(n)
            .collect(),
        Parameter::Erlang(p) => Gamma::new(p.k as f64, p.beta)
            .unwrap()
            .sample_iter(&mut rand::thread_rng())
            .take(n)
            .collect(),
    }
}

/// Container for a Poisson random variable parameter
pub struct PoissonParameter {
    /// Shape parameter for a Poisson distribution
    pub lambda: f64,
}

/// Container for an Exponential random variable parameter
pub struct ExponentialParameter {
    /// Shape parameter for an Exponential distribution
    pub lambda: f64,
}

pub struct ErlangParameter {
    /// Shape parameter for an Erlang distribution
    pub k: usize,
    /// Scale parameter for an Erlang distribution. The scale beta = 1/lambda, with a rate lambda.
    pub beta: f64,
}

/// Enumeration of available parameter types
/// This allows us to use rust strong typing checking system
pub enum Parameter {
    Poisson(PoissonParameter),
    Exponential(ExponentialParameter),
    Erlang(ErlangParameter),
}
