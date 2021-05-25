use rand::prelude::*;

use rand::Error;
use rand_distr::{Exp, Pareto, Poisson};

fn main() {
    n = 1000;
    client_param = 2.0; // lambda param for the Poisson distribution
    service_param = 2.0; // ??
    warmup_param = 2.0; // shape parameter (scale parameter is fixed)

    queue(n, client_param, service_param, warmump_param).expect("ah");
}

fn queue(n: usize, client_param: f64, service_param: f64, warmup_param: f64) -> Result<(), Error> {
    let mut accumulator = 0.0;
    let incoming_clients: Vec<_> = Poisson::new(client_param)?
        .sample_iter(&mut rand::thread_rng()) // generator following a poisson distribution
        .take(n) // iter n times (generate n values)
        .map(|x| {
            accumulator += x;
            accumulator
        }) // instead of manipulating the time between 2 arrivals, we get the arrivals time
        // .map(|x| x as u64)// convert to discrete
        .collect();

    // Since the clients are not allowed to return in the queue by the process, there will be
    // exactly as many service times as there are clients.
    // Since a service time is independent from the other, we can calculate them all here.
    let service_times: Vec<_> = Exp::new(service_param)?
        .sample_iter(&mut rand::thread_rng())
        .take(n)
        .collect();

    // Each server warmup time will be generated by this code.
    // The server has to warm up every time it has to do a pause because there is no client to chain.
    let warmup = Pareto::new(1.0, warmup_param)?.sample();

    let mut warmups: Vec<f64> = Vec::new();
    let mut delays: Vec<f64> = Vec::new();

    let mut last_departure = 0.0;
    for (&client_arrival, service_time) in incoming_clients.iter().zip(service_times) {
        let delay: f64;
        let warmup: f64;

        // If the new client arrive *after* the departure of the previous one, the server has taken
        // a break and needs to be warmed up.
        // If the new client arrive *before* the departure of the previous one, this client has to
        // wait for the server. The server will not take a break.
        if client_arrival > last_departure {
            delay = 0.0;
            warmup = Pareto::new(1.0, warmup_param).unwrap().sample();
        } else {
            delay = last_departure - client_arrival;
            warmup = 0.0;
        }
        warmups.push(warmup);
        delays.push(delay);

        last_departure = client_arrival + delay + warmup + service_time;
    }

    Ok(())
}