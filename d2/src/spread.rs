#[derive(Debug, Clone, Copy)]
pub struct SpreadInput {
    pub buy_price: f64,
    pub sell_price: f64,
    pub buy_is_maker: bool,
    pub sell_is_maker: bool,
}
