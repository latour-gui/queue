use crate::measures::{
    is_inside_interval, test_statistic, theoretic_p_off_exp, theoretic_p_setup_exp,
    theoretic_stay_avg_erlang, theoretic_stay_avg_exp, Data,
};
use plotters::prelude::*;

const HYPOTHESIS_INTERVAL: f64 = 1.96;

pub fn print_avg_stay_graph_for_exp(values: &Vec<Data>) -> Result<(), Box<dyn std::error::Error>> {
    let file_name: &'static str = "images/exp_avg_stay_by_rho.png";
    let title: &'static str = "Average stay time by rho";

    let width = 640;
    let height = 480;

    let theoretical_avg_stay = values
        .iter()
        .map(|v| theoretic_stay_avg_exp(v.rho, v.mu.unwrap(), v.theta))
        .collect::<Vec<_>>();

    let rhos = values.iter().map(|v| v.rho).collect::<Vec<_>>();

    let root = BitMapBackend::new(file_name, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..1f64, 0f64..20f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            rhos.iter().cloned().zip(theoretical_avg_stay.clone()),
            &MAGENTA,
        ))?
        .label("theoretical 𝔼[S]")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

    chart
        .draw_series(
            values
                .iter()
                .map(|v| Circle::new((v.rho, v.avg_stay_time), 2, BLUE.filled())),
        )?
        .label("𝔼[S]")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    // Hypotheses testing
    let file_name: &'static str = "images/exp_avg_stay_by_rho_test.png";

    let root = BitMapBackend::new(file_name, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..1f64, 0f64..20f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            rhos.iter().cloned().zip(theoretical_avg_stay.clone()),
            &MAGENTA,
        ))?
        .label("theoretical 𝔼[S]")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

    // Points that respect the hypotheses
    let t_xs = values
        .iter()
        .zip(&theoretical_avg_stay)
        .map(|(&d, &v)| {
            test_statistic(
                d.avg_stay_time,
                v,
                d.corrected_standard_deviation_avg_stay,
                d.n_simulations,
            )
        })
        .collect::<Vec<f64>>();
    chart
        .draw_series(
            values
                .iter()
                .zip(&t_xs)
                .filter(|(_, &t)| is_inside_interval(t, HYPOTHESIS_INTERVAL))
                .map(|(&d, _)| Circle::new((d.rho, d.avg_stay_time), 2, GREEN.filled())),
        )?
        .label("𝔼[S] verifies H_0")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    // Points that do not respect the hypotheses
    chart
        .draw_series(
            values
                .iter()
                .zip(&t_xs)
                .filter(|(_, &t)| !is_inside_interval(t, HYPOTHESIS_INTERVAL))
                .map(|(&d, _)| Circle::new((d.rho, d.avg_stay_time), 2, RED.filled())),
        )?
        .label("𝔼[S] invalidates H_0")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

pub fn print_avg_stay_graph_for_erlang(
    values: &Vec<Data>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name: &'static str = "images/erlang_avg_stay_by_rho.png";
    let title: &'static str = "Average stay time by rho";

    let width = 640;
    let height = 480;

    let theoretical_avg_stay = values
        .iter()
        .map(|v| theoretic_stay_avg_erlang(v.lambda, v.theta, v.rho, v.k.unwrap(), v.beta.unwrap()))
        .collect::<Vec<_>>();

    let rhos = values.iter().map(|v| v.rho).collect::<Vec<_>>();

    let root = BitMapBackend::new(file_name, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..1f64, 0f64..20f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            rhos.iter().cloned().zip(theoretical_avg_stay.clone()),
            &MAGENTA,
        ))?
        .label("theoretical 𝔼[S]")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

    chart
        .draw_series(
            values
                .iter()
                .map(|v| Circle::new((v.rho, v.avg_stay_time), 2, BLUE.filled())),
        )?
        .label("𝔼[S]")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    // Hypothesis testing

    let file_name: &'static str = "images/erlang_avg_stay_by_rho_test.png";
    let root = BitMapBackend::new(file_name, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..1f64, 0f64..20f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            rhos.iter().cloned().zip(theoretical_avg_stay.clone()),
            &MAGENTA,
        ))?
        .label("theoretical 𝔼[S]")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

    // Points that respect the hypotheses
    let t_xs = values
        .iter()
        .zip(&theoretical_avg_stay)
        .map(|(&d, &v)| {
            test_statistic(
                d.avg_stay_time,
                v,
                d.corrected_standard_deviation_avg_stay,
                d.n_simulations,
            )
        })
        .collect::<Vec<f64>>();

    chart
        .draw_series(
            values
                .iter()
                .zip(&t_xs)
                .filter(|(_, &t)| is_inside_interval(t, HYPOTHESIS_INTERVAL))
                .map(|(&d, _)| Circle::new((d.rho, d.avg_stay_time), 2, GREEN.filled())),
        )?
        .label("𝔼[S]")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .draw_series(
            values
                .iter()
                .zip(&t_xs)
                .filter(|(_, &t)| !is_inside_interval(t, HYPOTHESIS_INTERVAL))
                .map(|(&d, _)| Circle::new((d.rho, d.avg_stay_time), 2, RED.filled())),
        )?
        .label("𝔼[S]")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

