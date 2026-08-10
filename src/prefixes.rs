//! SI (base-10) and binary (base-2) unit prefixes, and the shared [`ScaleFactor`]
//! trait used to compute their numeric scale.

use std::str::FromStr;

/// Error returned when a parsed string does not match any known prefix name.
#[derive(Debug)]
pub struct ParsePrefixError;

/// SI prefixes defined with base-10.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SIPrefix {
    /// Value of 10^30
    Quetta,
    /// Value of 10^27
    Ronna,
    /// Value of 10^24
    Yotta,
    /// Value of 10^21
    Zetta,
    /// Value of 10^18
    Exa,
    /// Value of 10^15
    Peta,
    /// Value of 10^12
    Tera,
    /// Value of 10^9
    Giga,
    /// Value of 10^6
    Mega,
    /// Value of 10^3
    Kilo,
    /// Value of 100
    Hecto,
    /// Value of 10
    Deca,
    /// Value of 0.1
    Deci,
    /// Value of 0.01
    Centi,
    /// Value of 10^-3
    Milli,
    /// Value of 10^-6
    Micro,
    /// Value of 10^-9
    Nano,
    /// Value of 10^-12
    Pico,
    /// Value of 10^-15
    Femto,
    /// Value of 10^-18
    Atto,
    /// Value of 10^-21
    Zepto,
    /// Value of 10^-27
    Ronto,
    /// Value of 10^-30
    Quecto,
}

/// Binary prefixes defined with base 2 (IEC 60027-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryPrefix {
    /// Value of 2^10 = 1024
    Kibi,
    /// Value of 2^20 = 1024^2
    Mebi,
    /// Value of 2^30 = 1024^3
    Gibi,
    /// Value of 2^40 = 1024^4
    Tebi,
    /// Value of 2^50 = 1024^5
    Pebi,
    /// Value of 2^60 = 1024^6
    Exbi,
    /// Value of 2^70 = 1024^7
    Zebi,
    /// Value of 2^80 = 1024^8
    Yobi,
}

/// Parses a prefix name (case-insensitive), e.g. `"kilo"` or `"KILO"` → [`SIPrefix::Kilo`].
///
/// # Errors
/// Returns [`ParsePrefixError`] if `s` does not match any known SI prefix.
impl FromStr for SIPrefix {
    type Err = ParsePrefixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "quetta" => Ok(SIPrefix::Quetta),
            "ronna" => Ok(SIPrefix::Ronna),
            "yotta" => Ok(SIPrefix::Yotta),
            "zetta" => Ok(SIPrefix::Zetta),
            "exa" => Ok(SIPrefix::Exa),
            "peta" => Ok(SIPrefix::Peta),
            "tera" => Ok(SIPrefix::Tera),
            "giga" => Ok(SIPrefix::Giga),
            "mega" => Ok(SIPrefix::Mega),
            "kilo" => Ok(SIPrefix::Kilo),
            "hecto" => Ok(SIPrefix::Hecto),
            "deca" => Ok(SIPrefix::Deca),
            "deci" => Ok(SIPrefix::Deci),
            "centi" => Ok(SIPrefix::Centi),
            "milli" => Ok(SIPrefix::Milli),
            "micro" => Ok(SIPrefix::Micro),
            "nano" => Ok(SIPrefix::Nano),
            "pico" => Ok(SIPrefix::Pico),
            "femto" => Ok(SIPrefix::Femto),
            "atto" => Ok(SIPrefix::Atto),
            "zepto" => Ok(SIPrefix::Zepto),
            "ronto" => Ok(SIPrefix::Ronto),
            "quecto" => Ok(SIPrefix::Quecto),
            _ => Err(ParsePrefixError),
        }
    }
}

/// Parses a prefix name (case-insensitive), e.g. `"kibi"` or `"KIBI"` -> [`BinaryPrefix::Kibi`].
///
/// # Errors
/// Returns [`ParsePrefixError`] if `s` does not match any known binary prefix.
impl FromStr for BinaryPrefix {
    type Err = ParsePrefixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kibi" => Ok(BinaryPrefix::Kibi),
            "mebi" => Ok(BinaryPrefix::Mebi),
            "gibi" => Ok(BinaryPrefix::Gibi),
            "tebi" => Ok(BinaryPrefix::Tebi),
            "pebi" => Ok(BinaryPrefix::Pebi),
            "exbi" => Ok(BinaryPrefix::Exbi),
            "zebi" => Ok(BinaryPrefix::Zebi),
            "yobi" => Ok(BinaryPrefix::Yobi),
            _ => Err(ParsePrefixError),
        }
    }
}

/// Trait allowing a prefix to be converted into a numeric scale factor.
///
/// Implemented by both [`SIPrefix`] and [`BinaryPrefix`] to allow both enums to be
/// handled generically.
pub trait ScaleFactor {
    /// Returns the multiplicative scale as an `f64`.
    ///
    /// E.G.: `Kilo.factor()` -> `1000.0` and `Kibi.factor()` -> `1024.0`
    fn factor(&self) -> f64;
}

