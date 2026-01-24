use crate::OrderBook;

pub trait Invariant<S> {
    fn name(&self) -> &'static str;
    fn check(&self, s: &S) -> Result<(), String>;
}

pub struct InvariantSet<S> {
    checks: Vec<Box<dyn Invariant<S>>>,
}

impl<S> InvariantSet<S> {
    pub fn new() -> Self {
        Self { checks: Vec::new() }
    }

    pub fn push<I: Invariant<S> + 'static>(&mut self, inv: I) {
        self.checks.push(Box::new(inv));
    }

    pub fn run_all(&self, s: &S) -> Result<(), String> {
        for c in &self.checks {
            c.check(s)
                .map_err(|e| format!("{} failed: {}", c.name(), e))?;
        }
        Ok(())
    }
}

/* ================= INVARIANTS ================= */

pub struct NoNegativeQty;

impl Invariant<OrderBook> for NoNegativeQty {
    fn name(&self) -> &'static str {
        "NoNegativeQty"
    }

    fn check(&self, book: &OrderBook) -> Result<(), String> {
        for (p, q) in book.bids.iter().chain(book.asks.iter()) {
            if !p.0.is_finite() {
                return Err(format!("price not finite: {}", p.0));
            }
            if !q.is_finite() {
                return Err(format!("qty not finite at price {}", p.0));
            }
            if *q < 0.0 {
                return Err(format!("negative qty {} at price {}", q, p.0));
            }
        }
        Ok(())
    }
}

pub struct NoCross;

impl Invariant<OrderBook> for NoCross {
    fn name(&self) -> &'static str {
        "NoCross"
    }

    fn check(&self, book: &OrderBook) -> Result<(), String> {
        let bid = book.top_bid().map(|(p, _q)| p);
        let ask = book.top_ask().map(|(p, _q)| p);

        if let (Some(b), Some(a)) = (bid, ask) {
            if b > a {
                return Err(format!("crossed book: best_bid={} > best_ask={}", b, a));
            }
        }
        Ok(())
    }
}

impl<S> Default for InvariantSet<S> {
    fn default() -> Self {
        Self::new()
    }
}
