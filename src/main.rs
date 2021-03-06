// #![warn(missing_docs)]
//! The `queue` crate is created for the INFO M222 course at the UNamur university.
//! The project consist of a study of a queue simulation

use std::error::Error;
pub mod graphs;
pub mod measures;
pub mod simulation;
pub mod variables;
use crate::graphs::{
    print_avg_stay_graph_for_erlang, print_avg_stay_graph_for_exp, print_p_off_graph,
    print_p_setup_graph,
};
use crate::measures::{corrected_standard_deviation, Data};
use crate::simulation::Simulation;
use crate::variables::{generator, ErlangParameter, ExponentialParameter, PoissonParameter};
use indicatif::ProgressIterator;
use measures::Mean;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use variables::Parameter;

/// Entry point of the program
fn main() {
    let simulations_by_batch = 300; // number of simulations
    let arrivals_number = 400; // number of arrivals in our system
    let rhos = itertools_num::linspace(0.05, 0.95, 50).collect::<Vec<f64>>();

    let theta = 0.6; // parameter for the warmup (setup) random distribution (exp)

    launch_exp(simulations_by_batch, arrivals_number, theta, &rhos);
    launch_erlang(simulations_by_batch, arrivals_number, theta, &rhos);
}

/// This function will launch the experiments linked to the queue having Exponential law describing
/// the service times.
///
/// Arguments:
///  
/// * `simulations_by_batch`: The number of simulations that will be launched for each rho value.
/// * `arrivals_number`: The number of clients that will enter the queue.
/// * `theta`: The parameter of the law describing the setup/warmup times (Exponential)
/// * `rhos`: The values of rho for which we want results
///
/// Return:
///
/// Nothing BUT this function will generate 6 graphs: the average stay in the queue (by rho), the
/// probability of arriving with the server warming up (by rho) and the probability of arriving
/// with the server offline (by rho).
/// Those 3 kind of graphs are declined with the hypothesis testing that the computed point is equal
/// to the theoretical value.
fn launch_exp(simulations_by_batch: usize, arrivals_number: usize, theta: f64, rhos: &[f64]) {
    let lambda = 1.0; // the value for the parameter of the arrival (Poisson)

    let mut values: Vec<Data> = Vec::new();
    for &rho in rhos.iter().progress() {
        let mu = lambda / rho; // value for the service parameter (Exponential)

        // simulations are computed in parallel
        let simulations = (0..=simulations_by_batch)
            .collect::<Vec<_>>()
            .par_iter()
            .map(|_| exp_service_time(arrivals_number, lambda, mu, theta))
            .collect::<Vec<Simulation>>();

        let avg_stay_time = simulations
            .iter()
            .map(|s: &Simulation| s.avg_stay())
            .calculate_mean();

        let probability_p_off = simulations
            .iter()
            .map(|s: &Simulation| s.probability_server_off())
            .calculate_mean();

        let probability_p_setup = simulations
            .iter()
            .map(|s: &Simulation| s.probability_server_setup())
            .calculate_mean();

        // the way to calculate a corrected standard deviation:
        // sqrt( 1 / (n-1) * sum( (?? - ????)?? ) )

        let corrected_standard_deviation_avg_stay = f64::sqrt(
            1.0 / (simulations_by_batch - 1) as f64
                * simulations
                    .iter()
                    .map(|s: &Simulation| (s.avg_stay() - avg_stay_time).powi(2))
                    .sum::<f64>(),
        );
        let corrected_standard_deviation_p_off = f64::sqrt(
            1.0 / (simulations_by_batch - 1) as f64
                * simulations
                    .iter()
                    .map(|s: &Simulation| (s.probability_server_off() - probability_p_off).powi(2))
                    .sum::<f64>(),
        );

        let corrected_standard_deviation_p_setup = f64::sqrt(
            1.0 / (simulations_by_batch - 1) as f64
                * simulations
                    .iter()
                    .map(|s: &Simulation| {
                        (s.probability_server_setup() - probability_p_setup).powi(2)
                    })
                    .sum::<f64>(),
        );

        values.push(Data {
            rho,
            lambda,
            mu: Some(mu),
            k: None,
            beta: None,
            theta,
            avg_stay_time,
            corrected_variance_avg_stay: corrected_standard_deviation_avg_stay,
            probability_p_off,
            corrected_variance_p_off: corrected_standard_deviation_p_off,
            probability_p_setup,
            corrected_variance_p_setup: corrected_standard_deviation_p_setup,
            n_simulations: simulations_by_batch,
        });
    }
    let _ = print_avg_stay_graph_for_exp(&values);
    let _ = print_p_setup_graph(&values, "images/exp_p_setup_by_rho");
    let _ = print_p_off_graph(&values, "images/exp_p_off_by_rho");
}

