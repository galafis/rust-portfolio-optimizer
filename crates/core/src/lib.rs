use nalgebra::{DMatrix, DVector};
use rpo_data::load_historical_data;
use rpo_optimizer::{calculate_returns, optimize_portfolio};
use anyhow::Result;

pub fn run_optimization(path: &str) -> Result<(DVector<f64>, Vec<(f64, f64)>)> {
    let df = load_historical_data(path)?;
    let prices = df.select(["AAPL", "MSFT", "GOOG"])?;
    
    let nrows = prices.height();
    let ncols = prices.width();
    let data: Vec<f64> = prices
        .get_columns()
        .iter()
        .flat_map(|s| s.f64().unwrap().into_no_null_iter())
        .collect();

    let prices_matrix = DMatrix::from_vec(nrows, ncols, data);

    let returns = calculate_returns(&prices_matrix);
    let (best_weights, portfolios) = optimize_portfolio(&returns);

    Ok((best_weights, portfolios))
}

