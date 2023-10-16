pub mod errors;

use errors::NumericLiteralError;
use std::str::FromStr;

fn process(s: &str) {
    let s = s[2..].replace('_', "");
    let n = f64::from_str(&s);
}