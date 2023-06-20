use std::error::Error;

use arbitr::datasource::BinanceDatasource;
use arbitr::strategy::FloydWarshallMultiplicativeStrategy;
use arbitr::Arbitr;
use arbitr::ArbitrConfig;

fn main() -> Result<(), Box<dyn Error>> {
    let arbitr_config: ArbitrConfig = ArbitrConfig::default();
    let mut datasource: BinanceDatasource = BinanceDatasource::default();
    let mut strategy: FloydWarshallMultiplicativeStrategy =
        FloydWarshallMultiplicativeStrategy::default();
    let mut arbitr: Arbitr = Arbitr::new(arbitr_config, &mut datasource, &mut strategy);
    arbitr.execute()?;

    Ok(())
}
