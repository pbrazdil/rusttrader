use oms;
use oms::instruments;

fn main() {
    let _c = oms::instruments::Instrument::usd();
    let a = instruments::Instrument::usd();
    println!("{}", a);
    let a2 = instruments::Instrument::usd();
    let b2 = instruments::Instrument::btc();
    println!("true: {}, false: {}, true: {}", a == a2, a != a2, a != b2);
}
