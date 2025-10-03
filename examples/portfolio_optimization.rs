use rpo_core::run_optimization;
use rpo_utils::{setup_logger, plot_efficient_frontier};
use anyhow::Result;

fn main() -> Result<()> {
    setup_logger();

    let data_path = "data/historical_prices.csv";
    let (best_weights, portfolios) = run_optimization(data_path)?;

    println!("Pesos Ótimos: {:?}", best_weights);

    let sharpe_portfolio = portfolios.iter().max_by(|a, b| (a.1 / a.0).partial_cmp(&(b.1 / b.0)).unwrap()).unwrap().clone();

    plot_efficient_frontier("docs/efficient_frontier.png", &portfolios, sharpe_portfolio)?;
    println!("Gráfico da fronteira eficiente salvo em docs/efficient_frontier.png");

    Ok(())
}

