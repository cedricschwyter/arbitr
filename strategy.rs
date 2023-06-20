use std::{collections::HashMap, error::Error};

use crate::{
    graph::{add_edge, floyd_warshall_multiplicative, Graph},
    types::{TradingAsset, TradingPair},
};

pub trait ArbitrageStrategy<'a> {
    // updates new trading data
    fn refresh(&mut self, pairs: Vec<TradingPair<'a>>) -> Result<(), Box<dyn Error>>;

    // computes next step in arbitrage strategy
    fn compute(&self) -> Result<(), Box<dyn Error>>;
}

pub struct FloydWarshallMultiplicativeStrategy<'a> {
    graph: Graph<TradingAsset<'a>, TradingPair<'a>>,
}

impl<'a> Default for FloydWarshallMultiplicativeStrategy<'a> {
    fn default() -> Self {
        let graph: Graph<TradingAsset<'a>, TradingPair<'a>> = HashMap::new();
        FloydWarshallMultiplicativeStrategy { graph }
    }
}

impl FloydWarshallMultiplicativeStrategy<'_> {
    pub fn new() -> Box<Self> {
        Box::new(Self::default())
    }
}

impl<'a> ArbitrageStrategy<'a> for FloydWarshallMultiplicativeStrategy<'a> {
    fn refresh(&mut self, pairs: Vec<TradingPair<'a>>) -> Result<(), Box<dyn Error>> {
        pairs
            .into_iter()
            .for_each(|p| add_edge(&mut self.graph, p.quote_asset, p.base_asset, p));

        Ok(())
    }

    fn compute(&self) -> Result<(), Box<dyn Error>> {
        let result = floyd_warshall_multiplicative(&self.graph);

        for (k, v) in result {
            let value = *v.get(&k).unwrap();
            if value.price < 1.0 && value.price > 0.0 {
                dbg!((k, v.get(&k).unwrap()));
            }
        }

        Ok(())
    }
}
