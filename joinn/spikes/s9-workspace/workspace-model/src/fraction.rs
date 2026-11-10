//! Exact fractions. Typing `1.2` means 6/5. A value is shown as a decimal.

use num_rational::Rational64;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A rational number, stored in lowest terms.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Fraction(Rational64);

impl Fraction {
    pub fn new(numer: i64, denom: i64) -> Option<Self> {
        if denom == 0 {
            return None;
        }
        Some(Self(Rational64::new(numer, denom)))
    }

    pub fn from_i64(n: i64) -> Self {
        Self(Rational64::from_integer(n))
    }

    /// `1.2` → 6/5, `24` → 24/1, `92.8` → 464/5. A blank or junk string is `None`.
    pub fn parse_decimal(text: &str) -> Option<Self> {
        let text = text.trim();
        if text.is_empty() {
            return None;
        }
        let (neg, rest) = if let Some(rest) = text.strip_prefix('-') {
            (true, rest.trim())
        } else {
            (false, text)
        };
        let (whole, frac) = match rest.split_once('.') {
            Some((whole, frac)) => {
                if whole.is_empty() || frac.is_empty() {
                    return None;
                }
                if !frac.bytes().all(|b| b.is_ascii_digit()) {
                    return None;
                }
                (whole, frac)
            }
            None => (rest, ""),
        };
        if whole.is_empty() || !whole.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let digits = format!("{whole}{frac}");
        let mut numer: i64 = digits.parse().ok()?;
        let mut denom: i64 = 1;
        for _ in 0..frac.len() {
            denom = denom.checked_mul(10)?;
        }
        if neg {
            numer = numer.checked_neg()?;
        }
        Self::new(numer, denom)
    }

    pub fn numer(&self) -> i64 {
        *self.0.numer()
    }

    pub fn denom(&self) -> i64 {
        *self.0.denom()
    }

    /// Lowest terms, as `72/5` or `24` when the denominator is 1.
    pub fn exact(&self) -> String {
        if self.denom() == 1 {
            self.numer().to_string()
        } else {
            format!("{}/{}", self.numer(), self.denom())
        }
    }

    /// A decimal a person can read. Terminating values are exact (`14.4`).
    /// Others stop at eight places, rounded.
    pub fn decimal(&self) -> String {
        let neg = self.numer() < 0;
        let numer = self.numer().unsigned_abs() as u128;
        let denom = self.denom().unsigned_abs() as u128;
        let mut whole = numer / denom;
        let mut rem = numer % denom;
        let mut places = Vec::new();
        for _ in 0..8 {
            if rem == 0 {
                break;
            }
            rem *= 10;
            places.push((rem / denom) as u8);
            rem %= denom;
        }
        if rem != 0 {
            let next = (rem * 10) / denom;
            if next >= 5 {
                round_up(&mut places, &mut whole);
            }
        }
        let mut body = whole.to_string();
        if !places.is_empty() {
            body.push('.');
            for digit in places {
                body.push(char::from(b'0' + digit));
            }
        }
        if neg && body != "0" {
            format!("-{body}")
        } else {
            body
        }
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        let numer = self
            .numer()
            .checked_mul(other.denom())?
            .checked_add(other.numer().checked_mul(self.denom())?)?;
        let denom = self.denom().checked_mul(other.denom())?;
        Self::new(numer, denom)
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        let numer = self
            .numer()
            .checked_mul(other.denom())?
            .checked_sub(other.numer().checked_mul(self.denom())?)?;
        let denom = self.denom().checked_mul(other.denom())?;
        Self::new(numer, denom)
    }

    pub fn checked_mul(self, other: Self) -> Option<Self> {
        let numer = self.numer().checked_mul(other.numer())?;
        let denom = self.denom().checked_mul(other.denom())?;
        Self::new(numer, denom)
    }

    /// `None` when `other` is zero or the result does not fit.
    pub fn checked_div(self, other: Self) -> Option<Self> {
        if other.numer() == 0 {
            return None;
        }
        let numer = self.numer().checked_mul(other.denom())?;
        let denom = self.denom().checked_mul(other.numer())?;
        Self::new(numer, denom)
    }

    pub fn checked_pow(self, exp: u32) -> Option<Self> {
        let mut acc = Self::from_i64(1);
        for _ in 0..exp {
            acc = acc.checked_mul(self)?;
        }
        Some(acc)
    }

    pub fn is_zero(self) -> bool {
        self.numer() == 0
    }

    pub fn cmp_value(self, other: Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Serialize for Fraction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}/{}", self.numer(), self.denom()))
    }
}

impl<'de> Deserialize<'de> for Fraction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        parse_stored(&text).ok_or_else(|| serde::de::Error::custom(format!("bad fraction: {text}")))
    }
}

fn parse_stored(text: &str) -> Option<Fraction> {
    match text.split_once('/') {
        Some((numer, denom)) => {
            let numer: i64 = numer.parse().ok()?;
            let denom: i64 = denom.parse().ok()?;
            Fraction::new(numer, denom)
        }
        None => text.parse().ok().map(Fraction::from_i64),
    }
}

fn round_up(places: &mut Vec<u8>, whole: &mut u128) {
    for digit in places.iter_mut().rev() {
        if *digit < 9 {
            *digit += 1;
            return;
        }
        *digit = 0;
    }
    *whole += 1;
    places.clear();
}

#[cfg(test)]
mod tests {
    use super::Fraction;

    #[test]
    fn decimals_are_exact_fractions() {
        let w = Fraction::parse_decimal("1.2").expect("1.2");
        assert_eq!(w.exact(), "6/5");
        let ma = Fraction::parse_decimal("92.8").expect("92.8");
        assert_eq!(ma.exact(), "464/5");
        let moment = Fraction::new(432, 5).expect("432/5");
        assert_eq!(moment.decimal(), "86.4");
        assert_eq!(Fraction::new(72, 5).expect("72/5").decimal(), "14.4");
        assert_eq!(Fraction::parse_decimal("24").expect("24").exact(), "24");
    }
}
