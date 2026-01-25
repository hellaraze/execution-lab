#[derive(Debug, Clone, Copy)]
pub struct Fees {
    pub maker: f64,
    pub taker: f64,
    pub rebate: f64,
}

impl Fees {
    pub fn effective(&self, is_maker: bool) -> f64 {
        if is_maker {
            self.maker - self.rebate
        } else {
            self.taker
        }
    }
}
