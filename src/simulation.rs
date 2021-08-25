//! The `simulation.rs` contains the struct `Simulation` used to store values of a simulation and
//! some function associated, to compute more metrics.
use crate::measures::Mean;

/// The simulation struct groups valuable information needed for further processing.
///
/// Properties:
///
/// * `arrivals`: List of the clients arrivals, the exact time at which they entered the system.
/// * `delays`: List of delays encountered, for every client entered in the system. If a client did
/// not encounter delay, the value for its index will be `0.0`.
/// * `warmups`: List of the durations spent by clients waiting for the server to warmup. If a
/// client dit not encounter this kind of duration, the value for its index will be `0.0`. It is
/// impossible for a client of index `i` to have both a value in `delays[i]` and `warmups[i]`.
/// * `services`: List of the durations spent by clients during their service time.
/// * `nap_times`: List of duration that the server spent off. These duration are not really linked
/// to any client, because it is a measure of time spent while they were not in the system. Still,
/// this information is useful to calculate the total time that the server spent off line.
/// * `departures`: List of the clients departure, the exact moments at which they left the system.
/// * `total_time`: The total time, from the beginning to the end.
pub struct Simulation {
    arrivals: Vec<f64>,
    delays: Vec<f64>,
    warmups: Vec<f64>,
    services: Vec<f64>,
    nap_times: Vec<f64>,
    // computed
    departures: Vec<f64>,
    total_time: Option<f64>,
}

/// Constructor & static functions
impl Simulation {
    /// To create a new `Simulation` struct, some properties can be computed automatically.
    /// That's the purpose of this constructor function.
    ///
    /// Arguments:
    /// * `arrivals`: List of the clients arrivals, the exact time at which they entered the system.
    /// * `delays`: List of delays encountered, for every client entered in the system. If a client did
    /// not encounter delay, the value for its index will be `0.0`.
    /// * `warmups`: List of the durations spent by clients waiting for the server to warmup. If a
    /// client dit not encounter this kind of duration, the value for its index will be `0.0`. It is
    /// impossible for a client of index `i` to have both a value in `delays[i]` and `warmups[i]`.
    /// * `services`: List of the durations spent by clients during their service time.
    /// * `nap_times`: List of duration that the server spent off. These duration are not really linked
    /// to any client, because it is a measure of time spent while they were not in the system. Still,
    /// this information is useful to calculate the total time that the server spent off line.
    ///
    /// Returns:
    ///
    /// A `Simulation` object (struct).
    pub fn new(
        arrivals: Vec<f64>,
        delays: Vec<f64>,
        warmups: Vec<f64>,
        services: Vec<f64>,
        nap_times: Vec<f64>,
    ) -> Self {
        // sanity check
        assert_eq!(arrivals.len(), delays.len());
        assert_eq!(arrivals.len(), warmups.len());
        assert_eq!(arrivals.len(), services.len());

        let departures = Self::departures(&arrivals, &delays, &warmups, &services);
        let total_time = departures.last().map(|v| v.clone());
        Self {
            arrivals,
            delays,
            warmups,
            services,
            nap_times,
            departures,
            total_time,
        }
    }

    /// Compute when a client left the system.
    /// It's simply the addition of its arrival, the time he waited, server's warmup's time and its
    /// service time.
    ///
    /// Arguments:
    ///
    /// * `arrivals`: The times of arrival of the clients
    /// * `delays`: The durations the clients waited for the previous client to be served.
    /// * `warmups`: The durations the clients waited during the server's warmup.
    /// * `services`: The duration of the service.
    ///
    /// Returns:
    ///
    /// A list of the departures of the clients.
    fn departures(arrivals: &[f64], delays: &[f64], warmups: &[f64], services: &[f64]) -> Vec<f64> {
        arrivals
            .iter()
            .zip(delays)
            .zip(warmups)
            .zip(services)
            .map(|(((&a, &d), &w), &s)| a + d + w + s)
            .collect()
    }
}

/// Methods of the Simulation struct
impl Simulation {
    /// Calculate the mean service time.
    pub fn avg_service(&self) -> f64 {
        self.services.iter().calculate_mean()
    }

    /// The stay time has sometimes the confusing name "waiting time".
    ///
    /// It's the average time that a job spends in the node (delay or startup + service time),
    /// from its arrival to its departure.
    pub fn avg_stay(&self) -> f64 {
        self.arrivals
            .iter()
            .zip(&self.departures)
            .map(|(&a, &d)| d - a)
            .calculate_mean()
    }

    /// Calculate the probability of arriving and finding the server in an active state.
    /// The idea is to return the ratio of the time when the server was on by the total time spent.
    pub fn probability_server_on(&self) -> f64 {
        self.services.iter().sum::<f64>() / self.total_time.unwrap()
    }

    /// Calculate the probability of arriving and finding the server in an warming state.
    /// The idea is to return the ratio of the time when the server was warming up by the total
    /// time spent.
    pub fn probability_server_setup(&self) -> f64 {
        self.warmups.iter().sum::<f64>() / self.total_time.unwrap()
    }

    /// Calculate the probability of arriving and finding the server in an off state.
    /// The idea is to return the ratio of the time when the server was off by the total time spent.
    pub fn probability_server_off(&self) -> f64 {
        self.nap_times.iter().sum::<f64>() / self.total_time.unwrap()
    }

    /// Used to calculate 𝔼\[W²]
    pub fn second_order_moment_waiting_delay(&self) -> f64 {
        self.delays.iter().map(|d| d * d).calculate_mean()
    }
}
