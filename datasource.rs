use std::{collections::HashMap, error::Error};

use binance::{api::Binance, general::General, market::Market, model::ExchangeInformation};

use crate::types::{TradingAsset, TradingPair};

pub trait Datasource<'a> {
    fn init(&'a mut self) -> Result<(), Box<dyn Error>>;
    fn pairs(&'a self) -> Result<Vec<TradingPair<'a>>, Box<dyn Error>>;
}

pub struct BinanceDatasource<'a> {
    general: General,
    market: Market,
    exchange_info: ExchangeInformation,
    symbols: Vec<(String, String, String)>,
    pairs: HashMap<&'a str, (TradingAsset<'a>, TradingAsset<'a>)>,
}

impl<'a> Default for BinanceDatasource<'a> {
    fn default() -> Self {
        let general: General = Binance::new(None, None);
        let market: Market = Binance::new(None, None);

        let exchange_info: ExchangeInformation = general.exchange_info().unwrap();
        let pairs: HashMap<&'a str, (TradingAsset<'a>, TradingAsset<'a>)> = HashMap::new();
        let mut symbols: Vec<(String, String, String)> = vec![];
        exchange_info
            .symbols
            .clone()
            .into_iter()
            .for_each(|s| symbols.push((s.symbol, s.base_asset, s.quote_asset)));
        let datasource = BinanceDatasource {
            general,
            market,
            exchange_info,
            symbols,
            pairs,
        };

        datasource
    }
}

impl BinanceDatasource<'_> {
    pub fn new() -> Box<Self> {
        Box::new(Self::default())
    }
}

impl<'a> Datasource<'a> for BinanceDatasource<'a> {
    fn init(&'a mut self) -> Result<(), Box<dyn Error>> {
        for symbol in &self.symbols {
            self.pairs.insert(
                &symbol.0.as_str(),
                (
                    TradingAsset::new(&symbol.1.as_str()),
                    TradingAsset::new(&symbol.2.as_str()),
                ),
            );
        }

        Ok(())
    }

    fn pairs(&self) -> Result<Vec<TradingPair<'a>>, Box<dyn Error>> {
        todo!()
    }
}
