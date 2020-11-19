use crate::instruments::Instrument;

pub struct TradingPair {
    pub base: Instrument,
    pub quote: Instrument,
}
