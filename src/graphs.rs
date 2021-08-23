use crate::measures::{theoretic_p_off, theoretic_p_setup, theoretic_stay_avg, Data};
use plotters::prelude::*;

pub fn print_avg_stay_graph(values: &Vec<Data>) -> Result<(), Box<dyn std::error::Error>> {
    let file_name: &'static str = "images/exp_avg_stay_by_rho.png";
    let title: &'static str = "Average stay time by rho";

    let width = 640;
    let height = 480;

    let theoretical_avg_stay = values
        .iter()
        .map(|v| theoretic_stay_avg(v.rho, v.mu, v.theta))
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
            rhos.iter().cloned().zip(theoretical_avg_stay),
            &GREEN,
        ))?
        .label("theoretical 𝔼[S]")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .draw_series(
            values
                .iter()
                .map(|v| Circle::new((v.rho, v.avg_stay_time), 2, RED.filled())),
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

pub fn print_p_setup_graph(values: &Vec<Data>) -> Result<(), Box<dyn std::error::Error>> {
    let file_name: &'static str = "images/exp_p_setup_by_rho.png";
    let title: &'static str = "P(setup) time by rho";

    let width = 640;
    let height = 480;

    let theoretical_p_setup = values
        .iter()
        .map(|v| theoretic_p_setup(v.rho, v.lambda, v.theta))
        .collect::<Vec<_>>();

    let rhos = values.iter().map(|v| v.rho).collect::<Vec<_>>();

    let root = BitMapBackend::new(file_name, (width, height)).into_drawing_area();
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
            rhos.iter().cloned().zip(theoretical_p_setup),
            &GREEN,
        ))?
        .label("theoretical P(setup)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .draw_series(
            values
                .iter()
                .map(|v| Circle::new((v.rho, v.probability_p_setup), 2, RED.filled())),
        )?
        .label("P(setup)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}

pub fn print_p_off_graph(values: &Vec<Data>) -> Result<(), Box<dyn std::error::Error>> {
    let file_name: &'static str = "images/exp_p_off_by_rho.png";
    let title: &'static str = "P(off) time by rho";

    let width = 640;
    let height = 480;

    let theoretical_p_off = values
        .iter()
        .map(|v| theoretic_p_off(v.rho, v.lambda, v.theta))
        .collect::<Vec<_>>();

    let rhos = values.iter().map(|v| v.rho).collect::<Vec<_>>();

    let root = BitMapBackend::new(file_name, (width, height)).into_drawing_area();
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
            rhos.iter().cloned().zip(theoretical_p_off),
            &GREEN,
        ))?
        .label("theoretical P(off)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .draw_series(
            values
                .iter()
                .map(|v| Circle::new((v.rho, v.probability_p_off), 2, RED.filled())),
        )?
        .label("P(off)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}
