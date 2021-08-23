pub struct Simulation {
    arrivals: Vec<f64>,
    delays: Vec<f64>,
    warmups: Vec<f64>,
    services: Vec<f64>,
    nap_times: Vec<f64>,
    // computed
    arrival_states: Vec<ServerState>,
    inter_arrivals: Vec<f64>,
    departures: Vec<f64>,
    total_time: Option<f64>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ServerState {
    ON,
    OFF,
    SETUP,
}

impl Simulation {
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

        let arrival_states = Self::arrival_states(&arrivals, &delays, &warmups, &services);
        let inter_arrivals = Self::inter_arrivals(&arrivals);
        let departures = Self::departures(&arrivals, &delays, &warmups, &services);
        let total_time = departures.last().map(|v| v.clone());
        Self {
            arrivals,
            delays,
            warmups,
            services,
            nap_times,
            arrival_states,
            inter_arrivals,
            departures,
            total_time,
        }
    }

    fn arrival_states(
        arrivals: &[f64],
        delays: &[f64],
        warmups: &[f64],
        services: &[f64],
    ) -> Vec<ServerState> {
        let mut inter_warmups: Vec<(f64, f64)> = Vec::new();
        let mut inter_services: Vec<(f64, f64)> = Vec::new();

        for (((&a, &d), &w), &s) in arrivals.iter().zip(delays).zip(warmups).zip(services) {
            inter_warmups.push((a + d, a + d + w));
            inter_services.push((a + d + w, a + d + w + s));
        }

        arrivals
            .iter()
            .map(|a| {
                if inter_services
                    .iter()
                    .any(|(begin, end)| begin < a && a <= end)
                {
                    ServerState::ON
                } else if inter_warmups
                    .iter()
                    .any(|(begin, end)| begin < a && a <= end)
                {
                    ServerState::SETUP
                } else {
                    ServerState::OFF
                }
            })
            .collect()
    }

    fn inter_arrivals(arrivals: &[f64]) -> Vec<f64> {
        let mut last_arrival = 0.0;
        arrivals
            .iter()
            .map(|&a| {
                let tmp = a - last_arrival;
                last_arrival = a;
                tmp
            })
            .collect()
    }

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

impl Simulation {
    pub fn avg_inter_arrival(&self) -> f64 {
        self.inter_arrivals.iter().sum::<f64>() / self.inter_arrivals.len() as f64
    }

    fn avg_delay(&self) -> f64 {
        // todo: check if the delay avg is really calculated only with the non null delays
        let (sum, count): (f64, usize) = self
            .delays
            .iter()
            .filter(|&&x| x > 0f64)
            .fold((0f64, 0usize), |(acc, count), &v| (acc + v, count + 1));

        sum / count as f64
    }

    fn avg_warmup(&self) -> f64 {
        // todo: check if the warmups avg is really calculated only with the non null delays
        let (sum, count): (f64, usize) = self
            .warmups
            .iter()
            .filter(|&&x| x > 0f64)
            .fold((0f64, 0usize), |(acc, count), v| (acc + v, count + 1));

        sum / count as f64
    }

    pub fn avg_service(&self) -> f64 {
        self.services.iter().sum::<f64>() / self.services.len() as f64
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
            .sum::<f64>()
            / self.arrivals.len() as f64
    }

    fn count_arrival_state(&self, state: &ServerState) -> usize {
        self.arrival_states.iter().filter(|&s| s == state).count()
    }

    pub fn probability_server_on(&self) -> f64 {
        self.services.iter().sum::<f64>() / self.total_time.unwrap()
    }

    pub fn probability_server_setup(&self) -> f64 {
        self.warmups.iter().sum::<f64>() / self.total_time.unwrap()
    }

    pub fn probability_server_off(&self) -> f64 {
        self.nap_times.iter().sum::<f64>() / self.total_time.unwrap()
    }
}
