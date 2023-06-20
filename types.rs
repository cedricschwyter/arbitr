use std::{cmp::Ordering, ops::Mul};

use num_traits::One;

#[derive(Eq, Hash, Debug, Clone, Copy, PartialEq)]
pub struct TradingAsset<'a> {
    symbol: &'a str,
}

impl TradingAsset<'_> {
    pub fn new(symbol: &str) -> TradingAsset {
        TradingAsset { symbol }
    }
}

impl<'a> Default for TradingAsset<'a> {
    fn default() -> Self {
        TradingAsset { symbol: "" }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TradingPair<'a> {
    pub base_asset: TradingAsset<'a>,
    pub quote_asset: TradingAsset<'a>,
    pub price: f64,
}

impl<'a> Default for TradingPair<'a> {
    fn default() -> Self {
        TradingPair {
            base_asset: TradingAsset::default(),
            quote_asset: TradingAsset::default(),
            price: 0.0f64,
        }
    }
}

impl<'a> PartialOrd for TradingPair<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.price < other.price {
            return Some(Ordering::Less);
        } else if self.price > other.price {
            return Some(Ordering::Greater);
        }
        Some(Ordering::Equal)
    }
}

impl<'a> Mul for TradingPair<'a> {
    type Output = TradingPair<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        TradingPair {
            base_asset: self.base_asset,
            quote_asset: rhs.quote_asset,
            price: self.price * rhs.price,
        }
    }
}

impl<'a> One for TradingPair<'a> {
    fn one() -> Self {
        let mut default = TradingPair::default();
        default.price = 1.0f64;
        default
    }
}