/// This function will launch the experiments linked to the queue having Exponential law describing
/// the service times.
///
/// Arguments:
///  
/// * `simulations_by_batch`: The number of simulations that will be launched for each rho value.
/// * `arrivals_number`: The number of clients that will enter the queue.
/// * `theta`: The parameter of the law describing the setup/warmup times (Exponential)
/// * `rhos`: The values of rho for which we want results
///
/// Return:
///
/// Nothing BUT this function will generate 6 graphs: the average stay in the queue (by rho), the
/// probability of arriving with the server warming up (by rho) and the probability of arriving
/// with the server offline (by rho).
/// Those 3 kind of graphs are declined with the hypothesis testing that the computed point is equal
/// to the theoretical value.
fn launch_erlang(simulations_by_batch: usize, arrivals_number: usize, theta: f64, rhos: &[f64]) {
    let mut values: Vec<Data> = Vec::new();
    let mut d: Option<f64> = None; // used to compute ????[W??]

    let k: usize = 5; // Erlang shape
    let lambda = 1.0; // Poisson param
    for &rho in rhos.iter().progress() {
        let beta = rho / lambda / k as f64; // Erlang scale -> /!\ rate = 1/beta

        // simulations are computed in parallel
        let simulations = (0..=simulations_by_batch)
            .collect::<Vec<_>>()
            .par_iter()
            .map(|_| erlang_service_time(arrivals_number, lambda, theta, k, beta))
            .collect::<Vec<Simulation>>();

        let avg_stay_time = simulations
            .iter()
            .map(|s: &Simulation| s.avg_stay())
            .calculate_mean();

        let probability_p_off = simulations
            .iter()
            .map(|s: &Simulation| s.probability_server_off())
            .calculate_mean();

        let probability_p_setup = simulations
            .iter()
            .map(|s: &Simulation| s.probability_server_setup())
            .calculate_mean();

        let corrected_standard_deviation_avg_stay = corrected_standard_deviation(
            avg_stay_time,
            &simulations.iter().map(|s| s.avg_stay()).collect::<Vec<_>>(),
        );

        let corrected_standard_deviation_p_off = corrected_standard_deviation(
            probability_p_off,
            &simulations
                .iter()
                .map(|s| s.probability_server_off())
                .collect::<Vec<_>>(),
        );
        let corrected_standard_deviation_p_setup = corrected_standard_deviation(
            probability_p_setup,
            &simulations
                .iter()
                .map(|s| s.probability_server_setup())
                .collect::<Vec<_>>(),
        );
        if rho > 0.5 && d == None {
            d = Some(
                simulations
                    .iter()
                    .map(|s: &Simulation| s.second_order_moment_waiting_delay())
                    .calculate_mean(),
            );
            print!(
                "E[W??] = sum( (W_i - E[W])?? ) = {} ; for rho = {}",
                d.unwrap(),
                rho
            );
        }

        values.push(Data {
            rho,
            lambda,
            mu: None,
            k: Some(k),
            beta: Some(beta),
            theta,
            avg_stay_time,
            corrected_variance_avg_stay: corrected_standard_deviation_avg_stay,
            probability_p_off,
            corrected_variance_p_off: corrected_standard_deviation_p_off,
            probability_p_setup,
            corrected_variance_p_setup: corrected_standard_deviation_p_setup,
            n_simulations: simulations_by_batch,
        });
    }
    let _ = print_avg_stay_graph_for_erlang(&values);
    let _ = print_p_setup_graph(&values, "images/erlang_p_setup_by_rho");
    let _ = print_p_off_graph(&values, "images/erlang_p_off_by_rho");
}

