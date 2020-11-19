use std::fmt;

#[derive(Clone)]
pub struct Instrument {
    pub symbol: String,
    pub precision: i8,
    pub name: String,
}

impl Instrument {
    pub fn new(symbol: String, precision: i8, name: String) -> Self {
        Instrument {
            symbol,
            precision,
            name,
        }
    }

    pub fn usd() -> Self {
        Instrument::new(String::from("USD"), 2, String::from("U.S. Dollar"))
    }

    pub fn btc() -> Self {
        Instrument::new(String::from("BTC"), 8, String::from("Bitcoin"))
    }

    pub fn eth() -> Self {
        Instrument::new(String::from("ETH"), 8, String::from("Ether"))
    }

    pub fn xrp() -> Self {
        Instrument::new(String::from("XRP"), 8, String::from("Ripple"))
    }
}

impl fmt::Display for Instrument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl PartialEq for Instrument {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol && self.precision == other.precision && self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_init() {
        let btc = Instrument::btc();
        assert_eq!(btc.symbol, "BTC");
        assert_eq!(btc.precision, 8);
        assert_eq!(btc.name, "Bitcoin");
    }

    #[test]
    fn test_equals() {
        let btc1 = Instrument::btc();
        let btc2 = Instrument::btc();
        assert_eq!(btc1, btc2);

        let btc2 = Instrument::new("ETH".to_string(), 8, "Bitcoin".to_string());
        assert_ne!(btc1, btc2);

        let btc2 = Instrument::new("BTC".to_string(), 5, "Bitcoin".to_string());
        assert_ne!(btc1, btc2);

        let btc2 = Instrument::new("BTC".to_string(), 8, "Ethereum".to_string());
        assert_ne!(btc1, btc2);
    }
}
