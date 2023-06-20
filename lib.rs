pub mod datasource;
pub mod graph;
pub mod strategy;
pub mod types;

use std::error::Error;

use clap::Parser;
use strategy::ArbitrageStrategy;

use crate::datasource::Datasource;

#[derive(Parser)]
pub struct ArbitrConfig {}

impl Default for ArbitrConfig {
    fn default() -> Self {
        ArbitrConfig {}
    }
}

pub struct Arbitr<'a> {
    pub config: ArbitrConfig,
    pub datasource: &'a mut dyn Datasource<'a>,
    pub strategy: &'a mut dyn ArbitrageStrategy<'a>,
}

impl<'a> Arbitr<'a> {
    pub fn new(
        config: ArbitrConfig,
        datasource: &'a mut dyn Datasource<'a>,
        strategy: &'a mut dyn ArbitrageStrategy<'a>,
    ) -> Arbitr<'a> {
        Arbitr {
            config,
            datasource,
            strategy,
        }
    }

    pub fn execute(&'a mut self) -> Result<(), Box<dyn Error>> {
        self.datasource.init()?;
        loop {
            let pairs = self.datasource.pairs()?;
            self.strategy.refresh(pairs)?;
            self.strategy.compute()?;
            // TODO: have self.strategy.compute() return some sort of intermediate
            // trade instruction that will then be executed via self.datasource at
            // the end of this loop
        }
    }
}