/// Wrapper for the queue function, M/G/1 with service time distributed as exponential
///
/// entry: poisson of parameter lambda
/// service: exponential of parameter mu
/// warmup: exponential of parameter theta
/// n people are allowed to enter the queue
///
/// Arguments:
///
/// * `n`: The number of people that will enter the queue
/// * `lambda`: The parameter used for the Poisson distribution (arrival)
/// * `mu`: The parameter used for the Exponential distribution (service)
/// * `theta`: The parameter used for the Exponential distribution (warmup)
///
/// Returns:
/// A `Simulation` "object" (it is a struct really) containing all the useful information for
/// further calculations.
fn exp_service_time(n: usize, lambda: f64, mu: f64, theta: f64) -> Simulation {
    // sanity check : rho must always be less than one
    assert!(lambda / mu < 1.0);

    let inter_arrival_param = Parameter::Poisson(PoissonParameter { lambda });
    let service_param = Parameter::Exponential(ExponentialParameter { lambda: mu });
    let warming_up_param = Parameter::Exponential(ExponentialParameter { lambda: theta });

    queue(n, inter_arrival_param, service_param, warming_up_param).unwrap()
}

/// Wrapper for the queue function, M/G/1 with service time distributed as erlang.
///
/// entry: poisson of parameter lambda
/// service: erlang of parameters k, beta (beta is scale)
/// warmup: exponential of parameter theta
/// n people are allowed to enter the queue
///
/// Arguments:
///
/// * `n`: The number of people that will enter the queue
/// * `lambda`: The parameter used for the Poisson distribution (arrival)
/// * `theta`: The parameter used for the Exponential distribution (warmup)
/// * `k`: One of the two parameters (shape) used for the Erlang distribution (service)
/// * `beta`: The other parameter (scale) used for the Erlang distribution (service)
///
/// Returns:
/// A `Simulation` "object" (it is a struct really) containing all the useful information for
/// further calculations.
fn erlang_service_time(n: usize, lambda: f64, theta: f64, k: usize, beta: f64) -> Simulation {
    assert!(lambda * k as f64 * beta < 1.0);

    let inter_arrival_param = Parameter::Poisson(PoissonParameter { lambda });
    let service_param = Parameter::Erlang(ErlangParameter { k, beta });
    let warming_up_param = Parameter::Exponential(ExponentialParameter { lambda: theta });

    queue(n, inter_arrival_param, service_param, warming_up_param).unwrap()
}

/// The `queue` function is the core of this project.
///
/// This simulate the arrival of clients, their waiting time and service time.
///
/// Arguments:
///
/// * `n`: The number of jobs that will enter the queue.
/// * `inter_arrival_param`: The parameter of the law that will describe the inter-arrival rate.
/// * `service_param`: The parameter of the law that will describe the services times.
/// * `warming_up_param`: The parameter of the law that will describe the warmup time.
///
/// Returns:
///
/// A `Simulation` "object" (it is a struct really) containing all the useful information for
/// further calculations.
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
        // .map(|x| x*100 as u64)// convert to discrete
        .collect();

    // Since the clients are not allowed to return in the queue by the process, there will be
    // exactly as many service times as there are clients.
    // Since a service time is independent from the moment when a client arrive,
    // we can calculate them all here.
    let service_times: Vec<_> = generator(&service_param, n);

    let mut warmups: Vec<f64> = Vec::new();
    let mut delays: Vec<f64> = Vec::new();
    let mut nap_times: Vec<f64> = Vec::new();

    let mut previous_client_departure = 0.0;
    for (&client_arrival, &service_time) in incoming_clients.iter().zip(&service_times) {
        let delay: f64;
        let nap_time: f64;
        let warmup: f64;

        // If the new client arrive *after* the departure of the last one present in the node, the
        // server has taken a break and needs to be warmed up.
        // If the new client arrive *before* the departure of the last one present in the node, this
        // client has to wait for the server. The server will not take a break.
        if client_arrival > previous_client_departure {
            delay = 0.0;
            nap_time = client_arrival - previous_client_departure; // the nap time is the time that the server spent OFF
            warmup = generator(&warming_up_param, 1)[0]; // asking for only one draw so take the first value of the vector.
        } else {
            delay = previous_client_departure - client_arrival;
            nap_time = 0.0;
            warmup = 0.0;
        }
        warmups.push(warmup);
        delays.push(delay); // warning, the real time waited by clients are `delays.iter().zip(warmups).map(|(d, w)| d+w).collect()`
        nap_times.push(nap_time);

        // the nap time is only relevant to the server, so it's not counted in the sojourn time
        previous_client_departure = client_arrival + delay + warmup + service_time;
    }

    Ok(Simulation::new(
        incoming_clients,
        delays,
        warmups,
        service_times,
        nap_times,
    ))
}
