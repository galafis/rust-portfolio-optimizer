use nalgebra::{DMatrix, DVector};
use statrs::statistics::Statistics;

pub fn calculate_returns(prices: &DMatrix<f64>) -> DMatrix<f64> {
    let mut returns = DMatrix::from_element(prices.nrows() - 1, prices.ncols(), 0.0);
    for j in 0..prices.ncols() {
        for i in 1..prices.nrows() {
            returns[(i - 1, j)] = (prices[(i, j)] / prices[(i - 1, j)]) - 1.0;
        }
    }
    returns
}

pub fn portfolio_performance(weights: &DVector<f64>, mean_returns: &DVector<f64>, cov_matrix: &DMatrix<f64>) -> (f64, f64) {
    let returns = (weights.transpose() * mean_returns)[0];
    let volatility = (weights.transpose() * cov_matrix * weights)[0].sqrt();
    (returns, volatility)
}

pub fn optimize_portfolio(returns: &DMatrix<f64>) -> (DVector<f64>, Vec<(f64, f64)>) {
    let num_assets = returns.ncols();
    let mean_returns = DVector::from_iterator(num_assets, returns.column_iter().map(|c| c.mean()));
    let cov_matrix = Covariance::covariance(returns);

    let mut portfolio_returns = Vec::new();
    let mut portfolio_volatilities = Vec::new();
    let mut max_sharpe_ratio = 0.0;
    let mut best_weights = DVector::from_element(num_assets, 0.0);

    for _ in 0..10000 {
        let mut weights = DVector::from_iterator(num_assets, (0..num_assets).map(|_| rand::random::<f64>()));
        weights /= weights.sum();

        let (ret, vol) = portfolio_performance(&weights, &mean_returns, &cov_matrix);
        let sharpe_ratio = ret / vol;

        portfolio_returns.push(ret);
        portfolio_volatilities.push(vol);

        if sharpe_ratio > max_sharpe_ratio {
            max_sharpe_ratio = sharpe_ratio;
            best_weights = weights;
        }
    }

    let portfolios = portfolio_volatilities.into_iter().zip(portfolio_returns).collect();

    (best_weights, portfolios)
}

trait Covariance {
    fn covariance(&self) -> DMatrix<f64>;
}

impl Covariance for DMatrix<f64> {
    fn covariance(&self) -> DMatrix<f64> {
        let (rows, cols) = self.shape();
        let mut cov = DMatrix::from_element(cols, cols, 0.0);
        let means = self.column_iter().map(|c| c.mean()).collect::<Vec<f64>>();

        for i in 0..cols {
            for j in 0..cols {
                let mut sum = 0.0;
                for r in 0..rows {
                    sum += (self[(r, i)] - means[i]) * (self[(r, j)] - means[j]);
                }
                cov[(i, j)] = sum / (rows as f64 - 1.0);
            }
        }
        cov
    }
}