impl ScaleFactor for SIPrefix {
    fn factor(&self) -> f64 {
        match self {
            SIPrefix::Quetta => 1_000_000_000_000_000_000_000_000_000_000.0,
            SIPrefix::Ronna => 1_000_000_000_000_000_000_000_000_000.0,
            SIPrefix::Yotta => 1_000_000_000_000_000_000_000_000.0,
            SIPrefix::Zetta => 1_000_000_000_000_000_000_000.0,
            SIPrefix::Exa => 1_000_000_000_000_000_000.0,
            SIPrefix::Peta => 1_000_000_000_000_000.0,
            SIPrefix::Tera => 1_000_000_000_000.0,
            SIPrefix::Giga => 1_000_000_000.0,
            SIPrefix::Mega => 1_000_000.0,
            SIPrefix::Kilo => 1_000.0,
            SIPrefix::Hecto => 100.0,
            SIPrefix::Deca => 10.0,
            SIPrefix::Deci => 0.1,
            SIPrefix::Centi => 0.01,
            SIPrefix::Milli => 0.001,
            SIPrefix::Micro => 0.000_001,
            SIPrefix::Nano => 0.000_000_001,
            SIPrefix::Pico => 0.000_000_000_001,
            SIPrefix::Femto => 0.000_000_000_000_001,
            SIPrefix::Atto => 0.000_000_000_000_000_001,
            SIPrefix::Zepto => 0.000_000_000_000_000_000_001,
            SIPrefix::Ronto => 0.000_000_000_000_000_000_000_001,
            SIPrefix::Quecto => 0.000_000_000_000_000_000_000_000_001,
        }
    }
}

impl ScaleFactor for BinaryPrefix {
    fn factor(&self) -> f64 {
        match self {
            BinaryPrefix::Kibi => 1024.0,
            BinaryPrefix::Mebi => 1048576.0,
            BinaryPrefix::Gibi => 1073741824.0,
            BinaryPrefix::Tebi => 1099511627776.0,
            BinaryPrefix::Pebi => 1125899906842624.0,
            BinaryPrefix::Exbi => 1152921504606846976.0,
            BinaryPrefix::Zebi => 1180591620717411303424.0,
            BinaryPrefix::Yobi => 1208925819614629174706176.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn base_10_scale_factor() {
        assert_relative_eq!(SIPrefix::Quetta.factor(), 10.0_f64.powi(30));
        assert_relative_eq!(SIPrefix::Ronna.factor(), 10.0_f64.powi(27));
        assert_relative_eq!(SIPrefix::Yotta.factor(), 10.0_f64.powi(24));
        assert_relative_eq!(SIPrefix::Zetta.factor(), 10.0_f64.powi(21));
        assert_relative_eq!(SIPrefix::Exa.factor(), 10.0_f64.powi(18));
        assert_relative_eq!(SIPrefix::Peta.factor(), 10.0_f64.powi(15));
        assert_relative_eq!(SIPrefix::Tera.factor(), 10.0_f64.powi(12));
        assert_relative_eq!(SIPrefix::Giga.factor(), 10.0_f64.powi(9));
        assert_relative_eq!(SIPrefix::Mega.factor(), 10.0_f64.powi(6));
        assert_relative_eq!(SIPrefix::Kilo.factor(), 10.0_f64.powi(3));
        assert_relative_eq!(SIPrefix::Hecto.factor(), 10.0_f64.powi(2));
        assert_relative_eq!(SIPrefix::Deca.factor(), 10.0_f64.powi(1));
        assert_relative_eq!(SIPrefix::Deci.factor(), 10.0_f64.powi(-1));
        assert_relative_eq!(SIPrefix::Centi.factor(), 10.0_f64.powi(-2));
        assert_relative_eq!(SIPrefix::Milli.factor(), 10.0_f64.powi(-3));
        assert_relative_eq!(SIPrefix::Micro.factor(), 10.0_f64.powi(-6));
        assert_relative_eq!(SIPrefix::Nano.factor(), 10.0_f64.powi(-9));
        assert_relative_eq!(SIPrefix::Pico.factor(), 10.0_f64.powi(-12));
        assert_relative_eq!(SIPrefix::Femto.factor(), 10.0_f64.powi(-15));
        assert_relative_eq!(SIPrefix::Atto.factor(), 10.0_f64.powi(-18));
        assert_relative_eq!(SIPrefix::Zepto.factor(), 10.0_f64.powi(-21));
        assert_relative_eq!(SIPrefix::Ronto.factor(), 10.0_f64.powi(-27));
        assert_relative_eq!(SIPrefix::Quecto.factor(), 10.0_f64.powi(-30));
    }

    #[test]
    fn base_2_scale_factor() {
        assert_relative_eq!(BinaryPrefix::Kibi.factor(), 2.0_f64.powi(10));
        assert_relative_eq!(BinaryPrefix::Mebi.factor(), 2.0_f64.powi(20));
        assert_relative_eq!(BinaryPrefix::Gibi.factor(), 2.0_f64.powi(30));
        assert_relative_eq!(BinaryPrefix::Tebi.factor(), 2.0_f64.powi(40));
        assert_relative_eq!(BinaryPrefix::Pebi.factor(), 2.0_f64.powi(50));
        assert_relative_eq!(BinaryPrefix::Exbi.factor(), 2.0_f64.powi(60));
        assert_relative_eq!(BinaryPrefix::Zebi.factor(), 2.0_f64.powi(70));
        assert_relative_eq!(BinaryPrefix::Yobi.factor(), 2.0_f64.powi(80));
    }

    #[test]
    fn prefix_str_parsing() {
        let prefix: SIPrefix = "KILO".parse().unwrap();
        assert_eq!(prefix, SIPrefix::Kilo);
        let prefix: BinaryPrefix = "kibi".parse().unwrap();
        assert_eq!(prefix, BinaryPrefix::Kibi);
    }
}
