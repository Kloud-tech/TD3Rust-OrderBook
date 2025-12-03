
use crate::interfaces::{OrderBook, Price, Quantity, Side, Update};
use std::collections::BTreeMap;

pub struct OrderBookImpl {
    bids: BTreeMap<Price, Quantity>,
    asks: BTreeMap<Price, Quantity>,
}

impl OrderBook for OrderBookImpl {
    fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    fn apply_update(&mut self, update: Update) {
        match update {
            Update::Set {
                price,
                quantity,
                side,
            } => {
                let book = match side {
                    Side::Bid => &mut self.bids,
                    Side::Ask => &mut self.asks,
                };

                if quantity == 0 {
                    book.remove(&price);
                } else {
                    book.insert(price, quantity);
                }
            }
            Update::Remove { price, side } => {
                let book = match side {
                    Side::Bid => &mut self.bids,
                    Side::Ask => &mut self.asks,
                };
                book.remove(&price);
            }
        }
    }

    fn get_spread(&self) -> Option<Price> {
        match (self.get_best_ask(), self.get_best_bid()) {
            (Some(ask), Some(bid)) => Some(ask - bid),
            _ => None,
        }
    }

    fn get_best_bid(&self) -> Option<Price> {
        self.bids.keys().next_back().copied()
    }

    fn get_best_ask(&self) -> Option<Price> {
        self.asks.keys().next().copied()
    }

    fn get_quantity_at(&self, price: Price, side: Side) -> Option<Quantity> {
        match side {
            Side::Bid => self.bids.get(&price).copied(),
            Side::Ask => self.asks.get(&price).copied(),
        }
    }

    fn get_top_levels(&self, side: Side, n: usize) -> Vec<(Price, Quantity)> {
        let mut out = Vec::with_capacity(n);

        match side {
            Side::Bid => {
                for (price, qty) in self.bids.iter().rev().take(n) {
                    out.push((*price, *qty));
                }
            }
            Side::Ask => {
                for (price, qty) in self.asks.iter().take(n) {
                    out.push((*price, *qty));
                }
            }
        }

        out
    }

    fn get_total_quantity(&self, side: Side) -> Quantity {
        match side {
            Side::Bid => self.bids.values().sum(),
            Side::Ask => self.asks.values().sum(),
        }
    }
}
