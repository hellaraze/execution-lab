use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, f64>,
    pub asks: BTreeMap<OrderedFloat<f64>, f64>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn apply_bid(&mut self, price: f64, qty: f64) {
        let p = OrderedFloat(price);
        if qty == 0.0 {
            self.bids.remove(&p);
        } else {
            self.bids.insert(p, qty);
        }
    }

    pub fn apply_ask(&mut self, price: f64, qty: f64) {
        let p = OrderedFloat(price);
        if qty == 0.0 {
            self.asks.remove(&p);
        } else {
            self.asks.insert(p, qty);
        }
    }

    pub fn apply_levels(&mut self, bids: &[(f64, f64)], asks: &[(f64, f64)]) {
        for (p, q) in bids {
            self.apply_bid(*p, *q);
        }
        for (p, q) in asks {
            self.apply_ask(*p, *q);
        }
    }

    pub fn top_bid(&self) -> Option<(f64, f64)> {
        self.bids.iter().next_back().map(|(p, q)| (p.0, *q))
    }

    pub fn top_ask(&self) -> Option<(f64, f64)> {
        self.asks.iter().next().map(|(p, q)| (p.0, *q))
    }
}

pub mod invariants;
pub mod state_hash;

impl OrderBook {
    pub fn check_invariants(&self) -> Result<(), String> {
        let mut set = invariants::InvariantSet::new();
        set.push(invariants::NoNegativeQty);
        set.push(invariants::NoCross);
        set.run_all(self)
    }
}
