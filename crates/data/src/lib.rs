use polars::prelude::*;
use anyhow::Result;

pub fn load_historical_data(path: &str) -> Result<DataFrame> {
    let df = CsvReader::from_path(path)?.finish()?;
    Ok(df)
}

