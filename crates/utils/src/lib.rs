use log::info;
use plotters::prelude::*;
use anyhow::Result;

pub fn setup_logger() {
    env_logger::init();
    info!("Logger initialized.");
}

pub fn plot_efficient_frontier(
    path: &str,
    portfolios: &[(f64, f64)],
    sharpe_portfolio: (f64, f64),
) -> Result<()> {
    let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let (max_vol, max_ret) = portfolios.iter().fold((0.0, 0.0), |(v, r): (f64, f64), (vol, ret)| {
        (v.max(*vol), r.max(*ret))
    });

    let mut chart = ChartBuilder::on(&root)
        .caption("Efficient Frontier", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..max_vol * 1.1, 0f64..max_ret * 1.1)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(PointSeries::of_element(
        portfolios.iter().map(|(vol, ret)| (*vol, *ret)),
        5,
        &BLUE,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
        },
    ))?;

    chart.draw_series(PointSeries::of_element(
        vec![sharpe_portfolio],
        10,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)
                + Circle::new((0,0),s,st.filled())
        },
    ))?;

    root.present()?;
    Ok(())
}