pub fn print_p_setup_graph(
    values: &Vec<Data>,
    file_name: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    let real_file_name = file_name.to_owned() + ".png";
    let title: &'static str = "P(setup) time by rho";

    let width = 640;
    let height = 480;

    let theoretical_p_setup = values
        .iter()
        .map(|v| theoretic_p_setup_exp(v.rho, v.lambda, v.theta))
        .collect::<Vec<_>>();

    let rhos = values.iter().map(|v| v.rho).collect::<Vec<_>>();

    let root = BitMapBackend::new(&real_file_name, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            rhos.iter().cloned().zip(theoretical_p_setup.clone()),
            &MAGENTA,
        ))?
        .label("theoretical P(setup)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

    chart
        .draw_series(
            values
                .iter()
                .map(|v| Circle::new((v.rho, v.probability_p_setup), 2, BLUE.filled())),
        )?
        .label("P(setup)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    // Hypotheses testing
    let real_file_name = file_name.to_owned() + "_test.png";
    let root = BitMapBackend::new(&real_file_name, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            rhos.iter().cloned().zip(theoretical_p_setup.clone()),
            &MAGENTA,
        ))?
        .label("theoretical P(setup)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

    let t_xs = values
        .iter()
        .zip(&theoretical_p_setup)
        .map(|(&d, &v)| {
            test_statistic(
                d.probability_p_setup,
                v,
                d.corrected_standard_deviation_p_setup,
                d.n_simulations,
            )
        })
        .collect::<Vec<f64>>();

    chart
        .draw_series(
            values
                .iter()
                .zip(&t_xs)
                .filter(|(_, &t)| is_inside_interval(t, HYPOTHESIS_INTERVAL))
                .map(|(&v, _)| Circle::new((v.rho, v.probability_p_setup), 2, GREEN.filled())),
        )?
        .label("P(setup) verifies H_0")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
    chart
        .draw_series(
            values
                .iter()
                .zip(&t_xs)
                .filter(|(_, &t)| !is_inside_interval(t, HYPOTHESIS_INTERVAL))
                .map(|(&v, _)| Circle::new((v.rho, v.probability_p_setup), 2, RED.filled())),
        )?
        .label("P(setup) invalidates H_0")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

pub fn print_p_off_graph(
    values: &Vec<Data>,
    file_name: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    let title: &'static str = "P(off) time by rho";
    let real_file_name = file_name.to_owned() + ".png";
    let width = 640;
    let height = 480;

    let theoretical_p_off = values
        .iter()
        .map(|v| theoretic_p_off_exp(v.rho, v.lambda, v.theta))
        .collect::<Vec<_>>();

    let rhos = values.iter().map(|v| v.rho).collect::<Vec<_>>();

    let root = BitMapBackend::new(&real_file_name, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            rhos.iter().cloned().zip(theoretical_p_off.clone()),
            &MAGENTA,
        ))?
        .label("theoretical P(off)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

    chart
        .draw_series(
            values
                .iter()
                .map(|v| Circle::new((v.rho, v.probability_p_off), 2, BLUE.filled())),
        )?
        .label("P(off)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    // Hypothesis testing

    let real_file_name = file_name.to_owned() + "_test.png";

    let root = BitMapBackend::new(&real_file_name, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            rhos.iter().cloned().zip(theoretical_p_off.clone()),
            &MAGENTA,
        ))?
        .label("theoretical P(off)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

    let t_xs = values
        .iter()
        .zip(&theoretical_p_off)
        .map(|(&d, &v)| {
            test_statistic(
                d.probability_p_off,
                v,
                d.corrected_standard_deviation_p_off,
                d.n_simulations,
            )
        })
        .collect::<Vec<f64>>();

    chart
        .draw_series(
            values
                .iter()
                .zip(&t_xs)
                .filter(|(_, &t)| is_inside_interval(t, HYPOTHESIS_INTERVAL))
                .map(|(&v, _)| Circle::new((v.rho, v.probability_p_off), 2, GREEN.filled())),
        )?
        .label("P(off) satisfies H_0")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .draw_series(
            values
                .iter()
                .zip(&t_xs)
                .filter(|(_, &t)| !is_inside_interval(t, HYPOTHESIS_INTERVAL))
                .map(|(&v, _)| Circle::new((v.rho, v.probability_p_off), 2, RED.filled())),
        )?
        .label("P(off) invalidates H_0")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}
