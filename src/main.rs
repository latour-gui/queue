// #![warn(missing_docs)]
//! The `queue` crate is created for the INFO M222 course at the UNamur university.
//! The project consist of a study of a queue simulation

use std::error::Error;
pub mod graphs;
pub mod measures;
pub mod simulation;
pub mod variables;

use crate::measures::Data;
use crate::simulation::Simulation;
use crate::variables::{generator, ErlangParameter, ExponentialParameter, PoissonParameter};

use crate::graphs::{print_avg_stay_graph, print_p_off_graph, print_p_setup_graph};
use indicatif::ProgressIterator;
use rayon::prelude::*;
use variables::Parameter;

/// Entry point of the program
fn main() {
    let simulations_by_batch = 100; // number of simulations
    let arrivals_number = 1000; // number of arrivals in our system
    let theta = 0.4; // parameter for the warmup (setup) random distribution (exp)

    let rhos = itertools_num::linspace(0.05, 0.95, 50).collect::<Vec<f64>>();
    let lambda = 1.0;

    let mut values: Vec<Data> = Vec::new();
    for &rho in rhos.iter().progress() {
        let mu = lambda / rho;

        // simulations are computed in parallel
        let simulations = (0..=simulations_by_batch)
            .collect::<Vec<_>>()
            .par_iter()
            .map(|_| exp_service_time(arrivals_number, lambda, mu, theta))
            .collect::<Vec<Simulation>>();

        let avg_stay_time =
            simulations.par_iter().map(|s| s.avg_stay()).sum::<f64>() / simulations.len() as f64;

        values.push(Data::new(
            rho,
            lambda,
            mu,
            theta,
            avg_stay_time,
            simulations
                .iter()
                .map(|s: &Simulation| s.probability_server_on())
                .sum::<f64>()
                / simulations.len() as f64,
            simulations
                .iter()
                .map(|s: &Simulation| s.probability_server_off())
                .sum::<f64>()
                / simulations.len() as f64,
            simulations
                .iter()
                .map(|s: &Simulation| s.probability_server_setup())
                .sum::<f64>()
                / simulations.len() as f64,
            simulations_by_batch,
        ));
    }
    let _ = print_avg_stay_graph(&values);
    let _ = print_p_setup_graph(&values);
    let _ = print_p_off_graph(&values);
}

///
fn exp_service_time(n: usize, lambda: f64, mu: f64, theta: f64) -> Simulation {
    // sanity check : roh must always be less than one
    assert!(lambda / mu < 1.0);

    let inter_arrival_param = Parameter::Poisson(PoissonParameter { lambda });
    let service_param = Parameter::Exponential(ExponentialParameter { lambda: mu });
    let warming_up_param = Parameter::Exponential(ExponentialParameter { lambda: theta });

    queue(n, inter_arrival_param, service_param, warming_up_param).unwrap()
}

fn general_start_and_service_time(n: usize) {
    let inter_arrival_param = Parameter::Poisson(PoissonParameter { lambda: 2.0 });
    let service_param = Parameter::Erlang(ErlangParameter { k: 3, mu: 2.0 });
    let warming_up_param = Parameter::Exponential(ExponentialParameter { lambda: 2.0 });

    queue(n, inter_arrival_param, service_param, warming_up_param).unwrap();
}

/// The `queue` function is the core of this project.
///
/// This simulate the arrival of clients, their waiting time and service time.
fn queue(
    n: usize,
    inter_arrival_param: Parameter,
    service_param: Parameter,
    warming_up_param: Parameter,
) -> Result<Simulation, Box<dyn Error>> {
    let mut accumulator = 0.0;
    let inter_arrival_times = generator(&inter_arrival_param, n);
    let incoming_clients: Vec<_> = inter_arrival_times
        .iter()
        .map(|x| {
            accumulator += x;
            accumulator
        }) // instead of manipulating the time between 2 arrivals, we get the arrivals time
        // .map(|x| x as u64)// convert to discrete
        .collect();

    // Since the clients are not allowed to return in the queue by the process, there will be
    // exactly as many service times as there are clients.
    // Since a service time is independent from the moment when a client arrive,
    // we can calculate them all here.
    let service_times: Vec<_> = generator(&service_param, n);

    let mut warmups: Vec<f64> = Vec::new();
    let mut delays: Vec<f64> = Vec::new();
    let mut nap_times: Vec<f64> = Vec::new();

    let mut last_departure = 0.0;
    for (&client_arrival, service_time) in incoming_clients.iter().zip(&service_times) {
        let delay: f64;
        let nap_time: f64;
        let warmup: f64;

        // If the new client arrive *after* the departure of the last one present in the node, the
        // server has taken a break and needs to be warmed up.
        // If the new client arrive *before* the departure of the last one present in the node, this
        // client has to wait for the server. The server will not take a break.
        if client_arrival > last_departure {
            delay = 0.0;
            nap_time = client_arrival - last_departure; // the nap time is the time that the server spent OFF
            warmup = generator(&warming_up_param, 1)[0]; // asking for only one draw so take the first value of the vector.
        } else {
            delay = last_departure - client_arrival;
            nap_time = 0.0;
            warmup = 0.0;
        }
        warmups.push(warmup);
        delays.push(delay); // warning, the real time waited by clients are `delays.iter().zip(warmups).map(|(d, w)| d+w).collect()`
        nap_times.push(nap_time);

        last_departure = client_arrival + delay + warmup + service_time;
    }

    Ok(Simulation::new(
        incoming_clients,
        delays,
        warmups,
        service_times,
        nap_times,
    ))
}
